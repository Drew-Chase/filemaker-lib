# Filemaker Library (filemaker-lib)

This project is a Rust library (`filemaker-lib`) designed to interact with the FileMaker Data API. It provides a simple API to perform key operations against FileMaker databases, such as fetching records, performing searches, updating records, deleting records, and manipulating database sessions.

## Features

- **[Database Interaction](#fetching-databases)**: Fetch the list of available databases.
- **[Authentication](#initialization)**: Securely manage session tokens for interacting with the FileMaker Data API.
- **[Record Management](#record-management)**:
  - [Fetch records](#fetching-records) (paginated or all at once).
  - [Search for records](#searching-records) based on custom queries and sorting criteria.
  - [Add single or multiple records](#adding-records).
  - [Update specific records](#updating-records).
  - [Delete records](#deleting-records) from a database.
- **[Database Layouts](#fetching-available-layouts)**: Retrieve the layouts available in a given database.
- **[Database Clearing](#clearing-the-database)**: Delete all records within a database.
- **[Advanced Querying and Sorting](#searching-records)**: Search with advanced queries and sort results in ascending or descending order.
- **Utility Functions**: Extract field names from records and encode database parameters safely.

## Installation

Add `filemaker-lib` to your project's `Cargo.toml`:

```toml
[dependencies]
filemaker-lib = "0.2.0"
```
or using git

```toml
[dependencies]
filemaker-lib = {git = "https://github.com/Drew-Chase/filemaker-lib.git"}
```

## Usage

### Initialization

To create a `Filemaker` instance, you need to pass valid credentials (username and password), the name of the database, and the table you want to work with:

```rust
use filemaker_lib::Filemaker;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
  Filemaker::set_fm_url("https://fm.example.com/fmi/data/vLatest")?; // Replace with actual FileMaker server URL
  let username = "your_username";
  let password = "your_password";
  let database = "your_database";
  let table = "your_table";

  let filemaker = Filemaker::new(username, password, database, table).await?;
  println!("Filemaker instance created successfully.");
  Ok(())
}
```

## Setting Filemaker API URL (Version 0.1.x)

The library uses the `FM_URL` environment variable to specify the base URL of the FileMaker server. You need to set this variable before using the library:

```rust
std::env::set_var("FM_URL", "https://fm.example.com/fmi/data/vLatest");
```
Replace `"https://fm.example.com/fmi/data/vLatest"` with the actual URL of your FileMaker server.

## Setting Filemaker API URL (Version 0.2.x)
In version 0.2.x, the library uses a new method to set the URL:
```rust
Filemaker::set_fm_url("https://fm.example.com/fmi/data/vLatest")?;
```

### Fetching Records

Retrieve specific records with pagination:

```rust
let records = filemaker.get_records(1, 10).await?;
println!("Fetched Records: {:?}", records);
```

Fetch all records at once:

```rust
let all_records = filemaker.get_all_records().await?;
println!("All Records: {:?}", all_records);
```

### Adding Records

#### Adding a Single Record

To add a single record to your FileMaker database:

```rust
use serde_json::Value;
use std::collections::HashMap;

let mut single_record_data = HashMap::new();
single_record_data.insert("field_name1".to_string(), Value::String("Value 1".to_string()));
single_record_data.insert("field_name2".to_string(), Value::String("Value 2".to_string()));

let result = filemaker.add_record(single_record_data).await?;
println!("Single record added: {:?}", result);
```

#### Adding Multiple Records

To add multiple records to your FileMaker database:

```rust
use serde_json::Value;
use std::collections::HashMap;

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

for (i, record_data) in records.into_iter().enumerate() {
  match filemaker.add_record(record_data).await {
  Ok(result) => println!("Record {} added successfully: {:?}", i + 1, result),
  Err(e) => eprintln!("Failed to add record {}: {}", i + 1, e),
  }
}
```

### Counting Records

Count the total number of records available in the table:

```rust
let total_records = filemaker.get_number_of_records().await?;
println!("Total Records: {}", total_records);
```

### Searching Records

Perform a query with search parameters and sorting:

```rust
use std::collections::HashMap;

let mut query = HashMap::new();
query.insert("fieldName".to_string(), "example_value".to_string());

let sort_fields = vec!["fieldName".to_string()];
let ascending = true;

let search_results = filemaker.search::<serde_json::Value>(vec![query], sort_fields, ascending).await?;
println!("Search Results: {:?}", search_results);
```

### Updating Records

Update a record by its ID:

```rust
use serde_json::Value;

let record_id = 123;
let mut field_data = HashMap::new();
field_data.insert("fieldName".to_string(), Value::String("new_value".to_string()));

let update_result = filemaker.update_record(record_id, field_data).await?;
println!("Update Result: {:?}", update_result);
```

### Deleting Records

Delete a record by its ID:

```rust
let record_id = 123;
filemaker.delete_record(record_id).await?;
println!("Record deleted successfully.");
```

### Fetching Available Layouts

Retrieve a list of layouts in the specified database:

```rust
let layouts = Filemaker::get_layouts("your_username", "your_password", "your_database").await?;
println!("Available Layouts: {:?}", layouts);
```

### Fetching Databases

Retrieve the list of databases accessible with your credentials:

```rust
let databases = Filemaker::get_databases("your_username", "your_password").await?;
println!("Databases: {:?}", databases);
```

### Clearing the Database

Delete all records from the current database and table:

```rust
filemaker.clear_database().await?;
println!("All records cleared successfully.");
```

## Examples

This library comes with example implementations usable as references:

1. **Fetch List of Databases**: [`get_databases`](examples/get_databases.rs)
2. **Fetch Layouts from a Database**: [`filemaker_layout_fetcher`](examples/filemaker_layout_fetcher.rs)
3. **Retrieve Records**: [`main_filemaker`](examples/main_filemaker.rs)
4. **Add Single Record**: [`filemaker_single_record_adder`](examples/filemaker_single_record_adder.rs)
5. **Add Multiple Records**: [`filemaker_multiple_record_adder`](examples/filemaker_multiple_record_adder.rs)
6. **Update Database Records**: [`filemaker_record_updater`](examples/filemaker_record_updater.rs)
7. **Delete Database Records**: [`filemaker_record_deleter`](examples/filemaker_record_deleter.rs)
8. **Find Records Based on Query**: [`filemaker_search_results_output`](examples/filemaker_search_results_output.rs)

## Logging

The library uses the [`log`](https://docs.rs/log/) crate for logging. To capture and display logs, set up a logging framework such as [`env_logger`](https://docs.rs/env_logger/). Example:

```rust
use env_logger;

fn main() {
  env_logger::init();
}
```

## License

This library is licensed under the terms of the license detailed in the [`LICENSE`](LICENSE) file.

---

For more information, please refer to the [repository documentation](https://github.com/Drew-Chase/filemaker-lib). Contributions are welcome!