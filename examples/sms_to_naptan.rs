use std::error::Error;

use tflers::Client;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();
    let client = Client::new();
    let stop_point = client.stop_point_by_sms("58015").await?;
    println!("{stop_point:#?}");
    Ok(())
}
