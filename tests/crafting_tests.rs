use albion_crafting_overlay::domain::market::{MarketSnapshot, MarketVenue};
use albion_crafting_overlay::services::catalog::recipe_for;
use albion_crafting_overlay::services::crafting::{calculate_craft_cost, SourcingStrategy};
use chrono::{TimeZone, Utc};

#[test]
fn recipe_lookup_returns_ingredients_for_supported_item() {
    let recipe = recipe_for("T4_BAG").expect("recipe");
    assert!(!recipe.ingredients.is_empty());
}

#[test]
fn calculate_craft_cost_uses_cheapest_available_inputs() {
    let recipe = recipe_for("T4_BAG").expect("recipe");
    let observed_at = Utc.with_ymd_and_hms(2026, 3, 26, 10, 0, 0).unwrap();
    let snapshots = vec![
        MarketSnapshot {
            item_id: "T4_LEATHER".to_owned(),
            city: MarketVenue::new("Bridgewatch".to_owned()),
            sell_price_min: 80,
            sell_price_max: 90,
            buy_price_max: 0,
            quality: 1,
            observed_at,
        },
        MarketSnapshot {
            item_id: "T4_LEATHER".to_owned(),
            city: MarketVenue::new("Martlock".to_owned()),
            sell_price_min: 70,
            sell_price_max: 75,
            buy_price_max: 0,
            quality: 1,
            observed_at,
        },
        MarketSnapshot {
            item_id: "T4_CLOTH".to_owned(),
            city: MarketVenue::new("Lymhurst".to_owned()),
            sell_price_min: 120,
            sell_price_max: 125,
            buy_price_max: 0,
            quality: 1,
            observed_at,
        },
    ];

    let cost = calculate_craft_cost(
        &recipe,
        &snapshots,
        SourcingStrategy::CheapestPerIngredient,
        "Caerleon",
        0,
    )
    .expect("cost");

    assert_eq!(cost.total_cost, 2080);
    assert_eq!(cost.ingredients.len(), 2);
    assert_eq!(cost.ingredients[0].source_city, "Martlock");
}

#[test]
fn calculate_craft_cost_uses_current_city_for_local_strategy() {
    let recipe = recipe_for("T4_BAG").expect("recipe");
    let observed_at = Utc.with_ymd_and_hms(2026, 3, 26, 10, 0, 0).unwrap();
    let snapshots = vec![
        MarketSnapshot {
            item_id: "T4_LEATHER".to_owned(),
            city: MarketVenue::new("Bridgewatch".to_owned()),
            sell_price_min: 50,
            sell_price_max: 55,
            buy_price_max: 0,
            quality: 1,
            observed_at,
        },
        MarketSnapshot {
            item_id: "T4_LEATHER".to_owned(),
            city: MarketVenue::new("Martlock".to_owned()),
            sell_price_min: 80,
            sell_price_max: 85,
            buy_price_max: 0,
            quality: 1,
            observed_at,
        },
        MarketSnapshot {
            item_id: "T4_CLOTH".to_owned(),
            city: MarketVenue::new("Lymhurst".to_owned()),
            sell_price_min: 100,
            sell_price_max: 110,
            buy_price_max: 0,
            quality: 1,
            observed_at,
        },
        MarketSnapshot {
            item_id: "T4_CLOTH".to_owned(),
            city: MarketVenue::new("Martlock".to_owned()),
            sell_price_min: 130,
            sell_price_max: 135,
            buy_price_max: 0,
            quality: 1,
            observed_at,
        },
        MarketSnapshot {
            item_id: "T4_CLOTH".to_owned(),
            city: MarketVenue::new("Bridgewatch".to_owned()),
            sell_price_min: 90,
            sell_price_max: 95,
            buy_price_max: 0,
            quality: 1,
            observed_at,
        },
    ];

    let local_cost = calculate_craft_cost(
        &recipe,
        &snapshots,
        SourcingStrategy::CurrentCityOnly,
        "Martlock",
        0,
    )
    .expect("local cost");
    let single_city_cost = calculate_craft_cost(
        &recipe,
        &snapshots,
        SourcingStrategy::SingleCity,
        "Martlock",
        0,
    )
    .expect("single city cost");

    assert_eq!(local_cost.total_cost, 16 * 80 + 8 * 130);
    assert!(single_city_cost.total_cost < local_cost.total_cost);
    assert!(local_cost
        .ingredients
        .iter()
        .all(|ingredient| ingredient.source_city == "Martlock"));
}

#[test]
fn transport_penalty_can_outweigh_remote_material_discount() {
    let recipe = recipe_for("T4_BAG").expect("recipe");
    let observed_at = Utc.with_ymd_and_hms(2026, 3, 26, 10, 0, 0).unwrap();
    let snapshots = vec![
        MarketSnapshot {
            item_id: "T4_LEATHER".to_owned(),
            city: MarketVenue::new("Martlock".to_owned()),
            sell_price_min: 70,
            sell_price_max: 75,
            buy_price_max: 0,
            quality: 1,
            observed_at,
        },
        MarketSnapshot {
            item_id: "T4_LEATHER".to_owned(),
            city: MarketVenue::new("Bridgewatch".to_owned()),
            sell_price_min: 40,
            sell_price_max: 45,
            buy_price_max: 0,
            quality: 1,
            observed_at,
        },
        MarketSnapshot {
            item_id: "T4_CLOTH".to_owned(),
            city: MarketVenue::new("Martlock".to_owned()),
            sell_price_min: 100,
            sell_price_max: 105,
            buy_price_max: 0,
            quality: 1,
            observed_at,
        },
        MarketSnapshot {
            item_id: "T4_CLOTH".to_owned(),
            city: MarketVenue::new("Bridgewatch".to_owned()),
            sell_price_min: 60,
            sell_price_max: 65,
            buy_price_max: 0,
            quality: 1,
            observed_at,
        },
    ];

    let no_penalty = calculate_craft_cost(
        &recipe,
        &snapshots,
        SourcingStrategy::CheapestPerIngredient,
        "Martlock",
        0,
    )
    .expect("no penalty");
    let with_penalty = calculate_craft_cost(
        &recipe,
        &snapshots,
        SourcingStrategy::CheapestPerIngredient,
        "Martlock",
        50,
    )
    .expect("with penalty");

    assert!(with_penalty.total_cost > no_penalty.total_cost);
    assert_eq!(with_penalty.total_cost, (16 * (40 + 50)) + (8 * (60 + 50)));
}
