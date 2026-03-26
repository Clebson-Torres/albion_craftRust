use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PriceDto {
    pub item_id: String,
    pub city: String,
    pub quality: u8,
    pub sell_price_min: u64,
    pub sell_price_min_date: String,
    pub sell_price_max: u64,
    pub sell_price_max_date: String,
    pub buy_price_min: u64,
    pub buy_price_min_date: String,
    pub buy_price_max: u64,
    pub buy_price_max_date: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HistoryBucketDto {
    pub item_count: u64,
    pub avg_price: u64,
    pub timestamp: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HistoryDto {
    pub location: String,
    pub item_id: String,
    pub quality: u8,
    pub data: Vec<HistoryBucketDto>,
}
