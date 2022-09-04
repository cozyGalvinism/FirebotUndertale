// Current Health: double [[[["Undertale.exe"+00408950]+44]+10]+13C]+4B0
// Max Health: double [[[["Undertale.exe"+00408950]+44]+10]+13C]+4A0
// Currently in fight?: double [[[["Undertale.exe"+00408950]+44]+10]+1CC]+F0
// Experience: double [[[[["Undertale.exe"+003F9F44]+0]+44]+10]+364]+3F0
// Gold: double [[[[["Undertale.exe"+003F9F44]+0]+44]+10]+364]+400
// Love: double [[[[["Undertale.exe"+003F9F44]+0]+44]+10]+364]+3E0
// Speed: double [[[["Undertale.exe"+00408950]+44]+10]+BE0]+2B0
// Inventory:
// Slot 1: [[[[[[["Undertale.exe"+0040894C]+44]+10]+B20]+20]+24]+14]+0
// Slot 2: [[[[[[["Undertale.exe"+0040894C]+44]+10]+B20]+20]+24]+14]+10
// Slot 3: [[[[[[["Undertale.exe"+0040894C]+44]+10]+B20]+20]+24]+14]+20
// Slot 4: [[[[[[["Undertale.exe"+0040894C]+44]+10]+B20]+20]+24]+14]+30
// Slot 5: [[[[[[["Undertale.exe"+0040894C]+44]+10]+B20]+20]+24]+14]+40
// Slot 6: [[[[[[["Undertale.exe"+0040894C]+44]+10]+B20]+20]+24]+14]+50
// Slot 7: [[[[[[["Undertale.exe"+0040894C]+44]+10]+B20]+20]+24]+14]+60
// Slot 8: [[[[[[["Undertale.exe"+0040894C]+44]+10]+B20]+20]+24]+14]+70

use num_derive::{Float, FromPrimitive, ToPrimitive};
use num_traits::{ToPrimitive, FromPrimitive};
use serde_repr::{Serialize_repr, Deserialize_repr};
use strum::{EnumIter, Display};
use vmemory::ProcessMemory;

// Stats
pub const CURRENT_HEALTH_OFFSETS: [usize; 5] = [0x00408950, 0x44, 0x10, 0x13c, 0x4b0];
pub const MAX_HEALTH_OFFSETS: [usize; 5] = [0x00408950, 0x44, 0x10, 0x13c, 0x4a0];
pub const EXPERIENCE_OFFSETS: [usize; 6] = [0x003f9f44, 0x0, 0x44, 0x10, 0x364, 0x3f0];
pub const GOLD_OFFSETS: [usize; 6] = [0x003f9f44, 0x0, 0x44, 0x10, 0x364, 0x400];
pub const LOVE_OFFSETS: [usize; 6] = [0x003f9f44, 0x0, 0x44, 0x10, 0x364, 0x3e0];
pub const ENCOUNTER_COUNTER_OFFSETS: [usize; 5] = [0x00408950, 0x44, 0x10, 0x13c, 0x10];
pub const SPEED_OFFSETS: [usize; 5] = [0x00408950, 0x44, 0x10, 0xbe0, 0x2b0];

// Inventory
pub const INVENTORY_SLOT_1_OFFSETS: [usize; 8] =
    [0x0040894c, 0x44, 0x10, 0xB20, 0x20, 0x24, 0x14, 0x00];
pub const INVENTORY_SLOT_2_OFFSETS: [usize; 8] =
    [0x0040894c, 0x44, 0x10, 0xB20, 0x20, 0x24, 0x14, 0x10];
pub const INVENTORY_SLOT_3_OFFSETS: [usize; 8] =
    [0x0040894c, 0x44, 0x10, 0xB20, 0x20, 0x24, 0x14, 0x20];
pub const INVENTORY_SLOT_4_OFFSETS: [usize; 8] =
    [0x0040894c, 0x44, 0x10, 0xB20, 0x20, 0x24, 0x14, 0x30];
pub const INVENTORY_SLOT_5_OFFSETS: [usize; 8] =
    [0x0040894c, 0x44, 0x10, 0xB20, 0x20, 0x24, 0x14, 0x40];
pub const INVENTORY_SLOT_6_OFFSETS: [usize; 8] =
    [0x0040894c, 0x44, 0x10, 0xB20, 0x20, 0x24, 0x14, 0x50];
pub const INVENTORY_SLOT_7_OFFSETS: [usize; 8] =
    [0x0040894c, 0x44, 0x10, 0xB20, 0x20, 0x24, 0x14, 0x60];
pub const INVENTORY_SLOT_8_OFFSETS: [usize; 8] =
    [0x0040894c, 0x44, 0x10, 0xB20, 0x20, 0x24, 0x14, 0x70];
pub const EQUIPPED_WEAPON_OFFSETS: [usize; 7] = 
    [0x003f9f44, 0x6c, 0x164, 0x44, 0x10, 0x16c, 0x4c0];
pub const EQUIPPED_ARMOR_OFFSETS: [usize; 8] =
    [0x003f9f44, 0x6c, 0x164, 0x160, 0x44, 0x10, 0x4, 0x550];

pub const INVENTORY_OFFSETS: [&[usize; 8]; 8] = [
    &INVENTORY_SLOT_1_OFFSETS,
    &INVENTORY_SLOT_2_OFFSETS,
    &INVENTORY_SLOT_3_OFFSETS,
    &INVENTORY_SLOT_4_OFFSETS,
    &INVENTORY_SLOT_5_OFFSETS,
    &INVENTORY_SLOT_6_OFFSETS,
    &INVENTORY_SLOT_7_OFFSETS,
    &INVENTORY_SLOT_8_OFFSETS,
];

fn fetch_pointer_address(process: &ProcessMemory, offsets: &[usize]) -> usize {
    let mut previous_pointer =
        u32::from_le_bytes(process.read_memory(offsets[0], 4, true).try_into().unwrap());
    (1..offsets.len() - 1).for_each(|i| {
        previous_pointer = u32::from_le_bytes(
            process
                .read_memory(previous_pointer as usize + offsets[i], 4, false)
                .try_into()
                .unwrap(),
        );
    });
    previous_pointer as usize + offsets[offsets.len() - 1]
}

pub trait FetchAddress {
    fn fetch_address(&self, process: &ProcessMemory) -> usize;
}

impl FetchAddress for [usize] {
    fn fetch_address(&self, process: &ProcessMemory) -> usize {
        fetch_pointer_address(process, self)
    }
}

#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Debug, EnumIter, Display, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Item {
    Empty = 0,
    MnstrCndy = 1,
    CroqtRoll = 2,
    Stick = 3,
    Bandage = 4,
    RockCandy = 5,
    PmknRings = 6,
    SpdrDonut = 7,
    Onion = 8,
    GhstFruit = 9,
    SpdrCider = 10,
    Pie = 11,
    Ribbon = 12,
    ToyKnife = 13,
    Glove = 14,
    Bandanna = 15,
    SnowPiece = 16,
    NiceCream = 17,
    IceCream = 18,
    Bisicle = 19,
    Popsicle = 20,
    Cbun = 21,
    TemFlakes = 22,
    Quiche = 23,
    Tutu = 24,
    Shoes = 25,
    PunchCard = 26,
    Dog = 27,
    DogSalad = 28,
    DResidue1 = 29,
    DResidue2 = 30,
    DResidue3 = 31,
    DResidue4 = 32,
    DResidue5 = 33,
    DResidue6 = 34,
    AstrFood = 35,
    INoodles = 36,
    CrabApple = 37,
    HotDog = 38,
    HotCat = 39,
    GBurger = 40,
    SeaTea = 41,
    Starfait = 42,
    LHero = 43,
    Glasses = 44,
    Notebook = 45,
    Apron = 46,
    BurntPan = 47,
    CowboyHat = 48,
    EmptyGun = 49,
    HLocket = 50,
    WDagger = 51,
    RealKnife = 52,
    TheLocket = 53,
    BadMemory = 54,
    LastDream = 55,
    Letter = 56,
    LetterX = 57,
    Chips = 58,
    JunkFood = 59,
    Key = 60,
    Steak = 61,
    HushPuppy = 62,
    SnailPie = 63,
    TemArmor = 64,
}

impl Item {
    pub fn is_important_item(&self) -> bool {
        matches!(
            self,
            Item::Stick
                | Item::Bandage
                | Item::Pie
                | Item::Ribbon
                | Item::ToyKnife
                | Item::Glove
                | Item::Bandanna
                | Item::SnowPiece
                | Item::Tutu
                | Item::Shoes
                | Item::Dog
                | Item::Glasses
                | Item::Notebook
                | Item::Apron
                | Item::BurntPan
                | Item::CowboyHat
                | Item::EmptyGun
                | Item::HLocket
                | Item::WDagger
                | Item::RealKnife
                | Item::TheLocket
                | Item::Letter
                | Item::LetterX
                | Item::Key
                | Item::TemArmor
        )
    }
}

impl From<Item> for f64 {
    fn from(item: Item) -> f64 {
        item.to_f64().unwrap()
    }
}

impl TryFrom<f64> for Item {
    type Error = ();

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        FromPrimitive::from_f64(value)
            .ok_or(())
    }
}
