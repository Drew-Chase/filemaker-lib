use filemaker_lib::Filemaker;
use anyhow::Result;

#[tokio::main]
async fn main()->Result<()> {
	Filemaker::set_fm_url("https://fm.example.com/fmi/data/vLatest")?; // Replace with actual filemaker server url
	let username = "username"; // Replace with actual username
	let password = "password"; // Replace with actual password
	let databases = Filemaker::get_databases(username, password).await?;
	for db in databases {
		println!("{}", db);
	}
	
	Ok(())
}