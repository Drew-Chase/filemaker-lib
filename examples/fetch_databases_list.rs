use filemaker_lib::Filemaker;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    Filemaker::set_fm_url("https://fm.mardens.com/fmi/data/vLatest")?; // Replace with actual filemaker server url
    let username = "admin";
    let password = "19MRCC77!";

    // Fetch the list of databases
    let databases = Filemaker::get_databases(username, password).await?;
    println!("Databases:");
    for db in databases {
        println!("{}", db);
    }

    Ok(())
}