use filemaker_lib::Filemaker;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var("FM_URL", "https://fm.example.com/fmi/data/vLatest"); // Replace with actual filemaker server url
    let username = "your_username";
    let password = "your_password";
    let database = "your_database";
    let table = "your_table";
    let filemaker = Filemaker::new(username, password, database, table).await?;

    // Record ID to update
    let record_id = 123; // Replace with the actual record ID

    // Data to update
    let mut field_data = HashMap::new();
    field_data.insert("fieldName".to_string(), Value::String("new_value".to_string())); // Replace "fieldName" and "new_value"

    // Update the record
    let result = filemaker.update_record(record_id, field_data).await?;
    println!("Update Result: {:?}", result);

    Ok(())
}