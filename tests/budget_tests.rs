use albion_crafting_overlay::domain::market::{MarketHistory, MarketSnapshot, MarketVenue};
use albion_crafting_overlay::services::crafting::SourcingStrategy;
use albion_crafting_overlay::services::ranking::{
    rank_opportunities, PremiumStatus, RankingPreferences,
};
use chrono::{TimeZone, Utc};

#[test]
fn budget_limits_quantity_and_total_profit() {
    let now = Utc.with_ymd_and_hms(2026, 3, 26, 12, 0, 0).unwrap();
    let snapshots = vec![
        price("T4_BAG", "Black Market", 6100, now),
        price("T4_CLOTH", "Martlock", 120, now),
        price("T4_LEATHER", "Martlock", 80, now),
    ];

    let ranked = rank_opportunities(
        &["T4_BAG".to_owned()],
        &snapshots,
        &Vec::<MarketHistory>::new(),
        RankingPreferences {
            current_city: "Martlock",
            sourcing_strategy: SourcingStrategy::CurrentCityOnly,
            transport_cost_per_unit: 0,
            premium_status: PremiumStatus::Premium,
            budget: 10_000,
        },
    );

    assert_eq!(ranked[0].max_quantity, 4);
    assert!(ranked[0].budget_used <= 10_000);
    assert_eq!(ranked[0].budget_remaining, 10_000 - ranked[0].budget_used);
    assert_eq!(
        ranked[0].total_net_profit,
        ranked[0].net_profit * ranked[0].max_quantity as i64
    );
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
