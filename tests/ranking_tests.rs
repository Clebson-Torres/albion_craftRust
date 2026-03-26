use albion_crafting_overlay::domain::market::{
    HistoryBucket, MarketHistory, MarketSnapshot, MarketVenue,
};
use albion_crafting_overlay::services::crafting::SourcingStrategy;
use albion_crafting_overlay::services::ranking::{
    rank_opportunities, PremiumStatus, RankingPreferences,
};
use chrono::{Duration, TimeZone, Utc};

#[test]
fn rank_opportunities_prefers_higher_net_profit() {
    let now = Utc.with_ymd_and_hms(2026, 3, 26, 12, 0, 0).unwrap();
    let snapshots = vec![
        price("T4_BAG", "Caerleon", 5200, now),
        price("T4_BAG", "Black Market", 6100, now),
        price("T4_CAPE", "Caerleon", 3000, now),
        price("T4_CAPE", "Black Market", 3300, now),
        price("T4_LEATHER", "Martlock", 70, now),
        price("T4_CLOTH", "Lymhurst", 120, now),
    ];
    let histories = vec![
        history("T4_BAG", "Black Market", now, 200),
        history("T4_CAPE", "Black Market", now, 20),
    ];

    let ranked = rank_opportunities(
        &["T4_BAG".to_owned(), "T4_CAPE".to_owned()],
        &snapshots,
        &histories,
        RankingPreferences {
            current_city: "Caerleon",
            sourcing_strategy: SourcingStrategy::CheapestPerIngredient,
            transport_cost_per_unit: 0,
            premium_status: PremiumStatus::Premium,
            budget: 150_000,
        },
    );

    assert_eq!(ranked[0].item_id, "T4_BAG");
    assert!(ranked[0].net_profit > ranked[1].net_profit);
}

#[test]
fn stale_or_incomplete_data_reduces_confidence() {
    let fresh = Utc.with_ymd_and_hms(2026, 3, 26, 12, 0, 0).unwrap();
    let stale = fresh - Duration::hours(30);
    let snapshots = vec![
        price("T4_BAG", "Black Market", 6100, fresh),
        price("T4_BAG", "Caerleon", 5200, fresh),
        price("T4_CAPE", "Black Market", 6100, stale),
        price("T4_CAPE", "Caerleon", 5200, stale),
        price("T4_LEATHER", "Martlock", 70, fresh),
        price("T4_CLOTH", "Lymhurst", 120, fresh),
    ];
    let histories = vec![history("T4_BAG", "Black Market", fresh, 200)];

    let ranked = rank_opportunities(
        &["T4_BAG".to_owned(), "T4_CAPE".to_owned()],
        &snapshots,
        &histories,
        RankingPreferences {
            current_city: "Caerleon",
            sourcing_strategy: SourcingStrategy::CheapestPerIngredient,
            transport_cost_per_unit: 0,
            premium_status: PremiumStatus::Premium,
            budget: 150_000,
        },
    );
    let bag = ranked.iter().find(|item| item.item_id == "T4_BAG").unwrap();
    let cape = ranked
        .iter()
        .find(|item| item.item_id == "T4_CAPE")
        .unwrap();

    assert!(bag.confidence.score > cape.confidence.score);
}

#[test]
fn rank_opportunities_respects_current_city_strategy() {
    let now = Utc.with_ymd_and_hms(2026, 3, 26, 12, 0, 0).unwrap();
    let snapshots = vec![
        price("T4_BAG", "Black Market", 6100, now),
        price("T4_CLOTH", "Martlock", 120, now),
        price("T4_CLOTH", "Bridgewatch", 70, now),
        price("T4_LEATHER", "Martlock", 80, now),
        price("T4_LEATHER", "Bridgewatch", 40, now),
    ];

    let local_ranked = rank_opportunities(
        &["T4_BAG".to_owned()],
        &snapshots,
        &[],
        RankingPreferences {
            current_city: "Martlock",
            sourcing_strategy: SourcingStrategy::CurrentCityOnly,
            transport_cost_per_unit: 0,
            premium_status: PremiumStatus::Premium,
            budget: 150_000,
        },
    );
    let single_city_ranked = rank_opportunities(
        &["T4_BAG".to_owned()],
        &snapshots,
        &[],
        RankingPreferences {
            current_city: "Martlock",
            sourcing_strategy: SourcingStrategy::SingleCity,
            transport_cost_per_unit: 0,
            premium_status: PremiumStatus::Premium,
            budget: 150_000,
        },
    );

    assert!(single_city_ranked[0].craft_cost < local_ranked[0].craft_cost);
    assert!(local_ranked[0]
        .ingredients
        .iter()
        .all(|ingredient| ingredient.source_city == "Martlock"));
}

#[test]
fn transport_penalty_changes_opportunity_profit() {
    let now = Utc.with_ymd_and_hms(2026, 3, 26, 12, 0, 0).unwrap();
    let snapshots = vec![
        price("T4_BAG", "Black Market", 6100, now),
        price("T4_CLOTH", "Martlock", 120, now),
        price("T4_CLOTH", "Bridgewatch", 60, now),
        price("T4_LEATHER", "Martlock", 80, now),
        price("T4_LEATHER", "Bridgewatch", 40, now),
    ];

    let cheap_remote = rank_opportunities(
        &["T4_BAG".to_owned()],
        &snapshots,
        &[],
        RankingPreferences {
            current_city: "Martlock",
            sourcing_strategy: SourcingStrategy::CheapestPerIngredient,
            transport_cost_per_unit: 0,
            premium_status: PremiumStatus::Premium,
            budget: 150_000,
        },
    );
    let penalized_remote = rank_opportunities(
        &["T4_BAG".to_owned()],
        &snapshots,
        &[],
        RankingPreferences {
            current_city: "Martlock",
            sourcing_strategy: SourcingStrategy::CheapestPerIngredient,
            transport_cost_per_unit: 50,
            premium_status: PremiumStatus::Premium,
            budget: 150_000,
        },
    );

    assert!(penalized_remote[0].craft_cost > cheap_remote[0].craft_cost);
    assert!(penalized_remote[0].net_profit < cheap_remote[0].net_profit);
}

#[test]
fn ranking_prefers_higher_total_profit_with_budget() {
    let now = Utc.with_ymd_and_hms(2026, 3, 26, 12, 0, 0).unwrap();
    let snapshots = vec![
        price("T4_BAG", "Black Market", 5300, now),
        price("T5_BAG", "Black Market", 12000, now),
        price("T4_LEATHER", "Martlock", 50, now),
        price("T4_CLOTH", "Lymhurst", 90, now),
        price("T5_LEATHER", "Martlock", 300, now),
        price("T5_CLOTH", "Lymhurst", 350, now),
    ];

    let ranked = rank_opportunities(
        &["T4_BAG".to_owned(), "T5_BAG".to_owned()],
        &snapshots,
        &[],
        RankingPreferences {
            current_city: "Caerleon",
            sourcing_strategy: SourcingStrategy::CheapestPerIngredient,
            transport_cost_per_unit: 0,
            premium_status: PremiumStatus::Premium,
            budget: 20_000,
        },
    );

    assert_eq!(ranked[0].item_id, "T4_BAG");
    assert!(ranked[0].total_net_profit > ranked[1].total_net_profit);
    assert!(ranked[0].net_profit < ranked[1].net_profit);
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
