use anyhow::{Context, Result};

use crate::api::dto::{HistoryDto, PriceDto};

#[derive(Debug, Clone)]
pub struct AlbionApiClient {
    base_url: String,
    client: reqwest::Client,
}

impl AlbionApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_owned(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_prices(
        &self,
        item_ids: &[String],
        locations: &[String],
        qualities: &[u8],
    ) -> Result<Vec<PriceDto>> {
        let items = item_ids.join(",");
        let url = format!("{}/api/v2/stats/prices/{}.json", self.base_url, items);
        let response = self
            .client
            .get(url)
            .query(&[
                ("locations", locations.join(",")),
                (
                    "qualities",
                    qualities
                        .iter()
                        .map(u8::to_string)
                        .collect::<Vec<_>>()
                        .join(","),
                ),
            ])
            .send()
            .await
            .context("failed to fetch price rows")?
            .error_for_status()
            .context("price request returned error status")?;

        response
            .json::<Vec<PriceDto>>()
            .await
            .context("failed to deserialize price rows")
    }

    pub async fn fetch_history(
        &self,
        item_ids: &[String],
        locations: &[String],
        qualities: &[u8],
        time_scale: u8,
    ) -> Result<Vec<HistoryDto>> {
        let items = item_ids.join(",");
        let url = format!("{}/api/v2/stats/history/{}.json", self.base_url, items);
        let response = self
            .client
            .get(url)
            .query(&[
                ("locations", locations.join(",")),
                (
                    "qualities",
                    qualities
                        .iter()
                        .map(u8::to_string)
                        .collect::<Vec<_>>()
                        .join(","),
                ),
                ("time-scale", time_scale.to_string()),
            ])
            .send()
            .await
            .context("failed to fetch history rows")?
            .error_for_status()
            .context("history request returned error status")?;

        response
            .json::<Vec<HistoryDto>>()
            .await
            .context("failed to deserialize history rows")
    }
}
