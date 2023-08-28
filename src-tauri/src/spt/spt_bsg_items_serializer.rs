use serde::{Deserialize, Serialize};
use serde_json::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub _id: String,
    pub _props: Props,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Props {
    #[serde(rename = "Width")]
    pub width: u16,
    #[serde(rename = "Height")]
    pub height: u16,
    #[serde(rename = "SizeReduceRight")]
    pub size_reduced_right: Option<u16>,
    #[serde(rename = "ExtraSizeForceAdd")]
    pub extra_size_force_add: bool,
    #[serde(rename = "ExtraSizeLeft")]
    pub extra_size_left: u16,
    #[serde(rename = "ExtraSizeRight")]
    pub extra_size_right: u16,
    #[serde(rename = "ExtraSizeUp")]
    pub extra_size_up: u16,
    #[serde(rename = "ExtraSizeDown")]
    pub extra_size_down: u16,
    #[serde(rename = "StackMaxSize")]
    pub stack_max_size: u32,
    #[serde(rename = "MaxResource")]
    pub max_resource: Option<u16>,
    #[serde(rename = "MaxHpResource")]
    pub max_hp_resource: Option<u16>,
    #[serde(rename = "MaximumNumberOfUsage")]
    pub maximum_number_of_usages: Option<u16>,
    #[serde(rename = "MaxDurability")]
    pub max_durability: Option<u16>,
    #[serde(rename = "BackgroundColor")]
    pub background_color: String,
    #[serde(rename = "Foldable")]
    pub foldable: Option<bool>,
    #[serde(rename = "FoldedSlot")]
    pub folded_slot: Option<String>,
    #[serde(rename = "Grids")]
    pub grids: Option<Vec<Grids>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Grids {
    pub _name: String,
    pub _props: GridsProps,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GridsProps {
    #[serde(rename = "cellsH")]
    pub cells_h: u16,
    #[serde(rename = "cellsV")]
    pub cells_v: u16,
}

pub fn load_item(item_json: &str) -> Result<Item, Error> {
    serde_json::from_str(item_json)
}
