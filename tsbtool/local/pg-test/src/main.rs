use postgres::{Client, NoTls, Error};
use std::env;

fn main() -> Result<(), Error> {

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", database_url);

    // let mut client = Client::connect(database_url, NoTls)?;

    // client.batch_execute("
    //     CREATE TABLE pg-test (
    //         id      SERIAL PRIMARY KEY,
    //         name    TEXT NOT NULL,
    //         phone    INT NOT NULL
    //     )
    // ")?;
     
    Ok(())

}