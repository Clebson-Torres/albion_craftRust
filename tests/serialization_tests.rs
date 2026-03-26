use albion_crafting_overlay::domain::opportunity::{Confidence, IngredientSource, Opportunity};
use albion_crafting_overlay::services::refresh::RefreshResult;

#[test]
fn tauri_payload_uses_camel_case_keys() {
    let payload = RefreshResult {
        items: vec![Opportunity {
            item_id: "T4_BAG".to_owned(),
            craft_cost: 1234,
            sell_target: "Black Market".to_owned(),
            sell_price: 5678,
            net_profit: 4444,
            max_quantity: 12,
            budget_used: 14808,
            budget_remaining: 192,
            total_net_profit: 53328,
            buy_fee_total: 12,
            sell_fee_total: 22,
            total_fee_impact: 34,
            confidence: Confidence {
                score: 90,
                label: "High".to_owned(),
            },
            ingredients: vec![IngredientSource {
                material_id: "T4_CLOTH".to_owned(),
                source_city: "Martlock".to_owned(),
                unit_price: 100,
                amount: 8,
            }],
            observed_at: "2026-03-26T12:00:00Z".to_owned(),
        }],
        stale: false,
    };

    let value = serde_json::to_value(payload).expect("serialize payload");

    let item = &value["items"][0];
    assert_eq!(item["itemId"], "T4_BAG");
    assert_eq!(item["sellTarget"], "Black Market");
    assert_eq!(item["observedAt"], "2026-03-26T12:00:00Z");
    assert_eq!(item["maxQuantity"], 12);
    assert_eq!(item["budgetUsed"], 14808);
    assert_eq!(item["budgetRemaining"], 192);
    assert_eq!(item["totalNetProfit"], 53328);
    assert_eq!(item["buyFeeTotal"], 12);
    assert_eq!(item["sellFeeTotal"], 22);
    assert_eq!(item["totalFeeImpact"], 34);
    assert_eq!(item["ingredients"][0]["materialId"], "T4_CLOTH");
    assert_eq!(item["ingredients"][0]["sourceCity"], "Martlock");
    assert_eq!(item["ingredients"][0]["unitPrice"], 100);
}
