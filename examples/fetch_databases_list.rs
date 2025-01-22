use filemaker_lib::Filemaker;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var("FM_URL", "https://fm.example.com/fmi/data/vLatest"); // Replace with actual filemaker server url
    let username = "your_username";
    let password = "your_password";

    // Fetch the list of databases
    let databases = Filemaker::get_databases(username, password).await?;
    println!("Databases:");
    for db in databases {
        println!("{}", db);
    }

    Ok(())
}