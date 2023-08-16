use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::spt::spt_profile_serializer::{Location, TarkovProfile};

#[derive(Deserialize, Serialize, Debug)]
pub struct UIProfile {
    pub name: String,
    #[serde(rename = "sizeX")]
    pub size_x: u16,
    #[serde(rename = "sizeY")]
    pub size_y: u16,
    pub items: Vec<Item>,
    #[serde(rename = "bsgItem")]
    pub bsg_items: HashMap<String, BsgItem>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Item {
    pub id: String,
    pub tpl: String,
    pub x: u16,
    pub y: u16,
    pub amount: u32,
    #[serde(rename = "isStockable")]
    pub is_stockable: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BsgItem {
    pub id: String,
    pub name: String,
}

pub fn convert_profile_to_ui(tarkov_profile: TarkovProfile, bsg_items: &str) -> UIProfile {
    let stash = &tarkov_profile.characters.pmc.inventory.stash;
    let stash_bonuses = &tarkov_profile
        .characters
        .pmc
        .bonuses
        .iter()
        .filter(|b| b.t.eq("StashSize"))
        .count();
    let stash_size_y = if stash_bonuses <= &1 {
        28
    } else if stash_bonuses == &2 {
        38
    } else if stash_bonuses == &3 {
        48
    } else {
        68
    };

    let mut items: Vec<Item> = Vec::new();

    for item in tarkov_profile.characters.pmc.inventory.items.iter() {
        let parent_id = item.parent_id.as_ref();
        let item_slot = item.slot_id.as_ref();
        let location = item.location.as_ref();
        let udp_option = item.upd.as_ref();
        if parent_id.is_none() || parent_id.unwrap() != stash {
            continue;
        };
        if item_slot.is_none() || item_slot.unwrap() != "hideout" {
            continue;
        };
        let location_in_stash = if let Location::LocationInStash(xy) = location.unwrap() {
            xy
        } else {
            panic!("oh no, wrong item: {}", item._id);
        };

        let mut amount = 1;
        let mut is_stockable = false;

        if udp_option.is_some() {
            if let Some(udp) = udp_option {
                if udp.stack_objects_count.is_some() {
                    amount = udp.stack_objects_count.unwrap();
                    is_stockable = true;
                }
            }
        }

        let i = Item {
            id: item._id.to_string(),
            tpl: item._tpl.to_string(),
            x: location_in_stash.x,
            y: location_in_stash.y,
            amount,
            is_stockable,
        };
        items.push(i)
    }

    let bsg_items_root: HashMap<String, Value> = serde_json::from_str(bsg_items).unwrap();
    let mut bsg_items: HashMap<String, BsgItem> = HashMap::new();
    bsg_items_root.keys().for_each(|k| {
        let item = bsg_items_root.get(k).unwrap();
        let id = item.get("_id").unwrap().as_str().unwrap();
        if let Some(props) = item.get("_props") {
            if let Some(name) = props.get("ShortName") {
                bsg_items.insert(
                    id.to_string(),
                    BsgItem {
                        id: id.to_string(),
                        name: name.to_string(),
                    },
                );
            }
        }
    });

    UIProfile {
        name: tarkov_profile.characters.pmc.info.nickname,
        size_x: 10,
        size_y: stash_size_y,
        items,
        bsg_items,
    }
}
