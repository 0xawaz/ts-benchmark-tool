use structopt::StructOpt;
use tsbtool;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(help = "Path to the CSV file")]
    csv_file: String,
    
    #[structopt(short, long, default_value = "5", help = "Number of workers (default is 5)")]
    workers: usize,
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();

    // 1. read CSV and convert to SQL query
    let sql_queries = match tsbtool::read_csv(&args.csv_file) {
        Ok(queries) => queries,
        Err(err) => {
            eprintln!("Error reading CSV file: {}. Please provide a valid CSV file and a valid path.", err);
            std::process::exit(1);
        }
    };

    // 2. distribute queries among workers and display bench stats
    tsbtool::distribute_work(sql_queries).await;
}