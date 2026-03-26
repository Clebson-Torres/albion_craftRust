use std::sync::{Arc, Mutex};

use albion_crafting_overlay::config::AlbionServer;
use albion_crafting_overlay::domain::market::{
    HistoryBucket, MarketHistory, MarketSnapshot, MarketVenue,
};
use albion_crafting_overlay::services::crafting::SourcingStrategy;
use albion_crafting_overlay::services::ranking::PremiumStatus;
use albion_crafting_overlay::services::refresh::{
    MarketDataSource, RefreshPayload, RefreshService,
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use chrono::{TimeZone, Utc};

#[tokio::test]
async fn refresh_service_returns_ranked_top_items() {
    let source = StubSource::success();
    let service = RefreshService::new(
        Arc::new(source),
        AlbionServer::West,
        "Caerleon".to_owned(),
        SourcingStrategy::CheapestPerIngredient,
        0,
        PremiumStatus::Premium,
        150_000,
    );

    let result = service.refresh_top(10).await.expect("refresh");

    assert!(!result.items.is_empty());
    assert_eq!(result.items[0].item_id, "T4_BAG");
    assert!(!result.stale);
}

#[tokio::test]
async fn refresh_reuses_last_valid_result_when_api_fails() {
    let source = StubSource::success_then_failure();
    let service = RefreshService::new(
        Arc::new(source),
        AlbionServer::West,
        "Caerleon".to_owned(),
        SourcingStrategy::CheapestPerIngredient,
        0,
        PremiumStatus::Premium,
        150_000,
    );

    let first = service.refresh_top(10).await.expect("first refresh");
    let second = service.refresh_top(10).await.expect("second refresh");

    assert_eq!(first.items[0].item_id, second.items[0].item_id);
    assert!(second.stale);
}

#[tokio::test]
async fn changing_server_updates_data_source_host() {
    let source = TrackingSource::new();
    let service = RefreshService::new(
        Arc::new(source.clone()),
        AlbionServer::West,
        "Caerleon".to_owned(),
        SourcingStrategy::CheapestPerIngredient,
        0,
        PremiumStatus::Premium,
        150_000,
    );

    service.update_preferences(
        AlbionServer::Europe,
        "Caerleon".to_owned(),
        SourcingStrategy::CheapestPerIngredient,
        0,
        PremiumStatus::Premium,
        150_000,
    );
    let _ = service.refresh_top(10).await.expect("refresh");

    assert_eq!(source.last_server(), Some(AlbionServer::Europe));
}

struct StubSource {
    responses: Mutex<Vec<Result<RefreshPayload>>>,
}

#[derive(Clone)]
struct TrackingSource {
    last_server: Arc<Mutex<Option<AlbionServer>>>,
}

impl TrackingSource {
    fn new() -> Self {
        Self {
            last_server: Arc::new(Mutex::new(None)),
        }
    }

    fn last_server(&self) -> Option<AlbionServer> {
        *self.last_server.lock().unwrap()
    }
}

impl StubSource {
    fn success() -> Self {
        Self {
            responses: Mutex::new(vec![Ok(payload())]),
        }
    }

    fn success_then_failure() -> Self {
        Self {
            responses: Mutex::new(vec![Err(anyhow!("boom")), Ok(payload())]),
        }
    }
}

#[async_trait]
impl MarketDataSource for StubSource {
    async fn load_market_data(&self) -> Result<RefreshPayload> {
        self.responses
            .lock()
            .unwrap()
            .pop()
            .unwrap_or_else(|| Err(anyhow!("missing stub response")))
    }
}

#[async_trait]
impl MarketDataSource for TrackingSource {
    async fn load_market_data(&self) -> Result<RefreshPayload> {
        Ok(payload())
    }

    fn set_server(&self, server: AlbionServer) {
        *self.last_server.lock().unwrap() = Some(server);
    }
}

fn payload() -> RefreshPayload {
    let now = Utc.with_ymd_and_hms(2026, 3, 26, 12, 0, 0).unwrap();
    RefreshPayload {
        item_ids: vec!["T4_BAG".to_owned(), "T4_CAPE".to_owned()],
        snapshots: vec![
            price("T4_BAG", "Caerleon", 5200, now),
            price("T4_BAG", "Black Market", 6100, now),
            price("T4_CAPE", "Caerleon", 3000, now),
            price("T4_CAPE", "Black Market", 3300, now),
            price("T4_LEATHER", "Martlock", 70, now),
            price("T4_CLOTH", "Lymhurst", 120, now),
        ],
        histories: vec![
            history("T4_BAG", "Black Market", now, 200),
            history("T4_CAPE", "Black Market", now, 20),
        ],
    }
}

fn price(
    item_id: &str,
    city: &str,
    sell_price_min: u64,
    observed_at: chrono::DateTime<Utc>,
) -> MarketSnapshot {
    MarketSnapshot {
        item_id: item_id.to_owned(),
        city: MarketVenue::new(city.to_owned()),
        sell_price_min,
        sell_price_max: sell_price_min,
        buy_price_max: 0,
        quality: 1,
        observed_at,
    }
}

fn history(
    item_id: &str,
    location: &str,
    timestamp: chrono::DateTime<Utc>,
    item_count: u64,
) -> MarketHistory {
    MarketHistory {
        item_id: item_id.to_owned(),
        location: MarketVenue::new(location.to_owned()),
        quality: 1,
        data: vec![HistoryBucket {
            item_count,
            avg_price: 5000,
            timestamp,
        }],
    }
}
