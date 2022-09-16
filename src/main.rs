use std::{sync::Arc, time::Duration};

use axum::{Router, response::IntoResponse, routing::{post, get}, Extension, Json, http::StatusCode};
use http::{ItemModel, GetItemsResponse, FillInventoryBody, GetInventoryBody, GetInventoryResponse, SetInventoryBody};
use magic::*;
use serde_json::json;
use strum::IntoEnumIterator;
use sysinfo::{System, SystemExt, ProcessExt, PidExt};
use tokio::sync::Mutex;
use tput_proc::mem_value;
use vmemory::ProcessMemory;
use num_traits::{ToPrimitive, FromPrimitive};

use crate::config::AppConfig;

mod magic;
mod http;
mod config;

#[derive(Clone)]
struct UndertaleGame {
    pub process: Arc<Mutex<ProcessMemory>>,
    pub ready: Arc<Mutex<bool>>,
    pub config: Arc<AppConfig>,
}

fn inventory_address(process_memory: &ProcessMemory, config: &AppConfig, slot: usize) -> usize {
    match slot {
        0 => crate::config::parse_offsets_from_ce_string(&config.addresses.inventory_slot_1).fetch_address(process_memory),
        1 => crate::config::parse_offsets_from_ce_string(&config.addresses.inventory_slot_2).fetch_address(process_memory),
        2 => crate::config::parse_offsets_from_ce_string(&config.addresses.inventory_slot_3).fetch_address(process_memory),
        3 => crate::config::parse_offsets_from_ce_string(&config.addresses.inventory_slot_4).fetch_address(process_memory),
        4 => crate::config::parse_offsets_from_ce_string(&config.addresses.inventory_slot_5).fetch_address(process_memory),
        5 => crate::config::parse_offsets_from_ce_string(&config.addresses.inventory_slot_6).fetch_address(process_memory),
        6 => crate::config::parse_offsets_from_ce_string(&config.addresses.inventory_slot_7).fetch_address(process_memory),
        7 => crate::config::parse_offsets_from_ce_string(&config.addresses.inventory_slot_8).fetch_address(process_memory),
        _ => panic!("Invalid slot number"),
    }
}

fn get_inventory_item(process: &ProcessMemory, config: &AppConfig, slot: usize) -> Option<Item> {
    if slot >= INVENTORY_OFFSETS.len() {
        return None;
    }
    let inventory_address = inventory_address(process, config, slot);
    let item_bytes = process.read_memory(inventory_address, 8, false);
    let value = f64::from_le_bytes(item_bytes.try_into().unwrap());
    tracing::info!("Read value {} from address {:x} (get_inventory_item, slot {})", value, inventory_address, slot);

    num_traits::FromPrimitive::from_f64(value)
}

fn set_inventory_item(process: &ProcessMemory, config: &AppConfig, slot: usize, item: Item) {
    if slot >= INVENTORY_OFFSETS.len() {
        return;
    }
    let inventory_address = inventory_address(process, config, slot);
    let value = num_traits::ToPrimitive::to_f64(&item).unwrap();
    let item_bytes = value.to_le_bytes().to_vec();
    process.write_memory(inventory_address, &item_bytes, false);
    tracing::info!("Wrote value {} to address {:x} (set_inventory_item, slot {})", value, inventory_address, slot);
}

fn fill_inventory_with(process: &ProcessMemory, config: &AppConfig, item: Item, overwrite_important_items: bool, only_empty_slots: bool) {
    for slot in 0..INVENTORY_OFFSETS.len() {
        let inventory_item = get_inventory_item(process, config, slot);
        if let Some(inventory_item) = inventory_item {
            if !overwrite_important_items && inventory_item.is_important_item() {
                tracing::debug!("Not overwriting important item {}", inventory_item);
                continue;
            }

            if only_empty_slots && !matches!(inventory_item, Item::Empty) {
                tracing::debug!("Not overwriting non-empty slot {}", inventory_item);
                continue;
            }
            set_inventory_item(process, config, slot, item);
        } else {
            tracing::warn!("Failed to get inventory item at slot {}", slot);
        }
    }
}

async fn http_fill_inventory(
    process_extension: Extension<Arc<UndertaleGame>>,
    Json(fill_inventory_body): Json<FillInventoryBody>
) -> impl IntoResponse {
    let process_memory = process_extension.process.lock().await;
    fill_inventory_with(&process_memory, &process_extension.config, Item::from_u8(fill_inventory_body.item).unwrap(), fill_inventory_body.overwrite_important_items, fill_inventory_body.only_empty_slots);

    (StatusCode::OK, Json(json!({"status": "ok"}))).into_response()
}

async fn http_get_items() -> impl IntoResponse {
    let items = Item::iter()
        .map(|i| ItemModel {
            id: i.to_u8().unwrap(),
            name: i.to_string(),
        })
        .collect::<Vec<_>>();
    
    (StatusCode::OK, Json(GetItemsResponse { items })).into_response()
}

async fn http_get_inventory_at_slot(
    process_extension: Extension<Arc<UndertaleGame>>,
    Json(get_inventory_body): Json<GetInventoryBody>
) -> impl IntoResponse {
    let process_memory = process_extension.process.lock().await;
    let inventory_item = get_inventory_item(&process_memory, &process_extension.config, get_inventory_body.slot);
    let inventory_item = match inventory_item {
        Some(inventory_item) => inventory_item.to_u8().unwrap(),
        None => 0_u8,
    };
    (StatusCode::OK, Json(GetInventoryResponse { item: inventory_item })).into_response()
}

async fn http_set_inventory_at_slot(
    process_extension: Extension<Arc<UndertaleGame>>,
    Json(set_inventory_body): Json<SetInventoryBody>
) -> impl IntoResponse {
    let process_memory = process_extension.process.lock().await;
    set_inventory_item(&process_memory, &process_extension.config, set_inventory_body.slot, Item::from_u8(set_inventory_body.item).unwrap());
    (StatusCode::OK, Json(json!({"status": "ok"}))).into_response()
}

// Currently unused
/* fn get_main_window_handle(process: u32) -> Vec<usize> {
    let mut window_handles = Vec::new();
    let mut handle = std::ptr::null_mut::<winapi::shared::windef::HWND__>();
    unsafe {
        loop {
            let c_str = CString::new("YYGameMakerYY").unwrap();
            handle = winapi::um::winuser::FindWindowExA(std::ptr::null_mut(), handle, c_str.as_ptr(), std::ptr::null());
            let window_process: *mut u32 = &mut 0;
            winapi::um::winuser::GetWindowThreadProcessId(handle, window_process);
            
            if handle.is_null() {
                break;
            }
            
            if !window_process.is_null() && window_process.read() == process {
                window_handles.push(handle as usize);
            }
        }
    }

    tracing::debug!("window handles: {:?}", window_handles);
    window_handles
} */

mem_value!(kill_area, f64, false);
mem_value!(health, f64, true);
mem_value!(max_health, f64, false);
mem_value!(gold, f64, true);
mem_value!(speed, f64, true);
mem_value!(equipped_weapon, f64, true);
mem_value!(equipped_armor, f64, true);
mem_value!(encounter_counter, f64, true);
mem_value!(kills_ruins, f64, true);
mem_value!(kills_snowdin, f64, true);
mem_value!(kills_waterfall, f64, true);
mem_value!(kills_hotland, f64, true);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = Arc::new(AppConfig::new());
    let port = std::env::var("PORT").unwrap_or_else(|_| "1337".to_string());

    let mut s = System::new_all();

    let mut undertale = s.processes_by_name("UNDERTALE").next();
    let process: Arc<Mutex<ProcessMemory>>;
    
    if let Some(undertale) = undertale {
        tracing::info!("Attaching to Undertale process");
        let undertale_pid: u32 = undertale.pid().as_u32();
        process = Arc::new(Mutex::new(ProcessMemory::attach_process(undertale_pid).unwrap()));
    } else {
        tracing::info!("Undertale not running, waiting for it to start");
        loop {
            tracing::info!("Checking...");
            s.refresh_processes();
            undertale = s.processes_by_name("UNDERTALE").next();
            if undertale.is_some() {
                tracing::info!("There it is!...");
                break;
            }
            tracing::info!("Not there...");

            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        tracing::info!("Attaching to Undertale process");
        let undertale = undertale.unwrap();
        let undertale_pid: u32 = undertale.pid().as_u32();
        process = Arc::new(Mutex::new(ProcessMemory::attach_process(undertale_pid).unwrap()));
    }

    let game = Arc::new(UndertaleGame {
        process,
        ready: Arc::new(Mutex::new(true)),
        config,
    });
    let game_clone = game.clone();

    let refresh_task = tokio::task::spawn(async move {
        let game = game_clone;

        loop {
            s.refresh_processes();
            let mut undertale = s.processes_by_name("UNDERTALE").next();
            if undertale.is_none() {
                {
                    let mut ready = game.ready.lock().await;
                    *ready = false;
                }
                tracing::info!("UNDERTALE is no longer running, waiting...");
                loop {
                    s.refresh_processes();
                    undertale = s.processes_by_name("UNDERTALE").next();
                    if undertale.is_some() {
                        break;
                    }

                    tokio::time::sleep(Duration::from_millis(500)).await;
                }

                let undertale = undertale.unwrap();
                let undertale_pid: u32 = undertale.pid().as_u32();
                let process = ProcessMemory::attach_process(undertale_pid).unwrap();
                let mut existing_process = game.process.lock().await;
                let _ = std::mem::replace(&mut *existing_process, process);
                let mut ready = game.ready.lock().await;
                *ready = true;
            }

            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });

    let app = Router::new()
        .route("/setHealth", post(http_set_health))
        .route("/getHealth", get(http_get_health))
        .route("/getMaxHealth", get(http_get_max_health))
        .route("/getGold", get(http_get_gold))
        .route("/setGold", post(http_set_gold))
        .route("/getItems", get(http_get_items))
        .route("/fillInventory", post(http_fill_inventory))
        .route("/getInventory", post(http_get_inventory_at_slot))
        .route("/setInventory", post(http_set_inventory_at_slot))
        .route("/getEncounter", get(http_get_encounter_counter))
        .route("/setEncounter", post(http_set_encounter_counter))
        .route("/getSpeed", get(http_get_speed))
        .route("/setSpeed", post(http_set_speed))
        .route("/getEquippedWeapon", get(http_get_equipped_weapon))
        .route("/setEquippedWeapon", post(http_set_equipped_weapon))
        .route("/getEquippedArmor", get(http_get_equipped_armor))
        .route("/setEquippedArmor", post(http_set_equipped_armor))
        .route("/getKillArea", get(http_get_kill_area))
        .route("/getKillsRuins", get(http_get_kills_ruins))
        .route("/getKillsSnowdin", get(http_get_kills_snowdin))
        .route("/getKillsWaterfall", get(http_get_kills_waterfall))
        .route("/getKillsHotland", get(http_get_kills_hotland))
        .route("/setKillsRuins", post(http_set_kills_ruins))
        .route("/setKillsSnowdin", post(http_set_kills_snowdin))
        .route("/setKillsWaterfall", post(http_set_kills_waterfall))
        .route("/setKillsHotland", post(http_set_kills_hotland))

        .layer(Extension(game));
    
    tracing::debug!("Serving on http://localhost:{}", &port);
    axum::Server::bind(&format!("127.0.0.1:{}", &port).parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    // let mutex = window_handle.lock().await;
    refresh_task.abort();
    // manage_input(*mutex, false);
    Ok(())
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;
    use num_traits::ToPrimitive;

    use crate::{magic::Item, http::{GetItemsResponse, ItemModel}};

    #[tokio::test]
    async fn test_items() {
        let items = Item::iter()
            .map(|i| ItemModel {
                id: i.to_u8().unwrap(),
                name: i.to_string(),
            })
            .collect::<Vec<_>>();
        let response = GetItemsResponse { items };
        let str = serde_json::to_string_pretty(&response).unwrap();
        println!("{}", str);
    }
}