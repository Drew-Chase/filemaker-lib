use anyhow::Result;
use base64::Engine;
use log::*;
use reqwest::{Client, Method};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Filemaker {
    database: String,
    token: Arc<Mutex<Option<String>>>,
    table: String,
    client: Client,
}
impl Filemaker {
    /// Creates a new `Filemaker` instance.
    pub async fn new(username: &str, password: &str, database: &str, table: &str) -> Result<Self> {
        let encoded_database = Self::encode_parameter(database);
        let encoded_table = Self::encode_parameter(table);

        let client = Client::builder()
            .danger_accept_invalid_certs(true) // Disable SSL verification
            .build()
            .map_err(|e| {
                error!("Failed to build client: {}", e);
                anyhow::anyhow!(e)
            })?;

        let token = Self::get_session_token(&client, database, username, password).await?;

        info!("Filemaker instance created successfully");
        Ok(Self {
            database: encoded_database,
            table: encoded_table,
            token: Arc::new(Mutex::new(Some(token))),
            client,
        })
    }

    /// Gets a session token from the FileMaker Data API.
    async fn get_session_token(
        client: &Client,
        database: &str,
        username: &str,
        password: &str,
    ) -> Result<String> {
        let database = Self::encode_parameter(database);
        let url = format!(
            "{}/databases/{}/sessions",
            std::env::var("FM_URL").unwrap_or_default().as_str(),
            database
        );
        let auth_header = format!(
            "Basic {}",
            base64::engine::general_purpose::STANDARD.encode(format!("{}:{}", username, password))
        );

        debug!("Requesting session token from URL: {}", url);

        let response = client
            .post(&url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json")
            .body("{}")
            .send()
            .await
            .map_err(|e| {
                error!("Failed to send request for session token: {}", e);
                anyhow::anyhow!(e)
            })?;

        let json: Value = response.json().await.map_err(|e| {
            error!("Failed to parse session token response: {}", e);
            anyhow::anyhow!(e)
        })?;

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

    /// Sends an authenticated HTTP request.
    async fn authenticated_request(
        &self,
        url: &str,
        method: Method,
        body: Option<Value>,
    ) -> Result<Value> {
        let token = self.token.lock().await.clone();

        if token.is_none() {
            error!("No session token found");
            return Err(anyhow::anyhow!("No session token found"));
        }

        let auth_header = format!("Bearer {}", token.unwrap());

        let mut request = self
            .client
            .request(method, url)
            .header("Authorization", auth_header)
            .header("Content-Type", "application/json");

        if let Some(body_content) = body {
            let json_body = serde_json::to_string(&body_content).map_err(|e| {
                error!("Failed to serialize request body: {}", e);
                anyhow::anyhow!(e)
            })?;
            request = request.body(json_body);
        }

        debug!("Sending authenticated request to URL: {}", url);

        let response = request.send().await.map_err(|e| {
            error!("Failed to send authenticated request: {}", e);
            anyhow::anyhow!(e)
        })?;

        let json: Value = response.json().await.map_err(|e| {
            error!("Failed to parse authenticated request response: {}", e);
            anyhow::anyhow!(e)
        })?;

        info!("Authenticated request to {} completed successfully", url);
        Ok(json)
    }

    /// Retrieves records from the database.
    pub async fn get_records<T>(&self, start: T, limit: T) -> Result<Vec<Value>>
    where
        T: Sized + Clone + std::fmt::Display + std::str::FromStr + TryFrom<usize>,
    {
        let url = format!(
            "{}/databases/{}/layouts/{}/records?_offset={}&_limit={}",
            std::env::var("FM_URL").unwrap_or_default().as_str(),
            self.database,
            self.table,
            start,
            limit
        );

        debug!("Fetching records from URL: {}", url);

        let response = self.authenticated_request(&url, Method::GET, None).await?;
        if let Some(data) = response.get("response").and_then(|r| r.get("data")) {
            info!("Successfully retrieved records from database");
            Ok(data.as_array().unwrap_or(&vec![]).clone())
        } else {
            error!("Failed to retrieve records from response: {:?}", response);
            Err(anyhow::anyhow!("Failed to retrieve records"))
        }
    }

    /// Retrieves all records from the database.
    pub async fn get_all_records(&self) -> Result<Vec<Value>> {
        let total_count = self.get_number_of_records().await?;
        debug!("Total records to fetch: {}", total_count);

        self.get_records(1, total_count).await
    }

    /// Retrieves the total number of records in the database.
    pub async fn get_number_of_records(&self) -> Result<u64> {
        let url = format!(
            "{}/databases/{}/layouts/{}/records",
            std::env::var("FM_URL").unwrap_or_default().as_str(),
            self.database,
            self.table
        );

        debug!("Fetching total number of records from URL: {}", url);

        let response = self.authenticated_request(&url, Method::GET, None).await?;
        if let Some(total_count) = response
            .get("response")
            .and_then(|r| r.get("dataInfo"))
            .and_then(|d| d.get("totalRecordCount"))
            .and_then(|c| c.as_u64())
        {
            info!("Total record count retrieved successfully: {}", total_count);
            Ok(total_count)
        } else {
            error!(
                "Failed to retrieve total record count from response: {:?}",
                response
            );
            Err(anyhow::anyhow!("Failed to retrieve total record count"))
        }
    }

    /// Searches the database for records matching a query.
    pub async fn search(
        &self,
        query: Vec<HashMap<String, String>>,
        sort: Vec<String>,
        ascending: bool,
    ) -> Result<Vec<Value>> {
        let url = format!(
            "{}/databases/{}/layouts/{}/_find",
            std::env::var("FM_URL").unwrap_or_default().as_str(),
            self.database,
            self.table
        );

        let sort_order = if ascending { "ascend" } else { "descend" };
        let sort_map: Vec<_> = sort
            .into_iter()
            .map(|s| {
                let mut map = HashMap::new();
                map.insert("fieldName".to_string(), s);
                map.insert("sortOrder".to_string(), sort_order.to_string());
                map
            })
            .collect();

        let body: HashMap<String, Value> = HashMap::from([
            ("query".to_string(), serde_json::to_value(query)?),
            ("sort".to_string(), serde_json::to_value(sort_map)?),
        ]);

        debug!("Executing search query with URL: {}. Body: {:?}", url, body);

        let response = self
            .authenticated_request(&url, Method::POST, Some(serde_json::to_value(body)?))
            .await?;
        if let Some(data) = response.get("response").and_then(|r| r.get("data")) {
            info!("Search query executed successfully");
            Ok(data.as_array().unwrap_or(&vec![]).clone())
        } else {
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
            .and_then(|id| id.as_u64())
        {
            debug!("Record added successfully. Record ID: {}", record_id);
            let added_record = self.get_record_by_id(record_id).await?;
            Ok(HashMap::from([
                ("success".to_string(), Value::Bool(true)),
                ("result".to_string(), added_record),
            ]))
        } else {
            error!("Failed to add the record: {:?}", response);
            Ok(HashMap::from([
                ("success".to_string(), Value::Bool(false)),
                ("result".to_string(), response),
            ]))
        }
    }

    /// Updates a record in the database.
    pub async fn update_record<T>(&self, id: T, field_data: HashMap<String, Value>) -> Result<Value>
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
        let field_data_map: serde_json::Map<String, Value> = field_data.into_iter().collect();
        let body = HashMap::from([("fieldData".to_string(), Value::Object(field_data_map))]);

        debug!("Updating record ID: {}. URL: {}. Body: {:?}", id, url, body);

        let response = self
            .authenticated_request(&url, Method::PATCH, Some(serde_json::to_value(body)?))
            .await?;

        info!("Record ID: {} updated successfully", id);
        Ok(response)
    }

    /// Retrieves the list of databases accessible to the specified user.
    pub async fn get_databases(username: &str, password: &str) -> Result<Vec<String>> {
        let url = format!(
            "{}/databases",
            std::env::var("FM_URL").unwrap_or_default().as_str()
        );
        let auth_header = format!(
            "Basic {}",
            base64::engine::general_purpose::STANDARD.encode(format!("{}:{}", username, password))
        );

        debug!("Fetching list of databases from URL: {}", url);

        let client = Client::new();
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

        if let Some(databases) = response
            .get("response")
            .and_then(|r| r.get("databases"))
            .and_then(|d| d.as_array())
        {
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
            error!("Failed to retrieve databases from response: {:?}", response);
            Err(anyhow::anyhow!("Failed to retrieve databases"))
        }
    }

    /// Retrieves the list of layouts for the specified database and user credentials.
    pub async fn get_layouts(
        username: &str,
        password: &str,
        database: &str,
    ) -> Result<Vec<String>> {
        let encoded_database = Self::encode_parameter(database);
        let url = format!(
            "{}/databases/{}/layouts",
            std::env::var("FM_URL").unwrap_or_default().as_str(),
            encoded_database
        );

        debug!("Fetching layouts from URL: {}", url);

        // Get session token
        let client = Client::new();
        let token = Self::get_session_token(&client, database, username, password)
            .await
            .map_err(|e| {
                error!("Failed to get session token for layouts: {}", e);
                anyhow::anyhow!(e)
            })?;

        let auth_header = format!("Bearer {}", token);

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

        if let Some(layouts) = response
            .get("response")
            .and_then(|r| r.get("layouts"))
            .and_then(|l| l.as_array())
        {
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
    pub async fn clear_database(&self) -> Result<()> {
        debug!("Clearing all records from the database");
        let number_of_records = self.get_number_of_records().await?;
        let records = self.get_records(1, number_of_records).await.map_err(|e| {
            error!("Failed to retrieve records for clearing database: {}", e);
            anyhow::anyhow!(e)
        })?;

        for record in records {
            if let Some(id) = record.get("recordId").and_then(|id| id.as_u64()) {
                if let Err(e) = self.delete_record(id).await {
                    error!("Failed to delete record ID {}: {}", id, e);
                    return Err(anyhow::anyhow!(e));
                }
            } else {
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

    /// Gets the field names for the first record.
    pub async fn get_row_names(&self) -> Result<Vec<String>> {
        debug!("Attempting to fetch field names for the first record");
        let records = self.get_records(1, 1).await?;
        if let Some(first_record) = records.first() {
            info!("Successfully fetched field names for the first record");
            return Ok(Self::get_row_names_by_example(first_record));
        }
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
    fn encode_parameter(parameter: &str) -> String {
        let encoded = parameter.replace(" ", "%20");
        debug!("Encoded parameter '{}' to '{}'", parameter, encoded);
        encoded
    }
}
