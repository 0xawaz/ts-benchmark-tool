use std::env;
use dotenv::dotenv;
use postgres::{Client, NoTls};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    println!("################# Hi 0xawaz, let's rust! #####################");

    // Set the TS_DATABASE_URL for testing
    // env::set_var("TS_DATABASE_URL", "postgres://postgres:xxxx@yyyy/homework");

    // test database connection
    let database_url = &env::var("TS_DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", database_url);

    let mut client = Client::connect(database_url, NoTls)?;

    // create table if not exists
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS oxawaz (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            phone    INT NOT NULL
        )
    ")?;

    // check if table was created
    let table_exists = check_table_exists(&mut client, "oxawaz")?;

    if table_exists {
        println!("The table 'oxawaz' exists.");
    } else {
        println!("The table 'oxawaz' does not exist.");
    }

    Ok(())
}

fn check_table_exists(client: &mut Client, table_name: &str) -> Result<bool, postgres::Error> {
    let query = "
        SELECT EXISTS (
            SELECT 1 
            FROM pg_catalog.pg_tables 
            WHERE tablename = $1
        )
    ";

    let rows = client.query(query, &[&table_name])?;
    Ok(rows.get(0).unwrap().get(0))
}
