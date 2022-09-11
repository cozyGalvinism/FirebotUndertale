use reqwest::Client;
use tput_proc::mem_value_structs;
use serde::{Deserialize, Serialize};
use pretty_assertions::assert_eq;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FillInventoryBody {
    pub item: u8,
    pub overwrite_important_items: bool,
    pub only_empty_slots: bool,
}

#[derive(Serialize)]
pub struct GetInventoryBody {
    pub slot: usize,
}

#[derive(Deserialize)]
pub struct ItemModel {
    pub id: u8,
    pub name: String,
}

#[derive(Deserialize)]
pub struct GetItemsResponse {
    pub items: Vec<ItemModel>,
}

#[derive(Deserialize)]
pub struct GetInventoryResponse {
    pub item: u8,
}

mem_value_structs!(kill_area, f64);
mem_value_structs!(health, f64);
mem_value_structs!(max_health, f64);
mem_value_structs!(gold, f64);
mem_value_structs!(speed, f64);
mem_value_structs!(equipped_weapon, f64);
mem_value_structs!(equipped_armor, f64);
mem_value_structs!(encounter_counter, f64);
mem_value_structs!(kills_ruins, f64);
mem_value_structs!(kills_snowdin, f64);
mem_value_structs!(kills_waterfall, f64);
mem_value_structs!(kills_hotland, f64);

fn setup() -> Client {
    Client::new()
}

#[tokio::test]
async fn test_get_kill_areas() -> Result<(), ()> {
    dotenv::dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "1337".to_string());
    let client = setup();
    let res = client.get(&format!("http://localhost:{}/getKillArea", port))
        .send()
        .await
        .unwrap()
        .json::<GetKillAreaResponse>()
        .await
        .unwrap();
    pretty_assertions::assert_eq!(res.kill_area, 0.0);

    Ok(())
}

#[tokio::test]
async fn test_set_health() -> Result<(), ()> {
    dotenv::dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "1337".to_string());
    let client = setup();
    client.post(&format!("http://localhost:{}/setHealth", port))
        .json(&SetHealthRequest { health: 10.0 })
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    Ok(())
}

#[tokio::test]
async fn test_get_health() -> Result<(), ()> {
    dotenv::dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "1337".to_string());
    let client = setup();
    let res = client.get(&format!("http://localhost:{}/getHealth", port))
        .send()
        .await
        .unwrap()
        .json::<GetHealthResponse>()
        .await
        .unwrap();
    pretty_assertions::assert_eq!(res.health > 0.0, true);

    Ok(())
}

#[tokio::test]
async fn test_get_max_health() -> Result<(), ()> {
    dotenv::dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "1337".to_string());
    let client = setup();
    let res = client.get(&format!("http://localhost:{}/getMaxHealth", port))
        .send()
        .await
        .unwrap()
        .json::<GetMaxHealthResponse>()
        .await
        .unwrap();
    pretty_assertions::assert_eq!(res.max_health == 20.0, true);

    Ok(())
}

#[tokio::test]
async fn test_set_gold() -> Result<(), ()> {
    dotenv::dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "1337".to_string());
    let client = setup();
    client.post(&format!("http://localhost:{}/setGold", port))
        .json(&SetGoldRequest { gold: 10.0 })
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();

    Ok(())
}

#[tokio::test]
async fn test_get_gold() -> Result<(), ()> {
    dotenv::dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "1337".to_string());
    let client = setup();
    let res = client.get(&format!("http://localhost:{}/getGold", port))
        .send()
        .await
        .unwrap()
        .json::<GetGoldResponse>()
        .await
        .unwrap();
    pretty_assertions::assert_eq!(res.gold >= 0.0, true);

    Ok(())
}

#[tokio::test]
async fn test_get_equipped_weapon() -> Result<(), ()> {
    dotenv::dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "1337".to_string());
    let client = setup();
    let res = client.get(&format!("http://localhost:{}/getEquippedWeapon", port))
        .send()
        .await
        .unwrap()
        .json::<GetEquippedWeaponResponse>()
        .await
        .unwrap();
    pretty_assertions::assert_eq!(res.equipped_weapon, 3.0);

    Ok(())
}

#[tokio::test]
async fn test_get_equipped_armor() -> Result<(), ()> {
    dotenv::dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "1337".to_string());
    let client = setup();
    let res = client.get(&format!("http://localhost:{}/getEquippedArmor", port))
        .send()
        .await
        .unwrap()
        .json::<GetEquippedArmorResponse>()
        .await
        .unwrap();
    pretty_assertions::assert_eq!(res.equipped_armor, 4.0);

    Ok(())
}

#[tokio::test]
async fn test_inventory() -> Result<(), ()> {
    dotenv::dotenv().ok();

    let port = std::env::var("PORT").unwrap_or_else(|_| "1337".to_string());
    let client = setup();
    client.post(&format!("http://localhost:{}/fillInventory", port))
        .json(&FillInventoryBody {
            item: 29,
            only_empty_slots: true,
            overwrite_important_items: false
        })
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
    
    let res = client.post(&format!("http://localhost:{}/getInventory", port))
        .json(&GetInventoryBody {
            slot: 1
        })
        .send()
        .await
        .unwrap()
        .json::<GetInventoryResponse>()
        .await
        .unwrap();
    
    pretty_assertions::assert_eq!(res.item, 29);

    Ok(())
}
