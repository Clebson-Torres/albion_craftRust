use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Confidence {
    pub score: u32,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngredientSource {
    pub material_id: String,
    pub source_city: String,
    pub unit_price: u64,
    pub amount: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CraftCostBreakdown {
    pub total_cost: u64,
    pub ingredients: Vec<IngredientSource>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Opportunity {
    pub item_id: String,
    pub craft_cost: u64,
    pub sell_target: String,
    pub sell_price: u64,
    pub net_profit: i64,
    pub max_quantity: u64,
    pub budget_used: u64,
    pub budget_remaining: u64,
    pub total_net_profit: i64,
    pub buy_fee_total: u64,
    pub sell_fee_total: u64,
    pub total_fee_impact: u64,
    pub confidence: Confidence,
    pub ingredients: Vec<IngredientSource>,
    pub observed_at: String,
}
