use csv::Reader;

//
// ------------------------------ READ & PROCESS CSV FILE ------------------------------
//
pub fn read_csv(csv_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = Reader::from_path(csv_file)?;

    for result in rdr.records() {
        let record = result?;
        let hostname = &record[0];
        let start_time = &record[1];
        let end_time = &record[2];

        // Generate the SQL query for each record
        let sql_query = format!(
            "SELECT time_bucket_gapfill('1 minute', time) AS one_min, \
            MAX(cpu_usage) AS max_cpu, MIN(cpu_usage) AS min_cpu \
            FROM cpu_usage_data \
            WHERE hostname = '{}' AND \
            time >= '{}' AND time <= '{}' \
            GROUP BY one_min \
            ORDER BY one_min;",
            hostname, start_time, end_time
        );

        println!("{}", sql_query);
    }

    Ok(())
}