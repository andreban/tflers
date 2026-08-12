use std::{borrow::Cow, error::Error};

use crate::types::StopPoint;

pub mod types;

#[derive(Clone, Debug, Default)]
pub struct Client {
    reqwest: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        let reqwest = reqwest::Client::new();
        Self { reqwest }
    }

    async fn get_json<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T, Box<dyn Error>> {
        let request = self.reqwest.get(url).build()?;
        Ok(self
            .reqwest
            .execute(request)
            .await?
            .error_for_status()?
            .json::<T>()
            .await?)
    }

    pub async fn stop_point<T: Into<Cow<'static, str>>>(
        &self,
        id: T,
    ) -> Result<StopPoint, Box<dyn Error>> {
        let id = id.into();
        let url = format!("https://api.tfl.gov.uk/StopPoint/{id}");
        self.get_json(&url).await
    }

    pub async fn stop_point_by_sms<T: Into<Cow<'static, str>>>(
        &self,
        code: T,
    ) -> Result<StopPoint, Box<dyn Error>> {
        let code = code.into();
        let url = format!("https://api.tfl.gov.uk/StopPoint/Sms/{code}");
        self.get_json(&url).await
    }
}
