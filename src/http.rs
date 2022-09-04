use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SetHealthBody {
    pub health: f64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FillInventoryBody {
    pub item: u8,
    pub overwrite_important_items: bool,
    pub only_empty_slots: bool,
}

#[derive(Deserialize)]
pub struct DisableInputBody {
    pub disable: bool,
}

#[derive(Deserialize)]
pub struct GetInventoryBody {
    pub slot: usize,
}

#[derive(Deserialize)]
pub struct SetGoldBody {
    pub gold: f64,
}

#[derive(Deserialize)]
pub struct SetEncounterBody {
    pub counter: f64,
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

#[derive(Serialize)]
pub struct GetHealthResponse {
    pub health: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxHealthResponse {
    pub max_health: f64,
}

#[derive(Serialize)]
pub struct GetGoldResponse {
    pub gold: f64,
}

#[derive(Deserialize)]
pub struct SetSpeedBody {
    pub speed: f64,
}

#[derive(Serialize)]
pub struct GetSpeedResponse {
    pub speed: f64,
}