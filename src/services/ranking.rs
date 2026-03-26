use serde::{Deserialize, Serialize};

use chrono::{Duration, Utc};

use crate::domain::market::{MarketHistory, MarketSnapshot};
use crate::domain::opportunity::{Confidence, Opportunity};
use crate::services::catalog::recipe_for;
use crate::services::crafting::{calculate_craft_cost, SourcingStrategy};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PremiumStatus {
    Premium,
    Standard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RankingPreferences<'a> {
    pub current_city: &'a str,
    pub sourcing_strategy: SourcingStrategy,
    pub transport_cost_per_unit: u64,
    pub premium_status: PremiumStatus,
    pub budget: u64,
}

impl PremiumStatus {
    pub fn buy_tax_bps(self) -> u64 {
        match self {
            Self::Premium => 5,
            Self::Standard => 15,
        }
    }

    pub fn sell_tax_bps(self) -> u64 {
        match self {
            Self::Premium => 40,
            Self::Standard => 65,
        }
    }
}

pub fn rank_opportunities(
    item_ids: &[String],
    snapshots: &[MarketSnapshot],
    histories: &[MarketHistory],
    preferences: RankingPreferences<'_>,
) -> Vec<Opportunity> {
    let mut opportunities = Vec::new();

    for item_id in item_ids {
        let Some(recipe) = recipe_for(item_id) else {
            continue;
        };
        let Some(cost) = calculate_craft_cost(
            &recipe,
            snapshots,
            preferences.sourcing_strategy,
            preferences.current_city,
            preferences.transport_cost_per_unit,
        ) else {
            continue;
        };
        let Some(best_sale) = snapshots
            .iter()
            .filter(|snapshot| snapshot.item_id == *item_id && snapshot.sell_price_min > 0)
            .max_by_key(|snapshot| snapshot.sell_price_min)
        else {
            continue;
        };

        let buy_fee_total = (cost.total_cost * preferences.premium_status.buy_tax_bps()) / 10_000;
        let sell_fee_total =
            (best_sale.sell_price_min * preferences.premium_status.sell_tax_bps()) / 10_000;
        let total_cost_with_fees = cost.total_cost + buy_fee_total;
        let net_profit =
            best_sale.sell_price_min as i64 - total_cost_with_fees as i64 - sell_fee_total as i64;
        let max_quantity = if total_cost_with_fees == 0 {
            0
        } else {
            preferences.budget / total_cost_with_fees
        };
        let budget_used = max_quantity * total_cost_with_fees;
        let budget_remaining = preferences.budget.saturating_sub(budget_used);
        let total_net_profit = net_profit * max_quantity as i64;
        let confidence = score_confidence(item_id, best_sale, histories, net_profit);

        opportunities.push(Opportunity {
            item_id: item_id.clone(),
            craft_cost: cost.total_cost,
            sell_target: best_sale.city.as_str().to_owned(),
            sell_price: best_sale.sell_price_min,
            net_profit,
            max_quantity,
            budget_used,
            budget_remaining,
            total_net_profit,
            buy_fee_total,
            sell_fee_total,
            total_fee_impact: buy_fee_total + sell_fee_total,
            confidence,
            ingredients: cost.ingredients,
            observed_at: best_sale.observed_at.to_rfc3339(),
        });
    }

    opportunities.sort_by(|left, right| {
        right
            .total_net_profit
            .cmp(&left.total_net_profit)
            .then_with(|| right.net_profit.cmp(&left.net_profit))
            .then_with(|| right.confidence.score.cmp(&left.confidence.score))
    });
    opportunities
}

fn score_confidence(
    item_id: &str,
    best_sale: &MarketSnapshot,
    histories: &[MarketHistory],
    net_profit: i64,
) -> Confidence {
    let now = Utc::now();
    let age = now - best_sale.observed_at;
    let freshness_score = if age <= Duration::hours(6) {
        50
    } else if age <= Duration::hours(24) {
        30
    } else {
        5
    };

    let history_score = histories
        .iter()
        .find(|history| {
            history.item_id == item_id && history.location.as_str() == best_sale.city.as_str()
        })
        .and_then(|history| history.data.last())
        .map(|bucket| bucket.item_count.min(300) as u32 / 3)
        .unwrap_or(0);

    let margin_score = if net_profit > 2_000 {
        20
    } else if net_profit > 500 {
        10
    } else if net_profit > 0 {
        5
    } else {
        0
    };

    let score = freshness_score + history_score + margin_score;
    let label = if score >= 70 {
        "High"
    } else if score >= 35 {
        "Medium"
    } else {
        "Low"
    };

    Confidence {
        score,
        label: label.to_owned(),
    }
}
