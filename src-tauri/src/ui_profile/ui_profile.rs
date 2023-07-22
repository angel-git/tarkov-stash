use crate::spt_profile::spt_profile::{Location, TarkovProfile};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UIProfile {
    pub name: String,
    pub items: Vec<Item>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Item {
    pub id: String,
    pub tpl: String,
    pub x: u16,
    pub y: u16,
    pub amount: u32,
    pub isStockable: bool,
}

pub fn convert_profile_to_ui(tarkov_profile: TarkovProfile) -> UIProfile {
    let stash = &tarkov_profile.characters.pmc.Inventory.stash;

    let mut items: Vec<Item> = Vec::new();

    for item in tarkov_profile.characters.pmc.Inventory.items.iter() {
        println!("current item: {}", item._id.to_string());

        let parent_id = item.parentId.as_ref();
        let item_slot = item.slotId.as_ref();
        let location = item.location.as_ref();
        let udp_option = item.upd.as_ref();
        if !(parent_id.is_some() && parent_id.unwrap().eq(stash)) {
            continue;
        };
        if !(item_slot.is_some() && item_slot.unwrap().eq("hideout")) {
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
            let udp = udp_option.unwrap();
            if udp.StackObjectsCount.is_some() {
                amount = udp.StackObjectsCount.unwrap();
                is_stockable = true;
            }
        }

        let i = Item {
            id: item._id.to_string(),
            tpl: item._tpl.to_string(),
            x: location_in_stash.x,
            y: location_in_stash.y,
            amount,
            isStockable: is_stockable,
        };
        items.push(i)
    }

    UIProfile {
        name: tarkov_profile.characters.pmc.Info.Nickname,
        items,
    }
}
