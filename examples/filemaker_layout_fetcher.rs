use filemaker_lib::Filemaker;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    std::env::set_var("FM_URL", "https://fm.example.com/fmi/data/vLatest"); // Replace with actual filemaker server url
    let username = "your_username";
    let password = "your_password";
    let database = "your_database";

    // Fetch layouts
    let layouts = Filemaker::get_layouts(username, password, database).await?;
    println!("Available Layouts:");
    for layout in layouts {
        println!("{}", layout);
    }

    Ok(())
}