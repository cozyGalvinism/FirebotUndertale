use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub addresses: AddressesConfig
}

#[derive(Deserialize, Debug)]
pub struct AddressesConfig {
    pub max_health: String,
    pub health: String,
    pub kill_area: String,
    pub gold: String,
    pub speed: String,
    pub equipped_weapon: String,
    pub equipped_armor: String,
    pub encounter_counter: String,
    pub kills_ruins: String,
    pub kills_snowdin: String,
    pub kills_waterfall: String,
    pub kills_hotland: String,
    pub inventory_slot_1: String,
    pub inventory_slot_2: String,
    pub inventory_slot_3: String,
    pub inventory_slot_4: String,
    pub inventory_slot_5: String,
    pub inventory_slot_6: String,
    pub inventory_slot_7: String,
    pub inventory_slot_8: String,
}

impl AppConfig {
    pub fn new() -> Self {
        let config = config::Config::builder()
            .add_source(config::File::with_name("config.toml"))
            .build()
            .unwrap();
        config.try_deserialize().unwrap()
    }
}

pub fn parse_offsets_from_ce_string(string: &str) -> Vec<usize> {
    let replaced = string
        .replace("BASE+", "")
        .replace("\"Undertale.exe\"", "")
        .replace(" \"UNDERTALE.exe\"", "")
        .replace('[', "")
        .replace(']', "")
        .replace('+', ",");
    let mut offsets: Vec<usize> = Vec::new();
    for offset in replaced.split(',') {
        offsets.push(usize::from_str_radix(offset, 16).unwrap());
    }

    offsets
}