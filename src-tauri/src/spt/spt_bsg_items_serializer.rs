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
}

pub fn load_item(item_json: &str) -> Result<Item, Error> {
    serde_json::from_str(item_json)
}
