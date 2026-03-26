use std::sync::{Arc, Mutex};

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::api::client::AlbionApiClient;
use crate::api::mapper::{map_history_dto, map_price_dto};
use crate::config::{server_base_url, AlbionServer};
use crate::domain::market::{MarketHistory, MarketSnapshot};
use crate::domain::opportunity::Opportunity;
use crate::services::catalog::{supported_item_ids, supported_material_ids};
use crate::services::crafting::SourcingStrategy;
use crate::services::ranking::{rank_opportunities, PremiumStatus, RankingPreferences};

#[derive(Debug, Clone)]
pub struct RefreshPayload {
    pub item_ids: Vec<String>,
    pub snapshots: Vec<MarketSnapshot>,
    pub histories: Vec<MarketHistory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshResult {
    pub items: Vec<Opportunity>,
    pub stale: bool,
}

#[async_trait]
pub trait MarketDataSource: Send + Sync {
    async fn load_market_data(&self) -> Result<RefreshPayload>;

    fn set_server(&self, _server: AlbionServer) {}
}

pub struct RefreshService {
    source: Arc<dyn MarketDataSource>,
    last_valid: Mutex<Option<Vec<Opportunity>>>,
    current_city: Mutex<String>,
    server: Mutex<AlbionServer>,
    sourcing_strategy: Mutex<SourcingStrategy>,
    transport_cost_per_unit: Mutex<u64>,
    premium_status: Mutex<PremiumStatus>,
    budget: Mutex<u64>,
}

impl RefreshService {
    pub fn new(
        source: Arc<dyn MarketDataSource>,
        server: AlbionServer,
        current_city: String,
        sourcing_strategy: SourcingStrategy,
        transport_cost_per_unit: u64,
        premium_status: PremiumStatus,
        budget: u64,
    ) -> Self {
        source.set_server(server);
        Self {
            source,
            last_valid: Mutex::new(None),
            current_city: Mutex::new(current_city),
            server: Mutex::new(server),
            sourcing_strategy: Mutex::new(sourcing_strategy),
            transport_cost_per_unit: Mutex::new(transport_cost_per_unit),
            premium_status: Mutex::new(premium_status),
            budget: Mutex::new(budget),
        }
    }

    pub fn update_preferences(
        &self,
        server: AlbionServer,
        current_city: String,
        sourcing_strategy: SourcingStrategy,
        transport_cost_per_unit: u64,
        premium_status: PremiumStatus,
        budget: u64,
    ) {
        self.source.set_server(server);
        *self.server.lock().unwrap() = server;
        *self.current_city.lock().unwrap() = current_city;
        *self.sourcing_strategy.lock().unwrap() = sourcing_strategy;
        *self.transport_cost_per_unit.lock().unwrap() = transport_cost_per_unit;
        *self.premium_status.lock().unwrap() = premium_status;
        *self.budget.lock().unwrap() = budget;
    }

    pub async fn refresh_top(&self, limit: usize) -> Result<RefreshResult> {
        match self.source.load_market_data().await {
            Ok(payload) => {
                let current_city = self.current_city.lock().unwrap().clone();
                let sourcing_strategy = *self.sourcing_strategy.lock().unwrap();
                let transport_cost_per_unit = *self.transport_cost_per_unit.lock().unwrap();
                let premium_status = *self.premium_status.lock().unwrap();
                let budget = *self.budget.lock().unwrap();
                let mut items = rank_opportunities(
                    &payload.item_ids,
                    &payload.snapshots,
                    &payload.histories,
                    RankingPreferences {
                        current_city: &current_city,
                        sourcing_strategy,
                        transport_cost_per_unit,
                        premium_status,
                        budget,
                    },
                );
                items.truncate(limit);
                *self.last_valid.lock().unwrap() = Some(items.clone());
                Ok(RefreshResult {
                    items,
                    stale: false,
                })
            }
            Err(err) => {
                if let Some(items) = self.last_valid.lock().unwrap().clone() {
                    return Ok(RefreshResult { items, stale: true });
                }
                Err(err)
            }
        }
    }
}

#[derive(Debug)]
pub struct AlbionApiSource {
    server: Mutex<AlbionServer>,
    locations: Vec<String>,
    qualities: Vec<u8>,
}

impl AlbionApiSource {
    pub fn new(server: AlbionServer, locations: Vec<String>, qualities: Vec<u8>) -> Self {
        Self {
            server: Mutex::new(server),
            locations,
            qualities,
        }
    }

    pub fn set_server(&self, server: AlbionServer) {
        *self.server.lock().unwrap() = server;
    }
}

#[async_trait]
impl MarketDataSource for AlbionApiSource {
    async fn load_market_data(&self) -> Result<RefreshPayload> {
        let server = *self.server.lock().unwrap();
        let client = AlbionApiClient::new(server_base_url(server.as_str()).to_owned());
        let item_ids = supported_item_ids();
        let mut price_item_ids = item_ids.clone();
        price_item_ids.extend(supported_material_ids());

        let prices = client
            .fetch_prices(&price_item_ids, &self.locations, &self.qualities)
            .await?;
        let history = client
            .fetch_history(&item_ids, &self.locations, &self.qualities, 24)
            .await?;

        let mut snapshots = Vec::with_capacity(prices.len());
        for dto in &prices {
            if let Ok(snapshot) = map_price_dto(dto) {
                snapshots.push(snapshot);
            }
        }

        let mut histories = Vec::with_capacity(history.len());
        for dto in &history {
            if let Ok(mapped) = map_history_dto(dto) {
                histories.push(mapped);
            }
        }

        Ok(RefreshPayload {
            item_ids,
            snapshots,
            histories,
        })
    }

    fn set_server(&self, server: AlbionServer) {
        AlbionApiSource::set_server(self, server);
    }
}
