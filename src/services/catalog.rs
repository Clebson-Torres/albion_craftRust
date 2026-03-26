use crate::domain::item::{ItemFamily, ItemKey};
use crate::domain::recipe::{Recipe, RecipeLine};

const SUPPORTED_ITEMS: [ItemKey; 6] = [
    ItemKey {
        family: ItemFamily::Bag,
        tier: 4,
        enchantment: 0,
    },
    ItemKey {
        family: ItemFamily::Bag,
        tier: 5,
        enchantment: 0,
    },
    ItemKey {
        family: ItemFamily::Bag,
        tier: 6,
        enchantment: 0,
    },
    ItemKey {
        family: ItemFamily::Cape,
        tier: 4,
        enchantment: 0,
    },
    ItemKey {
        family: ItemFamily::Cape,
        tier: 5,
        enchantment: 0,
    },
    ItemKey {
        family: ItemFamily::Cape,
        tier: 6,
        enchantment: 0,
    },
];

pub fn supported_items() -> &'static [ItemKey] {
    &SUPPORTED_ITEMS
}

pub fn supported_item_ids() -> Vec<String> {
    supported_items().iter().map(item_id_for).collect()
}

pub fn supported_material_ids() -> Vec<String> {
    let mut ids = Vec::new();
    for recipe in supported_item_ids()
        .iter()
        .filter_map(|item_id| recipe_for(item_id.as_str()))
    {
        for ingredient in &recipe.ingredients {
            if !ids.contains(&ingredient.material_id) {
                ids.push(ingredient.material_id.clone());
            }
        }
    }
    ids
}

pub fn item_id_for(item: &ItemKey) -> String {
    match item.family {
        ItemFamily::Bag => format!("T{}_BAG", item.tier),
        ItemFamily::Cape => format!("T{}_CAPE", item.tier),
    }
}

pub fn recipe_for(item_id: &str) -> Option<Recipe> {
    let recipe = match item_id {
        "T4_BAG" => Recipe {
            item_id: item_id.to_owned(),
            ingredients: vec![ingredient("T4_LEATHER", 16), ingredient("T4_CLOTH", 8)],
        },
        "T5_BAG" => Recipe {
            item_id: item_id.to_owned(),
            ingredients: vec![ingredient("T5_LEATHER", 16), ingredient("T5_CLOTH", 8)],
        },
        "T6_BAG" => Recipe {
            item_id: item_id.to_owned(),
            ingredients: vec![ingredient("T6_LEATHER", 16), ingredient("T6_CLOTH", 8)],
        },
        "T4_CAPE" => Recipe {
            item_id: item_id.to_owned(),
            ingredients: vec![ingredient("T4_CLOTH", 14), ingredient("T4_LEATHER", 4)],
        },
        "T5_CAPE" => Recipe {
            item_id: item_id.to_owned(),
            ingredients: vec![ingredient("T5_CLOTH", 14), ingredient("T5_LEATHER", 4)],
        },
        "T6_CAPE" => Recipe {
            item_id: item_id.to_owned(),
            ingredients: vec![ingredient("T6_CLOTH", 14), ingredient("T6_LEATHER", 4)],
        },
        _ => return None,
    };

    Some(recipe)
}

fn ingredient(material_id: &str, amount: u16) -> RecipeLine {
    RecipeLine {
        material_id: material_id.to_owned(),
        amount,
    }
}
