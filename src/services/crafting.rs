use serde::{Deserialize, Serialize};

use crate::domain::market::MarketSnapshot;
use crate::domain::opportunity::{CraftCostBreakdown, IngredientSource};
use crate::domain::recipe::Recipe;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourcingStrategy {
    CheapestPerIngredient,
    CurrentCityOnly,
    SingleCity,
}

impl SourcingStrategy {
    pub const ALL: [Self; 3] = [
        Self::CurrentCityOnly,
        Self::SingleCity,
        Self::CheapestPerIngredient,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::CheapestPerIngredient => "Mais barato por material",
            Self::CurrentCityOnly => "Compra local",
            Self::SingleCity => "Cidade unica",
        }
    }
}

pub fn calculate_craft_cost(
    recipe: &Recipe,
    snapshots: &[MarketSnapshot],
    strategy: SourcingStrategy,
    current_city: &str,
    transport_cost_per_unit: u64,
) -> Option<CraftCostBreakdown> {
    match strategy {
        SourcingStrategy::CheapestPerIngredient => {
            cheapest_per_ingredient_cost(recipe, snapshots, current_city, transport_cost_per_unit)
        }
        SourcingStrategy::CurrentCityOnly => current_city_cost(recipe, snapshots, current_city),
        SourcingStrategy::SingleCity => {
            single_city_cost(recipe, snapshots, current_city, transport_cost_per_unit)
        }
    }
}

fn cheapest_per_ingredient_cost(
    recipe: &Recipe,
    snapshots: &[MarketSnapshot],
    current_city: &str,
    transport_cost_per_unit: u64,
) -> Option<CraftCostBreakdown> {
    let mut total_cost = 0_u64;
    let mut ingredients = Vec::with_capacity(recipe.ingredients.len());

    for line in &recipe.ingredients {
        let cheapest = snapshots
            .iter()
            .filter(|snapshot| snapshot.item_id == line.material_id && snapshot.sell_price_min > 0)
            .min_by_key(|snapshot| snapshot.sell_price_min)?;

        let unit_price = apply_transport_penalty(
            cheapest.sell_price_min,
            cheapest.city.as_str(),
            current_city,
            transport_cost_per_unit,
        );
        total_cost += unit_price * u64::from(line.amount);
        ingredients.push(IngredientSource {
            material_id: line.material_id.clone(),
            source_city: cheapest.city.as_str().to_owned(),
            unit_price,
            amount: line.amount,
        });
    }

    Some(CraftCostBreakdown {
        total_cost,
        ingredients,
    })
}

fn current_city_cost(
    recipe: &Recipe,
    snapshots: &[MarketSnapshot],
    current_city: &str,
) -> Option<CraftCostBreakdown> {
    let mut total_cost = 0_u64;
    let mut ingredients = Vec::with_capacity(recipe.ingredients.len());

    for line in &recipe.ingredients {
        let local = snapshots.iter().find(|snapshot| {
            snapshot.item_id == line.material_id
                && snapshot.city.as_str() == current_city
                && snapshot.sell_price_min > 0
        })?;

        total_cost += local.sell_price_min * u64::from(line.amount);
        ingredients.push(IngredientSource {
            material_id: line.material_id.clone(),
            source_city: local.city.as_str().to_owned(),
            unit_price: local.sell_price_min,
            amount: line.amount,
        });
    }

    Some(CraftCostBreakdown {
        total_cost,
        ingredients,
    })
}

fn single_city_cost(
    recipe: &Recipe,
    snapshots: &[MarketSnapshot],
    current_city: &str,
    transport_cost_per_unit: u64,
) -> Option<CraftCostBreakdown> {
    let mut candidate_cities = snapshots
        .iter()
        .map(|snapshot| snapshot.city.as_str().to_owned())
        .collect::<Vec<_>>();
    candidate_cities.sort();
    candidate_cities.dedup();

    candidate_cities
        .into_iter()
        .filter_map(|city| {
            current_city_cost(recipe, snapshots, &city).map(|mut breakdown| {
                if city != current_city {
                    for ingredient in &mut breakdown.ingredients {
                        ingredient.unit_price += transport_cost_per_unit;
                    }
                    breakdown.total_cost += breakdown
                        .ingredients
                        .iter()
                        .map(|ingredient| transport_cost_per_unit * u64::from(ingredient.amount))
                        .sum::<u64>();
                }
                breakdown
            })
        })
        .min_by_key(|cost| cost.total_cost)
}

fn apply_transport_penalty(
    base_price: u64,
    source_city: &str,
    current_city: &str,
    transport_cost_per_unit: u64,
) -> u64 {
    if source_city == current_city {
        base_price
    } else {
        base_price + transport_cost_per_unit
    }
}
