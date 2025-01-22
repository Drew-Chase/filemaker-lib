use serde_json::Value;
use std::collections::HashMap;
use filemaker_lib::Filemaker;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize the Filemaker instance
    let filemaker = Filemaker::new("username", "password", "database_name", "table_name").await?;

    // Create the field data for a single record
    let mut single_record_data = HashMap::new();
    single_record_data.insert("field_name1".to_string(), Value::String("Value 1".to_string()));
    single_record_data.insert("field_name2".to_string(), Value::String("Value 2".to_string()));

    // Add the single record
    let result = filemaker.add_record(single_record_data).await?;

    // Print the result
    println!("Single record added: {:?}", result);

    Ok(())
}