use std::{borrow::Cow, error::Error};

use crate::types::StopPoint;

pub mod types;

pub struct Client {
    reqwest: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        let reqwest = reqwest::Client::new();
        Self { reqwest }
    }

    pub async fn stop_point<T: Into<Cow<'static, str>>>(
        &self,
        id: T,
    ) -> Result<StopPoint, Box<dyn Error>> {
        let id = id.into();
        let url = format!("https://api.tfl.gov.uk/StopPoint/{id}");
        let request = self.reqwest.get(url).build()?;
        Ok(self
            .reqwest
            .execute(request)
            .await?
            .error_for_status()?
            .json::<StopPoint>()
            .await?)
    }
}
