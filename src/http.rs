use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillInventoryBody {
    pub item: u8,
    pub overwrite_important_items: bool,
    pub only_empty_slots: bool,
}

#[derive(Deserialize)]
pub struct GetInventoryBody {
    pub slot: usize,
}

#[derive(Serialize)]
pub struct ItemModel {
    pub id: u8,
    pub name: String,
}

#[derive(Serialize)]
pub struct GetItemsResponse {
    pub items: Vec<ItemModel>,
}

#[derive(Serialize)]
pub struct GetInventoryResponse {
    pub item: u8,
}