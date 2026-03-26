use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDateTime, Utc};

use crate::api::dto::{HistoryBucketDto, HistoryDto, PriceDto};
use crate::domain::market::{HistoryBucket, MarketHistory, MarketSnapshot, MarketVenue};

pub fn map_price_dto(dto: &PriceDto) -> Result<MarketSnapshot> {
    Ok(MarketSnapshot {
        item_id: dto.item_id.clone(),
        city: MarketVenue::new(dto.city.clone()),
        sell_price_min: dto.sell_price_min,
        sell_price_max: dto.sell_price_max,
        buy_price_max: dto.buy_price_max,
        quality: dto.quality,
        observed_at: parse_api_timestamp(&dto.sell_price_min_date)?,
    })
}

pub fn map_history_dto(dto: &HistoryDto) -> Result<MarketHistory> {
    let mut data = Vec::with_capacity(dto.data.len());
    for bucket in &dto.data {
        data.push(map_history_bucket(bucket)?);
    }

    Ok(MarketHistory {
        item_id: dto.item_id.clone(),
        location: MarketVenue::new(dto.location.clone()),
        quality: dto.quality,
        data,
    })
}

fn map_history_bucket(dto: &HistoryBucketDto) -> Result<HistoryBucket> {
    Ok(HistoryBucket {
        item_count: dto.item_count,
        avg_price: dto.avg_price,
        timestamp: parse_api_timestamp(&dto.timestamp)?,
    })
}

fn parse_api_timestamp(input: &str) -> Result<DateTime<Utc>> {
    let parsed = NaiveDateTime::parse_from_str(input, "%Y-%m-%dT%H:%M:%S")
        .with_context(|| format!("invalid API timestamp: {input}"))?;
    Ok(DateTime::from_naive_utc_and_offset(parsed, Utc))
}
