use filemaker_lib::Filemaker;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var("FM_URL", "https://fm.example.com/fmi/data/vLatest"); // Replace with actual filemaker server url
    let username = "your_username";
    let password = "your_password";
    let database = "your_database";
    let table = "your_table";
    let filemaker = Filemaker::new(username, password, database, table).await?;

    // Fetch the first 10 records
    let records = filemaker.get_records(1, 10).await?;
    println!("Fetched Records:");
    for record in records {
        println!("{:?}", record);
    }

    Ok(())
}