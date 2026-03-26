#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipeLine {
    pub material_id: String,
    pub amount: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recipe {
    pub item_id: String,
    pub ingredients: Vec<RecipeLine>,
}
