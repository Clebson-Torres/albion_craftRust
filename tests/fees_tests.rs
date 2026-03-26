use albion_crafting_overlay::domain::market::{
    HistoryBucket, MarketHistory, MarketSnapshot, MarketVenue,
};
use albion_crafting_overlay::services::crafting::SourcingStrategy;
use albion_crafting_overlay::services::ranking::{
    rank_opportunities, PremiumStatus, RankingPreferences,
};
use chrono::{TimeZone, Utc};

#[test]
fn premium_changes_total_fee_and_net_profit() {
    let now = Utc.with_ymd_and_hms(2026, 3, 26, 12, 0, 0).unwrap();
    let snapshots = vec![
        price("T4_BAG", "Black Market", 6100, now),
        price("T4_CLOTH", "Martlock", 120, now),
        price("T4_LEATHER", "Martlock", 80, now),
    ];
    let histories = vec![history("T4_BAG", "Black Market", now, 200)];

    let premium = rank_opportunities(
        &["T4_BAG".to_owned()],
        &snapshots,
        &histories,
        RankingPreferences {
            current_city: "Martlock",
            sourcing_strategy: SourcingStrategy::CurrentCityOnly,
            transport_cost_per_unit: 0,
            premium_status: PremiumStatus::Premium,
            budget: 150_000,
        },
    );
    let non_premium = rank_opportunities(
        &["T4_BAG".to_owned()],
        &snapshots,
        &histories,
        RankingPreferences {
            current_city: "Martlock",
            sourcing_strategy: SourcingStrategy::CurrentCityOnly,
            transport_cost_per_unit: 0,
            premium_status: PremiumStatus::Standard,
            budget: 150_000,
        },
    );

    assert!(premium[0].total_fee_impact < non_premium[0].total_fee_impact);
    assert!(premium[0].net_profit > non_premium[0].net_profit);
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
