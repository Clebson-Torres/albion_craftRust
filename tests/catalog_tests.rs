use albion_crafting_overlay::domain::item::{ItemFamily, ItemKey};
use albion_crafting_overlay::services::catalog::supported_items;

#[test]
fn item_key_preserves_family_and_tier() {
    let item = ItemKey::new(ItemFamily::Bag, 4, 0);
    assert_eq!(item.tier, 4);
    assert_eq!(item.enchantment, 0);
}

#[test]
fn supported_items_contains_bags_and_capes() {
    let items = supported_items();
    assert!(items.iter().any(|item| item.family.is_bag()));
    assert!(items.iter().any(|item| item.family.is_cape()));
}
