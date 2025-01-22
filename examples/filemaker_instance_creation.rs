use filemaker_lib::Filemaker;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let username = "your_username";
    let password = "your_password";
    let database = "your_database";
    let table = "your_table";

    // Create a Filemaker instance
    let filemaker = Filemaker::new(username, password, database, table).await?;
    println!("Filemaker instance created successfully.");

    Ok(())
}