use anyhow::Result;
use filemaker_lib::Filemaker;
use serde_json::Value;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    Filemaker::set_fm_url("https://fm.example.com/fmi/data/vLatest")?; // Replace with actual filemaker server url
    let username = "your_username";
    let password = "your_password";
    let database = "your_database";
    let table = "your_table";
    let filemaker = Filemaker::new(username, password, database, table).await?;

    // Construct search query
    let mut query = HashMap::new();
    query.insert("fieldName".to_string(), "example_value".to_string()); // Replace "fieldName" and "example_value"

    // Sorting criteria
    let sort_fields = vec!["fieldName".to_string()]; // Replace with the field name to sort
    let ascending = true;

    let records = filemaker
        .search::<Value>(vec![query], sort_fields, ascending, Some(10))
        .await?;
    println!("Search Results:");
    for record in records.response.data {
        println!("{:?}", record);
    }

    Ok(())
}
