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

    // Record ID to delete
    let record_id = 123; // Replace with the actual record ID

    // Delete the record
    let result = filemaker.delete_record(record_id).await?;
    println!("Record deleted successfully: {:?}", result);

    Ok(())
}