use std::{process, sync::Arc, time::Duration, ffi::CString};

use axum::{Router, response::IntoResponse, routing::{post, get}, Extension, Json, http::StatusCode};
use http::{SetHealthBody, ItemModel, GetItemsResponse, FillInventoryBody, DisableInputBody, GetInventoryBody, GetInventoryResponse, GetHealthResponse, GetMaxHealthResponse, GetGoldResponse, SetGoldBody, SetEncounterBody, GetSpeedResponse, SetSpeedBody};
use magic::*;
use serde_json::json;
use strum::IntoEnumIterator;
use sysinfo::{System, SystemExt, ProcessExt, PidExt};
use tokio::sync::Mutex;
use vmemory::ProcessMemory;
use num_traits::{ToPrimitive, FromPrimitive};

mod magic;
mod http;

#[derive(Clone)]
struct UndertaleGame {
    pub process: Arc<Mutex<ProcessMemory>>,
    pub window_handle: Arc<Mutex<usize>>,
    pub ready: Arc<Mutex<bool>>,
}

fn health_address(process_memory: &ProcessMemory) -> usize {
    CURRENT_HEALTH_OFFSETS.fetch_address(process_memory)
}

fn max_health_address(process_memory: &ProcessMemory) -> usize {
    MAX_HEALTH_OFFSETS.fetch_address(process_memory)
}

fn inventory_address(process_memory: &ProcessMemory, slot: usize) -> usize {
    INVENTORY_OFFSETS[slot].fetch_address(process_memory)
}

fn gold_address(process_memory: &ProcessMemory) -> usize {
    GOLD_OFFSETS.fetch_address(process_memory)
}

fn encounter_address(process_memory: &ProcessMemory) -> usize {
    ENCOUNTER_COUNTER_OFFSETS.fetch_address(process_memory)
}

fn speed_address(process: &ProcessMemory) -> usize {
    SPEED_OFFSETS.fetch_address(process)
}

fn equipped_weapon_address(process: &ProcessMemory) -> usize {
    EQUIPPED_WEAPON_OFFSETS.fetch_address(process)
}

fn equipped_armor_address(process: &ProcessMemory) -> usize {
    EQUIPPED_ARMOR_OFFSETS.fetch_address(process)
}

fn get_health(process: &ProcessMemory) -> f64 {
    let health_address = health_address(process);
    f64::from_le_bytes(process.read_memory(health_address, 8, false).try_into().unwrap())
}

fn set_health(process: &ProcessMemory, health: f64) {
    let health_address = health_address(process);
    let health_bytes = health.to_le_bytes().to_vec();
    process.write_memory(health_address, &health_bytes, false);
}

fn get_max_health(process: &ProcessMemory) -> f64 {
    let max_health_address = max_health_address(process);
    f64::from_le_bytes(process.read_memory(max_health_address, 8, false).try_into().unwrap())
}

fn get_speed(process: &ProcessMemory) -> f64 {
    let speed_address = speed_address(process);
    f64::from_le_bytes(process.read_memory(speed_address, 8, false).try_into().unwrap())
}

fn set_speed(process: &ProcessMemory, speed: f64) {
    let speed_address = speed_address(process);
    let speed_bytes = speed.to_le_bytes().to_vec();
    process.write_memory(speed_address, &speed_bytes, true);
}

fn get_gold(process: &ProcessMemory) -> f64 {
    let gold_address = gold_address(process);
    f64::from_le_bytes(process.read_memory(gold_address, 8, false).try_into().unwrap())
}

fn set_gold(process: &ProcessMemory, gold: f64) {
    let gold_address = gold_address(process);
    let gold_bytes = gold.to_le_bytes().to_vec();
    process.write_memory(gold_address, &gold_bytes, true);
}

fn get_equipped_weapon(process: &ProcessMemory) -> f64 {
    let equipped_weapon_address = equipped_weapon_address(process);
    f64::from_le_bytes(process.read_memory(equipped_weapon_address, 8, false).try_into().unwrap())
}

fn set_equipped_weapon(process: &ProcessMemory, weapon: f64) {
    let equipped_weapon_address = equipped_weapon_address(process);
    let weapon_bytes = weapon.to_le_bytes().to_vec();
    process.write_memory(equipped_weapon_address, &weapon_bytes, true);
}

fn get_equipped_armor(process: &ProcessMemory) -> f64 {
    let equipped_armor_address = equipped_armor_address(process);
    f64::from_le_bytes(process.read_memory(equipped_armor_address, 8, false).try_into().unwrap())
}

fn set_equipped_armor(process: &ProcessMemory, armor: f64) {
    let equipped_armor_address = equipped_armor_address(process);
    let armor_bytes = armor.to_le_bytes().to_vec();
    process.write_memory(equipped_armor_address, &armor_bytes, true);
}

fn get_inventory_item(process: &ProcessMemory, slot: usize) -> Option<Item> {
    if slot >= INVENTORY_OFFSETS.len() {
        return None;
    }
    let inventory_address = inventory_address(process, slot);
    let item_bytes = process.read_memory(inventory_address, 4, false);

    num_traits::FromPrimitive::from_f64(f64::from_le_bytes(item_bytes.try_into().unwrap()))
}

fn set_inventory_item(process: &ProcessMemory, slot: usize, item: Item) {
    if slot >= INVENTORY_OFFSETS.len() {
        return;
    }
    let inventory_address = inventory_address(process, slot);
    let item_bytes = num_traits::ToPrimitive::to_f64(&item).unwrap().to_le_bytes().to_vec();
    process.write_memory(inventory_address, &item_bytes, true);
}

fn fill_inventory_with(process: &ProcessMemory, item: Item, overwrite_important_items: bool, only_empty_slots: bool) {
    for slot in 0..INVENTORY_OFFSETS.len() {
        let inventory_item = get_inventory_item(process, slot);
        if let Some(inventory_item) = inventory_item {
            if overwrite_important_items && inventory_item.is_important_item() {
                continue;
            }

            if only_empty_slots && !matches!(inventory_item, Item::Empty) {
                continue;
            }
            set_inventory_item(process, slot, item);
        }
    }
}

fn set_encounter_counter(process: &ProcessMemory, counter: f64) {
    let encounter_address = encounter_address(process);
    let counter_bytes = counter.to_le_bytes().to_vec();
    process.write_memory(encounter_address, &counter_bytes, true);
}

fn manage_input(handle: usize, disable: bool) {
    unsafe {
        let _ = winapi::um::winuser::EnableWindow(handle as winapi::shared::windef::HWND, !disable as i32);
    };
}

async fn http_set_health(
    process_extension: Extension<Arc<UndertaleGame>>,
    Json(set_health_body): Json<SetHealthBody>
) -> impl IntoResponse {
    let process = process_extension.process.lock().await;
    set_health(&process, set_health_body.health);

    (StatusCode::OK, Json(json!({"status": "ok"}))).into_response()
}

async fn http_get_health(process_extension: Extension<Arc<UndertaleGame>>) -> impl IntoResponse {
    let process = process_extension.process.lock().await;
    let health = get_health(&process);

    (StatusCode::OK, Json(GetHealthResponse { health })).into_response()
}

async fn http_get_max_health(process_extension: Extension<Arc<UndertaleGame>>) -> impl IntoResponse {
    let process = process_extension.process.lock().await;
    let max_health = get_max_health(&process);

    (StatusCode::OK, Json(GetMaxHealthResponse { max_health })).into_response()
}

async fn http_get_gold(process_extension: Extension<Arc<UndertaleGame>>) -> impl IntoResponse {
    let process = process_extension.process.lock().await;
    let gold = get_gold(&process);

    (StatusCode::OK, Json(GetGoldResponse { gold })).into_response()
}

async fn http_set_gold(
    process_extension: Extension<Arc<UndertaleGame>>,
    Json(set_gold_body): Json<SetGoldBody>
) -> impl IntoResponse {
    let process = process_extension.process.lock().await;
    set_gold(&process, set_gold_body.gold);

    (StatusCode::OK, Json(json!({"status": "ok"}))).into_response()
}

async fn http_fill_inventory(
    process_extension: Extension<Arc<UndertaleGame>>,
    Json(fill_inventory_body): Json<FillInventoryBody>
) -> impl IntoResponse {
    let process_memory = process_extension.process.lock().await;
    fill_inventory_with(&process_memory, Item::from_u8(fill_inventory_body.item).unwrap(), fill_inventory_body.overwrite_important_items, fill_inventory_body.only_empty_slots);

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
    let inventory_item = get_inventory_item(&process_memory, get_inventory_body.slot);
    let inventory_item = match inventory_item {
        Some(inventory_item) => inventory_item.to_u8().unwrap(),
        None => 0_u8,
    };
    (StatusCode::OK, Json(GetInventoryResponse { item: inventory_item })).into_response()
}

async fn http_disable_input(process_extension: Extension<Arc<UndertaleGame>>, Json(disable_body): Json<DisableInputBody>) -> impl IntoResponse {
    {
        let ready_state = process_extension.ready.lock().await;
        if !*ready_state {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"status": "not ready"}))).into_response();
        }
    }
    let window_handle = process_extension.window_handle.lock().await;
    manage_input(*window_handle, disable_body.disable);

    (StatusCode::OK, Json(json!({"status": "ok"}))).into_response()
}

async fn http_set_encounter(
    process_extension: Extension<Arc<UndertaleGame>>,
    Json(set_encounter_body): Json<SetEncounterBody>
) -> impl IntoResponse {
    let process = process_extension.process.lock().await;
    set_encounter_counter(&process, set_encounter_body.counter);

    (StatusCode::OK, Json(json!({"status": "ok"}))).into_response()
}

async fn http_get_speed(process_extension: Extension<Arc<UndertaleGame>>) -> impl IntoResponse {
    let process = process_extension.process.lock().await;
    let speed = get_speed(&process);

    (StatusCode::OK, Json(GetSpeedResponse { speed })).into_response()
}

async fn http_set_speed(
    process_extension: Extension<Arc<UndertaleGame>>,
    Json(set_speed_body): Json<SetSpeedBody>
) -> impl IntoResponse {
    let process = process_extension.process.lock().await;
    set_speed(&process, set_speed_body.speed);

    (StatusCode::OK, Json(json!({"status": "ok"}))).into_response()
}

fn get_main_window_handle(process: u32) -> Vec<usize> {
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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let port = std::env::var("PORT").unwrap_or_else(|_| "1337".to_string());

    let mut s = System::new_all();

    let mut undertale = s.processes_by_name("UNDERTALE").next();
    let window_handle: Arc<Mutex<usize>>;
    let process: Arc<Mutex<ProcessMemory>>;
    
    if let Some(undertale) = undertale {
        tracing::info!("Attaching to Undertale process");
        let undertale_pid: u32 = undertale.pid().as_u32();
        process = Arc::new(Mutex::new(ProcessMemory::attach_process(undertale_pid).unwrap()));
        let handles = get_main_window_handle(undertale_pid);
        if handles.is_empty() {
            return Err(anyhow::anyhow!("No main window found"));
        }
        window_handle = Arc::new(Mutex::new(handles.into_iter().next().expect("Could not find main window")));
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
        let handles = get_main_window_handle(undertale_pid);
        if handles.is_empty() {
            return Err(anyhow::anyhow!("No main window found"));
        }
        window_handle = Arc::new(Mutex::new(handles.into_iter().next().expect("Could not find main window")));
    }

    let game = Arc::new(UndertaleGame {
        process,
        window_handle: window_handle.clone(),
        ready: Arc::new(Mutex::new(true)),
    });
    // let game_clone = game.clone();

    /* let refresh_task = tokio::task::spawn(async move {
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
                let window_handles = get_main_window_handle(undertale_pid);
                if window_handles.is_empty() {
                    continue;
                }
                let window_handle = window_handles.into_iter().next();
                let mut existing_process = game.process.lock().await;
                let mut existing_window_handle = game.window_handle.lock().await;
                let _ = std::mem::replace(&mut *existing_process, process);
                let _ = std::mem::replace(&mut *existing_window_handle, window_handle.expect("Could not find main window"));
                let mut ready = game.ready.lock().await;
                *ready = true;
            }

            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }); */

    let app = Router::new()
        .route("/setHealth", post(http_set_health))
        .route("/getHealth", get(http_get_health))
        .route("/getMaxHealth", get(http_get_max_health))
        .route("/getGold", get(http_get_gold))
        .route("/setGold", post(http_set_gold))
        .route("/getItems", get(http_get_items))
        .route("/fillInventory", post(http_fill_inventory))
        .route("/getInventory", post(http_get_inventory_at_slot))
        // .route("/disableInput", post(http_disable_input))
        .route("/setEncounter", post(http_set_encounter))
        .route("/getSpeed", get(http_get_speed))
        .route("/setSpeed", post(http_set_speed))
        .layer(Extension(game));
    
    tracing::debug!("Serving on http://localhost:{}", &port);
    axum::Server::bind(&format!("127.0.0.1:{}", &port).parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    // let mutex = window_handle.lock().await;
    // refresh_task.abort();
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