#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemFamily {
    Bag,
    Cape,
}

impl ItemFamily {
    pub fn is_bag(&self) -> bool {
        matches!(self, Self::Bag)
    }

    pub fn is_cape(&self) -> bool {
        matches!(self, Self::Cape)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemKey {
    pub family: ItemFamily,
    pub tier: u8,
    pub enchantment: u8,
}

impl ItemKey {
    pub fn new(family: ItemFamily, tier: u8, enchantment: u8) -> Self {
        Self {
            family,
            tier,
            enchantment,
        }
    }
}
