// SPDX-License-Identifier: Apache-2.0

use std::error::Error;

use tflers::Client;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();
    let client = Client::new();
    let arrivals = client.arrivals("490004105N").await?;
    println!("Arrivals for 490004105N: {arrivals:#?}");
    Ok(())
}
