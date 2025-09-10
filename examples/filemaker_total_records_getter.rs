use filemaker_lib::Filemaker;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    Filemaker::set_fm_url("https://fm.example.com/fmi/data/vLatest")?; // Replace with actual filemaker server url
    let username = "your_username";
    let password = "your_password";
    let database = "your_database";
    let table = "your_table";
    let filemaker = Filemaker::new(username, password, database, table).await?;

    // Get the total number of records
    let total_records = filemaker.get_number_of_records().await?;
    println!("Total Records: {}", total_records);

    Ok(())
}