use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// Path to the CSV file
    csv_file: String,
    /// Number of workers
    #[structopt(short, long, default_value = "5")]
    workers: usize,
}

fn main() {
    let args = Cli::from_args();

    if let Err(err) = tsbtool::read_csv(&args.csv_file) {
        eprintln!("Error reading CSV file: {}", err);
    }

    println!("Number of workers: {}", args.workers);
}