//!
//! # Filemaker Library (filemaker-lib)
//! 
//! This project is a Rust library (`filemaker-lib`) designed to interact with the FileMaker Data API. It provides a simple API to perform key operations against FileMaker databases, such as fetching records, performing searches, updating records, deleting records, and manipulating database sessions.
//! 
//! ## Features
//! 
//! - **[Database Interaction](#fetching-databases)**: Fetch the list of available databases.
//! - **[Authentication](#initialization)**: Securely manage session tokens for interacting with the FileMaker Data API.
//! - **[Record Management](#record-management)**:
//!   - [Fetch records](#fetching-records) (paginated or all at once).
//!   - [Search for records](#searching-records) based on custom queries and sorting criteria.
//!   - [Add single or multiple records](#adding-records).
//!   - [Update specific records](#updating-records).
//!   - [Delete records](#deleting-records) from a database.
//! - **[Database Layouts](#fetching-available-layouts)**: Retrieve the layouts available in a given database.
//! - **[Database Clearing](#clearing-the-database)**: Delete all records within a database.
//! - **[Advanced Querying and Sorting](#searching-records)**: Search with advanced queries and sort results in ascending or descending order.
//! - **Utility Functions**: Extract field names from records and encode database parameters safely.
//! 
//! ## Installation
//! 
//! Add `filemaker-lib` to your project's `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! filemaker-lib = "0.1.0"
//! ```
//! or using git
//! 
//! ```toml
//! [dependencies]
//! filemaker-lib = {git = "https://github.com/Drew-Chase/filemaker-lib.git"}
//! ```
//! 
//! ## Usage
//! 
//! ### Initialization
//! 
//! To create a `Filemaker` instance, you need to pass valid credentials (username and password), the name of the database, and the table you want to work with:
//! 
//! ```rust
//! use filemaker_lib::Filemaker;
//! use anyhow::Result;
//! 
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!   std::env::set_var("FM_URL", "https://fm.example.com/fmi/data/vLatest"); // Replace with actual FileMaker server URL
//!   let username = "your_username";
//!   let password = "your_password";
//!   let database = "your_database";
//!   let table = "your_table";
//! 
//!   let filemaker = Filemaker::new(username, password, database, table).await?;
//!   println!("Filemaker instance created successfully.");
//!   Ok(())
//! }
//! ```
//! 
//! ### Fetching Records
//! 
//! Retrieve specific records with pagination:
//! 
//! ```rust
//! let records = filemaker.get_records(1, 10).await?;
//! println!("Fetched Records: {:?}", records);
//! ```
//! 
//! Fetch all records at once:
//! 
//! ```rust
//! let all_records = filemaker.get_all_records().await?;
//! println!("All Records: {:?}", all_records);
//! ```
//! 
//! ### Adding Records
//! 
//! #### Adding a Single Record
//! 
//! To add a single record to your FileMaker database:
//! 
//! ```rust
//! use serde_json::Value;
//! use std::collections::HashMap;
//! 
//! let mut single_record_data = HashMap::new();
//! single_record_data.insert("field_name1".to_string(), Value::String("Value 1".to_string()));
//! single_record_data.insert("field_name2".to_string(), Value::String("Value 2".to_string()));
//! 
//! let result = filemaker.add_record(single_record_data).await?;
//! println!("Single record added: {:?}", result);
//! ```
//! 
//! #### Adding Multiple Records
//! 
//! To add multiple records to your FileMaker database:
//! 
//! ```rust
//! use serde_json::Value;
//! use std::collections::HashMap;
//! 
//! let records = vec![
//!   {
//!     let mut record = HashMap::new();
//!     record.insert("field_name1".to_string(), Value::String("First Record - Value 1".to_string()));
//!     record.insert("field_name2".to_string(), Value::String("First Record - Value 2".to_string()));
//!     record
//!   },
//!   {
//!     let mut record = HashMap::new();
//!     record.insert("field_name1".to_string(), Value::String("Second Record - Value 1".to_string()));
//!     record.insert("field_name2".to_string(), Value::String("Second Record - Value 2".to_string()));
//!     record
//!   },
//! ];
//! 
//! for (i, record_data) in records.into_iter().enumerate() {
//!   match filemaker.add_record(record_data).await {
//!   Ok(result) => println!("Record {} added successfully: {:?}", i + 1, result),
//!   Err(e) => eprintln!("Failed to add record {}: {}", i + 1, e),
//!   }
//! }
//! ```
//! 
//! ### Counting Records
//! 
//! Count the total number of records available in the table:
//! 
//! ```rust
//! let total_records = filemaker.get_number_of_records().await?;
//! println!("Total Records: {}", total_records);
//! ```
//! 
//! ### Searching Records
//! 
//! Perform a query with search parameters and sorting:
//! 
//! ```rust
//! use std::collections::HashMap;
//! 
//! let mut query = HashMap::new();
//! query.insert("fieldName".to_string(), "example_value".to_string());
//! 
//! let sort_fields = vec!["fieldName".to_string()];
//! let ascending = true;
//! 
//! let search_results = filemaker.search(vec![query], sort_fields, ascending).await?;
//! println!("Search Results: {:?}", search_results);
//! ```
//! 
//! ### Updating Records
//! 
//! Update a record by its ID:
//! 
//! ```rust
//! use serde_json::Value;
//! 
//! let record_id = 123;
//! let mut field_data = HashMap::new();
//! field_data.insert("fieldName".to_string(), Value::String("new_value".to_string()));
//! 
//! let update_result = filemaker.update_record(record_id, field_data).await?;
//! println!("Update Result: {:?}", update_result);
//! ```
//! 
//! ### Deleting Records
//! 
//! Delete a record by its ID:
//! 
//! ```rust
//! let record_id = 123;
//! filemaker.delete_record(record_id).await?;
//! println!("Record deleted successfully.");
//! ```
//! 
//! ### Fetching Available Layouts
//! 
//! Retrieve a list of layouts in the specified database:
//! 
//! ```rust
//! let layouts = Filemaker::get_layouts("your_username", "your_password", "your_database").await?;
//! println!("Available Layouts: {:?}", layouts);
//! ```
//! 
//! ### Fetching Databases
//! 
//! Retrieve the list of databases accessible with your credentials:
//! 
//! ```rust
//! let databases = Filemaker::get_databases("your_username", "your_password").await?;
//! println!("Databases: {:?}", databases);
//! ```
//! 
//! ### Clearing the Database
//! 
//! Delete all records from the current database and table:
//! 
//! ```rust
//! filemaker.clear_database().await?;
//! println!("All records cleared successfully.");
//! ```
//! 
//! ## Environment Variables
//! 
//! The library uses the `FM_URL` environment variable to specify the base URL of the FileMaker server. You need to set this variable before using the library:
//! 
//! ```rust
//! std::env::set_var("FM_URL", "https://fm.example.com/fmi/data/vLatest");
//! ```
//! 
//! Replace `"https://fm.example.com/fmi/data/vLatest"` with the actual URL of your FileMaker server.
//! 
//! ## Examples
//! 
//! This library comes with example implementations usable as references:
//! 
//! 1. **Fetch List of Databases**: [`get_databases`](examples/get_databases.rs)
//! 2. **Fetch Layouts from a Database**: [`filemaker_layout_fetcher`](examples/filemaker_layout_fetcher.rs)
//! 3. **Retrieve Records**: [`main_filemaker`](examples/main_filemaker.rs)
//! 4. **Add Single Record**: [`filemaker_single_record_adder`](examples/filemaker_single_record_adder.rs)
//! 5. **Add Multiple Records**: [`filemaker_multiple_record_adder`](examples/filemaker_multiple_record_adder.rs)
//! 6. **Update Database Records**: [`filemaker_record_updater`](examples/filemaker_record_updater.rs)
//! 7. **Delete Database Records**: [`filemaker_record_deleter`](examples/filemaker_record_deleter.rs)
//! 8. **Find Records Based on Query**: [`filemaker_search_results_output`](examples/filemaker_search_results_output.rs)
//! 
//! ## Logging
//! 
//! The library uses the [`log`](https://docs.rs/log/) crate for logging. To capture and display logs, set up a logging framework such as [`env_logger`](https://docs.rs/env_logger/). Example:
//! 
//! ```rust
//! use env_logger;
//! 
//! fn main() {
//!   env_logger::init();
//! }
//! ```
//! 
//! ## License
//! 
//! This library is licensed under the terms of the license detailed in the [`LICENSE`](LICENSE) file.
//! 
//! ---
//! 
//! For more information, please refer to the [repository documentation](https://github.com/Drew-Chase/filemaker-lib). Contributions are welcome!


use anyhow::Result;
use base64::Engine;
use log::*;
use reqwest::{Client, Method};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Represents a connection to a Filemaker database with authentication and query capabilities.
///
/// This struct manages the connection details and authentication token needed
/// to interact with a Filemaker database through its Data API.
#[derive(Clone)]
pub struct Filemaker {
    // Name of the database to connect to
    database: String,
    // Authentication token stored in thread-safe container that can be updated
    // Option is used since the token might not be available initially
    token: Arc<Mutex<Option<String>>>,
    // Name of the table/layout to operate on
    table: String,
    // HTTP client for making API requests
    client: Client,
}
impl Filemaker {
    /// Creates a new `Filemaker` instance.
    ///
    /// Initializes a connection to a FileMaker database with the provided credentials.
    /// This function performs authentication and sets up the HTTP client with appropriate configuration.
    ///
    /// # Arguments
    /// * `username` - The username for FileMaker authentication
    /// * `password` - The password for FileMaker authentication
    /// * `database` - The name of the FileMaker database to connect to
    /// * `table` - The name of the table/layout to operate on
    ///
    /// # Returns
    /// * `Result<Self>` - A new Filemaker instance or an error
    pub async fn new(username: &str, password: &str, database: &str, table: &str) -> Result<Self> {
        // URL-encode database and table names to handle spaces and special characters
        let encoded_database = Self::encode_parameter(database);
        let encoded_table = Self::encode_parameter(table);

        // Create an HTTP client that accepts invalid SSL certificates (for development)
        let client = Client::builder()
            .danger_accept_invalid_certs(true) // Disable SSL verification
            .build()
            .map_err(|e| {
                error!("Failed to build client: {}", e);
                anyhow::anyhow!(e)
            })?;

        // Authenticate with FileMaker and obtain a session token
        let token = Self::get_session_token(&client, database, username, password).await?;
        info!("Filemaker instance created successfully");

        // Return the initialized Filemaker instance
        Ok(Self {
            database: encoded_database,
            table: encoded_table,
            token: Arc::new(Mutex::new(Some(token))), // Wrap token in thread-safe container
            client,
        })
    }

    /// Gets a session token from the FileMaker Data API.
    ///
    /// Performs authentication against the FileMaker Data API and retrieves a session token
    /// that can be used for subsequent API requests.
    ///
    /// # Arguments
    /// * `client` - The HTTP client to use for the request
    /// * `database` - The name of the FileMaker database to authenticate against
    /// * `username` - The username for FileMaker authentication
    /// * `password` - The password for FileMaker authentication
    ///
    /// # Returns
    /// * `Result<String>` - The session token or an error
    async fn get_session_token(
        client: &Client,
        database: &str,
        username: &str,
        password: &str,
    ) -> Result<String> {
        // URL-encode the database name to handle spaces and special characters
        let database = Self::encode_parameter(database);

        // Construct the URL for the sessions endpoint
        let url = format!(
            "{}/databases/{}/sessions",
            std::env::var("FM_URL").unwrap_or_default().as_str(),
            database
        );

        // Create a Base64-encoded Basic authentication header
        let auth_header = format!(
            "Basic {}",
            base64::engine::general_purpose::STANDARD.encode(format!("{}:{}", username, password))
        );

        debug!("Requesting session token from URL: {}", url);

        // Send the authentication request to FileMaker
        let response = client
            .post(&url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .body("{}") // Empty JSON body for session creation
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send request for session token: {}", e);
                anyhow::anyhow!(e)
            })?;

        // Parse the JSON response
        let json: Value = response.json().await.map_err(|e| {
            error!("Failed to parse session token response: {}", e);
            anyhow::anyhow!(e)
        })?;

        // Extract the token from the response JSON structure
        if let Some(token) = json
            .get("response")
            .and_then(|r| r.get("token"))
            .and_then(|t| t.as_str())
        {
            info!("Session token retrieved successfully");
            Ok(token.to_string())
        } else {
            error!(
                "Failed to get token from FileMaker API response: {:?}",
                json
            );
            Err(anyhow::anyhow!("Failed to get token from FileMaker API"))
        }
    }

    /// Sends an authenticated HTTP request to the FileMaker Data API.
    ///
    /// This method handles adding the authentication token to requests and processing
    /// the response from the FileMaker Data API.
    ///
    /// # Arguments
    /// * `url` - The endpoint URL to send the request to
    /// * `method` - The HTTP method to use (GET, POST, etc.)
    /// * `body` - Optional JSON body to include with the request
    ///
    /// # Returns
    /// * `Result<Value>` - The parsed JSON response or an error
    async fn authenticated_request(
        &self,
        url: &str,
        method: Method,
        body: Option<Value>,
    ) -> Result<Value> {
        // Retrieve the session token from the shared state
        let token = self.token.lock().await.clone();
        if token.is_none() {
            error!("No session token found");
            return Err(anyhow::anyhow!("No session token found"));
        }

        // Create Bearer authentication header with the token
        let auth_header = format!("Bearer {}", token.unwrap());

        // Start building the request with appropriate headers
        let mut request = self
            .client
            .request(method, url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json");

        // Add the JSON body to the request if provided
        if let Some(body_content) = body {
            let json_body = serde_json::to_string(&body_content).map_err(|e| {
                error!("Failed to serialize request body: {}", e);
                anyhow::anyhow!(e)
            })?;
            debug!("Request body: {}", json_body);
            request = request.body(json_body);
        }

        debug!("Sending authenticated request to URL: {}", url);

        // Send the request and handle any network errors
        let response = request.send().await.map_err(|e| {
            error!("Failed to send authenticated request: {}", e);
            anyhow::anyhow!(e)
        })?;

        // Parse the response JSON and handle parsing errors
        let json: Value = response.json().await.map_err(|e| {
            error!("Failed to parse authenticated request response: {}", e);
            anyhow::anyhow!(e)
        })?;

        info!("Authenticated request to {} completed successfully", url);
        Ok(json)
    }

    /// Retrieves a specified range of records from the database.
    ///
    /// # Arguments
    /// * `start` - The starting position (offset) for record retrieval
    /// * `limit` - The maximum number of records to retrieve
    ///
    /// # Returns
    /// * `Result<Vec<Value>>` - A vector of record objects on success, or an error
    pub async fn get_records<T>(&self, start: T, limit: T) -> Result<Vec<Value>>
    where
        T: Sized + Clone + std::fmt::Display + std::str::FromStr + TryFrom<usize>,
    {
        // Construct the URL for the FileMaker Data API records endpoint
        let url = format!(
            "{}/databases/{}/layouts/{}/records?_offset={}&_limit={}",
            std::env::var("FM_URL").unwrap_or_default().as_str(),
            self.database,
            self.table,
            start,
            limit
        );
        debug!("Fetching records from URL: {}", url);

        // Send authenticated request to the API endpoint
        let response = self.authenticated_request(&url, Method::GET, None).await?;

        // Extract the records data from the response if available
        if let Some(data) = response.get("response").and_then(|r| r.get("data")) {
            info!("Successfully retrieved records from database");
            Ok(data.as_array().unwrap_or(&vec![]).clone())
        } else {
            // Log and return error if the expected data structure is not found
            error!("Failed to retrieve records from response: {:?}", response);
            Err(anyhow::anyhow!("Failed to retrieve records"))
        }
    }

    /// Retrieves all records from the database in a single query.
    ///
    /// This method first determines the total record count and then
    /// fetches all records in a single request.
    ///
    /// # Returns
    /// * `Result<Vec<Value>>` - A vector containing all records on success, or an error
    pub async fn get_all_records(&self) -> Result<Vec<Value>> {
        // First get the total number of records in the database
        let total_count = self.get_number_of_records().await?;
        debug!("Total records to fetch: {}", total_count);

        // Retrieve all records in a single request
        self.get_records(1, total_count).await
    }

    /// Retrieves the total number of records in the database table.
    ///
    /// # Returns
    /// * `Result<u64>` - The total record count on success, or an error
    pub async fn get_number_of_records(&self) -> Result<u64> {
        // Construct the URL for the FileMaker Data API records endpoint
        let url = format!(
            "{}/databases/{}/layouts/{}/records",
            std::env::var("FM_URL").unwrap_or_default().as_str(),
            self.database,
            self.table
        );
        debug!("Fetching total number of records from URL: {}", url);

        // Send authenticated request to the API endpoint
        let response = self.authenticated_request(&url, Method::GET, None).await?;

        // Extract the total record count from the response if available
        if let Some(total_count) = response
            .get("response")
            .and_then(|r| r.get("dataInfo"))
            .and_then(|d| d.get("totalRecordCount"))
            .and_then(|c| c.as_u64())
        {
            info!("Total record count retrieved successfully: {}", total_count);
            Ok(total_count)
        } else {
            // Log and return error if the expected data structure is not found
            error!(
                "Failed to retrieve total record count from response: {:?}",
                response
            );
            Err(anyhow::anyhow!("Failed to retrieve total record count"))
        }
    }

    /// Searches the database for records matching specified criteria.
    ///
    /// # Arguments
    /// * `query` - Vector of field-value pairs to search for
    /// * `sort` - Vector of field names to sort by
    /// * `ascending` - Whether to sort in ascending (true) or descending (false) order
    ///
    /// # Returns
    /// * `Result<Vec<Value>>` - A vector of matching records on success, or an error
    pub async fn search(
        &self,
        query: Vec<HashMap<String, String>>,
        sort: Vec<String>,
        ascending: bool,
    ) -> Result<Vec<Value>> {
        // Construct the URL for the FileMaker Data API find endpoint
        let url = format!(
            "{}/databases/{}/layouts/{}/_find",
            std::env::var("FM_URL").unwrap_or_default().as_str(),
            self.database,
            self.table
        );

        // Determine sort order based on ascending parameter
        let sort_order = if ascending { "ascend" } else { "descend" };

        // Transform the sort fields into the format expected by FileMaker API
        let sort_map: Vec<_> = sort
            .into_iter()
            .map(|s| {
                let mut map = HashMap::new();
                map.insert("fieldName".to_string(), s);
                map.insert("sortOrder".to_string(), sort_order.to_string());
                map
            })
            .collect();

        // Construct the request body with query and sort parameters
        let body: HashMap<String, Value> = HashMap::from([
            ("query".to_string(), serde_json::to_value(query)?),
            ("sort".to_string(), serde_json::to_value(sort_map)?),
        ]);
        debug!("Executing search query with URL: {}. Body: {:?}", url, body);

        // Send authenticated POST request to the API endpoint
        let response = self
            .authenticated_request(&url, Method::POST, Some(serde_json::to_value(body)?))
            .await?;

        // Extract the search results from the response if available
        if let Some(data) = response.get("response").and_then(|r| r.get("data")) {
            info!("Search query executed successfully");
            Ok(data.as_array().unwrap_or(&vec![]).clone())
        } else {
            // Log and return error if the expected data structure is not found
            error!(
                "Failed to retrieve search results from response: {:?}",
                response
            );
            Err(anyhow::anyhow!("Failed to retrieve search results"))
        }
    }

    /// Adds a record to the database.
    ///
    /// # Parameters
    /// - `field_data`: A `HashMap` representing the field data for the new record.
    ///
    /// # Returns
    /// A `Result` containing the added record as a `Value` on success, or an error.
    pub async fn add_record(
        &self,
        field_data: HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>> {
        // Define the URL for the FileMaker Data API endpoint
        let url = format!(
            "{}/databases/{}/layouts/{}/records",
            std::env::var("FM_URL").unwrap_or_default().as_str(),
            self.database,
            self.table
        );

        // Prepare the request body
        let field_data_map: serde_json::Map<String, Value> = field_data.into_iter().collect();
        let body = HashMap::from([("fieldData".to_string(), Value::Object(field_data_map))]);

        debug!("Adding a new record. URL: {}. Body: {:?}", url, body);

        // Make the API call
        let response = self
            .authenticated_request(&url, Method::POST, Some(serde_json::to_value(body)?))
            .await?;

        if let Some(record_id) = response
            .get("response")
            .and_then(|r| r.get("recordId"))
            .and_then(|id| id.as_str())
        {
            if let Ok(record_id) = record_id.parse::<u64>() {
                debug!("Record added successfully. Record ID: {}", record_id);
                let added_record = self.get_record_by_id(record_id).await?;
                Ok(HashMap::from([
                    ("success".to_string(), Value::Bool(true)),
                    ("result".to_string(), added_record),
                ]))
            } else {
                error!("Failed to parse record id {} - {:?}", record_id, response);
                Ok(HashMap::from([
                    ("success".to_string(), Value::Bool(false)),
                    ("result".to_string(), response),
                ]))
            }
        } else {
            error!("Failed to add the record: {:?}", response);
            Ok(HashMap::from([
                ("success".to_string(), Value::Bool(false)),
                ("result".to_string(), response),
            ]))
        }
    }

    /// Updates a record in the database using the FileMaker Data API.
    ///
    /// # Arguments
    /// * `id` - The unique identifier of the record to update
    /// * `field_data` - A hashmap containing the field names and their new values
    ///
    /// # Returns
    /// * `Result<Value>` - The server response as a JSON value or an error
    ///
    /// # Type Parameters
    /// * `T` - A type that can be used as a record identifier and meets various trait requirements
    pub async fn update_record<T>(&self, id: T, field_data: HashMap<String, Value>) -> Result<Value>
    where
        T: Sized + Clone + std::fmt::Display + std::str::FromStr + TryFrom<usize>,
    {
        // Construct the API endpoint URL for updating a specific record
        let url = format!(
            "{}/databases/{}/layouts/{}/records/{}",
            std::env::var("FM_URL").unwrap_or_default().as_str(),
            self.database,
            self.table,
            id
        );

        // Convert the field data hashmap to the format expected by FileMaker Data API
        let field_data_map: serde_json::Map<String, Value> = field_data.into_iter().collect();
        // Create the request body with fieldData property
        let body = HashMap::from([("fieldData".to_string(), Value::Object(field_data_map))]);

        debug!("Updating record ID: {}. URL: {}. Body: {:?}", id, url, body);

        // Send the PATCH request to update the record
        let response = self
            .authenticated_request(&url, Method::PATCH, Some(serde_json::to_value(body)?))
            .await?;

        info!("Record ID: {} updated successfully", id);
        Ok(response)
    }

    /// Retrieves the list of databases accessible to the specified user.
    ///
    /// # Arguments
    /// * `username` - The FileMaker username for authentication
    /// * `password` - The FileMaker password for authentication
    ///
    /// # Returns
    /// * `Result<Vec<String>>` - A list of accessible database names or an error
    pub async fn get_databases(username: &str, password: &str) -> Result<Vec<String>> {
        // Construct the API endpoint URL for retrieving databases
        let url = format!(
            "{}/databases",
            std::env::var("FM_URL").unwrap_or_default().as_str()
        );

        // Create Base64 encoded Basic auth header from username and password
        let auth_header = format!(
            "Basic {}",
            base64::engine::general_purpose::STANDARD.encode(format!("{}:{}", username, password))
        );

        debug!("Fetching list of databases from URL: {}", url);

        // Initialize HTTP client
        let client = Client::new();

        // Send request to get list of databases with authentication
        let response = client
            .get(&url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send request for databases: {}", e);
                anyhow::anyhow!(e)
            })?
            .json::<Value>()
            .await
            .map_err(|e| {
                error!("Failed to parse database list response: {}", e);
                anyhow::anyhow!(e)
            })?;

        // Extract database names from the response JSON
        if let Some(databases) = response
            .get("response")
            .and_then(|r| r.get("databases"))
            .and_then(|d| d.as_array())
        {
            // Extract the name field from each database object
            let database_names = databases
                .iter()
                .filter_map(|db| {
                    db.get("name")
                        .and_then(|n| n.as_str())
                        .map(|s| s.to_string())
                })
                .collect();

            info!("Database list retrieved successfully");
            Ok(database_names)
        } else {
            // Handle case where response doesn't contain expected data structure
            error!("Failed to retrieve databases from response: {:?}", response);
            Err(anyhow::anyhow!("Failed to retrieve databases"))
        }
    }

    /// Retrieves the list of layouts for the specified database using the provided credentials.
    ///
    /// # Arguments
    /// * `username` - The FileMaker username for authentication
    /// * `password` - The FileMaker password for authentication
    /// * `database` - The name of the database to get layouts from
    ///
    /// # Returns
    /// * `Result<Vec<String>>` - A list of layout names or an error
    pub async fn get_layouts(
        username: &str,
        password: &str,
        database: &str,
    ) -> Result<Vec<String>> {
        // URL encode the database name and construct the API endpoint URL
        let encoded_database = Self::encode_parameter(database);
        let url = format!(
            "{}/databases/{}/layouts",
            std::env::var("FM_URL").unwrap_or_default().as_str(),
            encoded_database
        );

        debug!("Fetching layouts from URL: {}", url);

        // Create HTTP client and get session token for authentication
        let client = Client::new();
        let token = Self::get_session_token(&client, database, username, password)
            .await
            .map_err(|e| {
                error!("Failed to get session token for layouts: {}", e);
                anyhow::anyhow!(e)
            })?;

        // Create Bearer auth header from the session token
        let auth_header = format!("Bearer {}", token);

        // Send request to get list of layouts with token authentication
        let response = client
            .get(&url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send request to retrieve layouts: {}", e);
                anyhow::anyhow!(e)
            })?
            .json::<Value>()
            .await
            .map_err(|e| {
                error!("Failed to parse response for layouts: {}", e);
                anyhow::anyhow!(e)
            })?;

        // Extract layout names from the response JSON
        if let Some(layouts) = response
            .get("response")
            .and_then(|r| r.get("layouts"))
            .and_then(|l| l.as_array())
        {
            // Extract the name field from each layout object
            let layout_names = layouts
                .iter()
                .filter_map(|layout| {
                    layout
                        .get("name")
                        .and_then(|n| n.as_str())
                        .map(|s| s.to_string())
                })
                .collect();

            info!("Successfully retrieved layouts");
            Ok(layout_names)
        } else {
            // Handle case where response doesn't contain expected data structure
            error!("Failed to retrieve layouts from response: {:?}", response);
            Err(anyhow::anyhow!("Failed to retrieve layouts"))
        }
    }

    /// Gets a record from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the record to get.
    ///
    /// # Returns
    /// A JSON object representing the record.
    pub async fn get_record_by_id<T>(&self, id: T) -> Result<Value>
    where
        T: Sized + Clone + std::fmt::Display + std::str::FromStr + TryFrom<usize>,
    {
        let url = format!(
            "{}/databases/{}/layouts/{}/records/{}",
            std::env::var("FM_URL").unwrap_or_default().as_str(),
            self.database,
            self.table,
            id
        );

        debug!("Fetching record with ID: {} from URL: {}", id, url);

        let response = self
            .authenticated_request(&url, Method::GET, None)
            .await
            .map_err(|e| {
                error!("Failed to get record ID {}: {}", id, e);
                anyhow::anyhow!(e)
            })?;

        if let Some(data) = response.get("response").and_then(|r| r.get("data")) {
            if let Some(record) = data.as_array().and_then(|arr| arr.first()) {
                info!("Record ID {} retrieved successfully", id);
                Ok(record.clone())
            } else {
                error!("No record found for ID {}", id);
                Err(anyhow::anyhow!("No record found"))
            }
        } else {
            error!("Failed to get record from response: {:?}", response);
            Err(anyhow::anyhow!("Failed to get record"))
        }
    }

    /// Deletes a record from the database by its ID.
    ///
    /// # Arguments
    /// * `id` - The ID of the record to delete.
    ///
    /// # Returns
    /// A result indicating the deletion was successful or an error message.
    pub async fn delete_record<T>(&self, id: T) -> Result<Value>
    where
        T: Sized + Clone + std::fmt::Display + std::str::FromStr + TryFrom<usize>,
    {
        let url = format!(
            "{}/databases/{}/layouts/{}/records/{}",
            std::env::var("FM_URL").unwrap_or_default().as_str(),
            self.database,
            self.table,
            id
        );

        debug!("Deleting record with ID: {} at URL: {}", id, url);

        let response = self
            .authenticated_request(&url, Method::DELETE, None)
            .await
            .map_err(|e| {
                error!("Failed to delete record ID {}: {}", id, e);
                anyhow::anyhow!(e)
            })?;

        if response.is_object() {
            info!("Record ID {} deleted successfully", id);
            Ok(json!({"success": true}))
        } else {
            error!("Failed to delete record ID {}", id);
            Err(anyhow::anyhow!("Failed to delete record"))
        }
    }

    /// Deletes the specified database.
    ///
    /// # Arguments
    /// * `database` - The name of the database to delete.
    /// * `username` - The username for authentication.
    /// * `password` - The password for authentication.
    pub async fn delete_database(database: &str, username: &str, password: &str) -> Result<()> {
        let encoded_database = Self::encode_parameter(database);
        let url = format!(
            "{}/databases/{}",
            std::env::var("FM_URL").unwrap_or_default().as_str(),
            encoded_database
        );

        debug!("Deleting database: {}", database);

        let client = Client::new();
        let token = Self::get_session_token(&client, database, username, password)
            .await
            .map_err(|e| {
                error!("Failed to get session token for database deletion: {}", e);
                anyhow::anyhow!(e)
            })?;
        let auth_header = format!("Bearer {}", token);

        client
            .delete(&url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| {
                error!("Failed to delete database {}: {}", database, e);
                anyhow::anyhow!(e)
            })?;

        info!("Database {} deleted successfully", database);
        Ok(())
    }

    /// Deletes all records from the current database.
    ///
    /// This function retrieves and systematically removes all records from the database.
    /// It first checks if there are any records to delete, then proceeds with deletion
    /// if records exist.
    ///
    /// # Returns
    /// * `Result<()>` - Ok(()) if all records were successfully deleted, or an error
    ///
    /// # Errors
    /// * Returns error if unable to retrieve records
    /// * Returns error if record ID parsing fails
    /// * Returns error if record deletion fails
    pub async fn clear_database(&self) -> Result<()> {
        debug!("Clearing all records from the database");
        // Get the total count of records in the database
        let number_of_records = self.get_number_of_records().await?;

        // Check if there are any records to delete
        if number_of_records == 0 {
            warn!("No records found in the database. Nothing to clear");
            return Ok(());
        }

        // Retrieve all records that need to be deleted
        // The number_of_records value is used as limit to fetch all records at once
        let records = self.get_records(1, number_of_records).await.map_err(|e| {
            error!("Failed to retrieve records for clearing database: {}", e);
            anyhow::anyhow!(e)
        })?;

        // Iterate through each record and delete it individually
        for record in records {
            // Extract the record ID from the record data
            if let Some(id) = record.get("recordId").and_then(|id| id.as_str()) {
                // The record ID is usually marked as a string even though it's a u64,
                // so we need to parse it to the correct type
                if let Ok(id) = id.parse::<u64>() {
                    debug!("Deleting record ID: {}", id);
                    // Attempt to delete the record and handle any errors
                    if let Err(e) = self.delete_record(id).await {
                        error!("Failed to delete record ID {}: {}", id, e);
                        return Err(anyhow::anyhow!(e));
                    }
                } else {
                    // Handle case where ID exists but cannot be parsed as u64
                    error!("Failed to parse record ID {} as u64", id);
                    return Err(anyhow::anyhow!("Failed to parse record ID as u64"));
                }
            } else {
                // Handle case where record doesn't contain an ID field
                error!("Record ID not found in record: {:?}", record);
                return Err(anyhow::anyhow!(
                    "Record ID not found in record: {:?}",
                    record
                ));
            }
        }

        info!("All records cleared from the database");
        Ok(())
    }
    /// Returns the names of fields in the given record excluding the ones starting with 'g_' (global fields)
    ///
    /// # Arguments
    /// * `record` - An example record with 'fieldData' element containing field names as keys.
    ///
    /// # Returns
    /// An array of field names.
    pub fn get_row_names_by_example(record: &Value) -> Vec<String> {
        let mut fields = Vec::new();
        if let Some(field_data) = record.get("fieldData").and_then(|fd| fd.as_object()) {
            for field in field_data.keys() {
                if !field.starts_with("g_") {
                    fields.push(field.clone());
                }
            }
        }
        info!("Extracted row names: {:?}", fields);
        fields
    }

    /// Gets the field names for the first record in the database.
    ///
    /// This function retrieves a single record from the database and extracts
    /// field names from it. If no records exist, an empty vector is returned.
    ///
    /// # Returns
    /// * `Result<Vec<String>>` - A vector of field names on success, or an error
    pub async fn get_row_names(&self) -> Result<Vec<String>> {
        debug!("Attempting to fetch field names for the first record");

        // Fetch just the first record to use as a template
        let records = self.get_records(1, 1).await?;

        if let Some(first_record) = records.first() {
            info!("Successfully fetched field names for the first record");
            // Extract field names from the first record using the helper method
            return Ok(Self::get_row_names_by_example(first_record));
        }

        // Handle the case where no records exist in the database
        warn!("No records found while fetching field names");
        Ok(vec![])
    }

    /// Searches the database for records matching the specified query.
    ///
    /// # Arguments
    /// * `fields` - The query fields.
    /// * `sort` - The sort order.
    /// * `ascending` - Whether to sort in ascending order.
    ///
    /// # Returns
    /// A vector of matching records.
    pub async fn advanced_search(
        &self,
        fields: HashMap<String, Value>,
        sort: Vec<String>,
        ascending: bool,
    ) -> Result<Vec<Value>> {
        let url = format!(
            "{}/databases/{}/layouts/{}/_find",
            std::env::var("FM_URL").unwrap_or_default().as_str(),
            self.database,
            self.table
        );

        debug!(
            "Preparing advanced search with fields: {:?}, sort: {:?}, ascending: {}",
            fields, sort, ascending
        );

        let mut content = serde_json::Map::new();
        content.insert(
            "query".to_string(),
            Value::Array(fields.into_iter().map(|(k, v)| json!({ k: v })).collect()),
        );

        if !sort.is_empty() {
            let sort_array: Vec<Value> = sort
                .into_iter()
                .map(|s| {
                    json!({
                        "fieldName": s,
                        "sortOrder": if ascending { "ascend" } else { "descend" }
                    })
                })
                .collect();
            content.insert("sort".to_string(), Value::Array(sort_array));
        }

        debug!(
            "Sending authenticated request to URL: {} with content: {:?}",
            url, content
        );

        let response = self
            .authenticated_request(&url, Method::POST, Some(Value::Object(content)))
            .await?;

        if let Some(data) = response
            .get("response")
            .and_then(|r| r.get("data"))
            .and_then(|d| d.as_array())
        {
            info!(
                "Advanced search completed successfully, retrieved {} records",
                data.len()
            );
            Ok(data.clone())
        } else {
            error!("Failed to retrieve advanced search results: {:?}", response);
            Err(anyhow::anyhow!(
                "Failed to retrieve advanced search results"
            ))
        }
    }

    /// Encodes a parameter by replacing spaces with `%20`.
    ///
    /// This function takes a string parameter and replaces all spaces with URL-encoded
    /// representation (%20), which is useful for preparing strings to be included in URLs.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The string to be encoded
    ///
    /// # Returns
    ///
    /// A new String with all spaces replaced by %20
    fn encode_parameter(parameter: &str) -> String {
        // Replace all spaces with %20 URL encoding
        let encoded = parameter.replace(" ", "%20");

        // Log the encoding operation at debug level
        debug!("Encoded parameter '{}' to '{}'", parameter, encoded);

        // Return the encoded string
        encoded
    }
}
