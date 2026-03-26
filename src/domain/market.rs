use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketVenue(String);

impl MarketVenue {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketSnapshot {
    pub item_id: String,
    pub city: MarketVenue,
    pub sell_price_min: u64,
    pub sell_price_max: u64,
    pub buy_price_max: u64,
    pub quality: u8,
    pub observed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryBucket {
    pub item_count: u64,
    pub avg_price: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketHistory {
    pub item_id: String,
    pub location: MarketVenue,
    pub quality: u8,
    pub data: Vec<HistoryBucket>,
}
