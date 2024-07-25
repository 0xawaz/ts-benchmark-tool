use csv::Reader;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::task;
use tokio::time::{Duration, Instant};
use futures::future::join_all;
use tokio_postgres::{NoTls};
use std::env;
use dotenv::dotenv;

//
// ------------------------------ READ & PROCESS CSV FILE ------------------------------
//
pub fn read_csv(csv_file: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!("---> Reading CSV file ...");
    let mut rdr = Reader::from_path(csv_file)?;
    let mut queries = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let hostname = &record[0];
        let start_time = &record[1];
        let end_time = &record[2];

        // Generate the SQL query for each record
        let sql_query = format!(
            "SELECT time_bucket_gapfill('1 minute', ts) AS one_min, \
            MAX(usage) AS max_cpu, MIN(usage) AS min_cpu \
            FROM cpu_usage \
            WHERE host = '{}' AND \
            ts >= '{}' AND ts <= '{}' \
            GROUP BY one_min \
            ORDER BY one_min;",
            hostname, start_time, end_time
        );

        // Collect the generated SQL query
        queries.push(sql_query);
    }

    Ok(queries)
}

//
// ------------------------------ PROCESS SQL QUERY ASYNC ------------------------------
// 
async fn process_query(query: String) -> Duration {
    dotenv().ok();  // Load environment variables from .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Start the timer
    let start = Instant::now();

    // Establish the connection
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .expect("Failed to connect to the database");

    // Spawn the connection task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Execute the query
    match client
        .query(query.as_str(), &[])
        .await
    {
        Ok(_rows) => {
            // println!("Query executed successfully, {} rows returned.", rows.len());
        }
        Err(e) => {
            eprintln!("Query execution error: {}", e);
        }
    }

    // Calculate and return the elapsed time
    start.elapsed()
}

//
// ------------------------------ DISTRIBUTE WORK AMONG WORKERS ------------------------------
//
pub async fn distribute_work(queries: Vec<String>, num_workers: usize) {
    let mut workers: Vec<Arc<Mutex<Vec<String>>>> = Vec::new();
    for _ in 0..num_workers {
        workers.push(Arc::new(Mutex::new(Vec::new())));
    }

    let mut hostname_to_worker = HashMap::new();
    let mut tasks = Vec::new();

    for query in queries {
        let hostname = extract_hostname(&query);
        let worker = hostname_to_worker.entry(hostname.to_string()).or_insert_with(|| {
            workers.iter().min_by_key(|w| w.lock().unwrap().len()).unwrap().clone()
        });

        worker.lock().unwrap().push(query);
    }

    println!("---> Start distributing queries among workers ...");
    for worker in workers {
        let worker_clone = worker.clone();
        let task = task::spawn(async move {
            let mut durations = Vec::new();
            let queries = worker_clone.lock().unwrap().clone();
            for query in queries {
                let duration = process_query(query).await;
                durations.push(duration);
            }
            durations
        });
        tasks.push(task);
    }

    let results = join_all(tasks).await;
    let mut all_durations = Vec::new();
    for result in results {
        match result {
            Ok(durations) => all_durations.extend(durations),
            Err(e) => eprintln!("Worker task failed: {:?}", e),
        }
    }

    // Calculate statistics
    println!("---> Calculate benchmark statistics ...");
    if all_durations.is_empty() {
        eprintln!("No queries were processed.");
        return;
    }

    all_durations.sort();
    let num_queries = all_durations.len();
    let total_time: Duration = all_durations.iter().sum();
    let min_time = *all_durations.first().unwrap();
    let max_time = *all_durations.last().unwrap();
    let median_time = if num_queries % 2 == 0 {
        (all_durations[num_queries / 2 - 1] + all_durations[num_queries / 2]) / 2
    } else {
        all_durations[num_queries / 2]
    };
    let avg_time = total_time / (num_queries as u32);

    println!("\n---------------------------- Benchmark Stats ----------------------------");
    println!("Number of queries run: {}", num_queries);
    println!("Total processing time: {:?}", total_time);
    println!("Minimum query time: {:?}", min_time);
    println!("Median query time: {:?}", median_time);
    println!("Average query time: {:?}", avg_time);
    println!("Maximum query time: {:?}", max_time);
}

fn extract_hostname(query: &str) -> &str {
    let start = query.find("WHERE host = '").unwrap() + "WHERE host = '".len();
    let end = query[start..].find('\'').unwrap() + start;
    &query[start..end]
}
