use filemaker_lib::Filemaker;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    Filemaker::set_fm_url("https://example.com/fmi/data/vLatest")?; // Replace with actual filemaker server url
    let username = "username";
    let password = "password";

    // Fetch the list of databases
    let databases = Filemaker::get_databases(username, password).await?;
    println!("Databases:");
    for db in databases {
        println!("{}", db);
    }

    Ok(())
}
