use serde_json::Value;
use std::collections::HashMap;
use filemaker_lib::Filemaker;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize the Filemaker instance
    let filemaker = Filemaker::new("username", "password", "database_name", "table_name").await?;

    // Prepare data for multiple records
    let records = vec![
        {
            let mut record = HashMap::new();
            record.insert("field_name1".to_string(), Value::String("First Record - Value 1".to_string()));
            record.insert("field_name2".to_string(), Value::String("First Record - Value 2".to_string()));
            record
        },
        {
            let mut record = HashMap::new();
            record.insert("field_name1".to_string(), Value::String("Second Record - Value 1".to_string()));
            record.insert("field_name2".to_string(), Value::String("Second Record - Value 2".to_string()));
            record
        },
    ];

    // Add each record to the database
    for (i, record_data) in records.into_iter().enumerate() {
        match filemaker.add_record(record_data).await {
            Ok(result) => println!("Record {} added successfully: {:?}", i + 1, result),
            Err(e) => eprintln!("Failed to add record {}: {}", i + 1, e),
        }
    }

    Ok(())
}