use std::collections::HashMap;

use crate::spt;
use crate::spt::spt_bsg_items_serializer::load_item;
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
    #[serde(rename = "bsgItems")]
    pub bsg_items: HashMap<String, BsgItem>,
    #[serde(rename = "sptVersion")]
    pub spt_version: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Item {
    pub id: String,
    pub tpl: String,
    pub x: u16,
    pub y: u16,
    #[serde(rename = "sizeX")]
    pub size_x: u16,
    #[serde(rename = "sizeY")]
    pub size_y: u16,
    pub amount: u32,
    #[serde(rename = "stackMaxSize")]
    pub stack_max_size: u32,
    #[serde(rename = "isStockable")]
    pub is_stockable: bool,
    #[serde(rename = "isFir")]
    pub is_fir: bool,
    #[serde(rename = "rotation")]
    pub r: String,
    pub resource: Option<u16>,
    #[serde(rename = "maxResource")]
    pub max_resource: Option<u16>,
    #[serde(rename = "backgroundColor")]
    pub background_color: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BsgItem {
    pub id: String,
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
}

pub fn convert_profile_to_ui(
    tarkov_profile: TarkovProfile,
    bsg_items: &str,
    locale: &str,
) -> UIProfile {
    let bsg_items_root: HashMap<String, Value> = serde_json::from_str(bsg_items).unwrap();
    let locale_root: HashMap<String, Value> = serde_json::from_str(locale).unwrap();
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

        let bsg_item = get_bsg_item(item, &bsg_items_root);

        let mut amount = 1;
        let mut spawned_in_session = false;
        let mut resource = None;

        let mut max_resource = None
            .or(bsg_item._props.max_resource)
            .or(bsg_item._props.max_hp_resource)
            .or(bsg_item._props.maximum_number_of_usages);

        if udp_option.is_some() {
            if let Some(udp) = udp_option {
                if udp.stack_objects_count.is_some() {
                    amount = udp.stack_objects_count.unwrap();
                }
                if udp.spawned_in_session.is_some() {
                    spawned_in_session = udp.spawned_in_session.unwrap();
                }
                if udp.food_drink.is_some() {
                    resource = Some(udp.food_drink.as_ref().unwrap().hp_percent);
                }
                if udp.med_kit.is_some() {
                    resource = Some(udp.med_kit.as_ref().unwrap().hp_resource);
                }
                if udp.resource.is_some() {
                    resource = Some(udp.resource.as_ref().unwrap().value);
                }
                if udp.repairable.is_some() {
                    resource = Some(udp.repairable.as_ref().unwrap().durability);
                    max_resource = Some(udp.repairable.as_ref().unwrap().max_durability);
                }
                if udp.repairable.is_some() {
                    resource = Some(udp.repairable.as_ref().unwrap().durability);
                    max_resource = Some(udp.repairable.as_ref().unwrap().max_durability);
                }
                if udp.key.is_some() {
                    resource = Some(
                        bsg_item._props.maximum_number_of_usages.unwrap()
                            - udp.key.as_ref().unwrap().number_of_usages,
                    );
                }
            }
        }

        let (size_x, size_y) = calculate_item_size(
            item,
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
        );

        let stack_max_size = bsg_item._props.stack_max_size;
        let background_color = bsg_item._props.background_color;

        let i = Item {
            id: item._id.to_string(),
            tpl: item._tpl.to_string(),
            x: location_in_stash.x,
            y: location_in_stash.y,
            size_x,
            size_y,
            amount,
            is_stockable: stack_max_size != 1,
            stack_max_size,
            is_fir: spawned_in_session,
            r: location_in_stash.r.to_string(),
            max_resource,
            resource,
            background_color,
        };
        items.push(i)
    }

    let mut bsg_items: HashMap<String, BsgItem> = HashMap::new();
    bsg_items_root.keys().for_each(|k| {
        let item = bsg_items_root.get(k).unwrap();
        let id = item.get("_id").unwrap().as_str().unwrap();
        if let Some(props) = item.get("_props") {
            if let Some(short_name) = props.get("ShortName") {
                let maybe_name = locale_root.get(format!("{} Name", id).as_str());
                let maybe_short_name = locale_root.get(format!("{} ShortName", id).as_str());
                let name = maybe_name
                    .and_then(|v| v.as_str())
                    .unwrap_or_else(|| short_name.as_str().unwrap());
                let short_name = maybe_short_name
                    .and_then(|v| v.as_str())
                    .unwrap_or_else(|| short_name.as_str().unwrap());

                bsg_items.insert(
                    id.to_string(),
                    BsgItem {
                        id: id.to_string(),
                        name: name.to_string(),
                        short_name: short_name.to_string(),
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
        spt_version: None,
    }
}

fn calculate_item_size(
    item: &spt::spt_profile_serializer::Item,
    items: &[spt::spt_profile_serializer::Item],
    bsg_items_root: &HashMap<String, Value>,
) -> (u16, u16) {
    let all_children = find_all_items_from_parent(item._id.as_str(), items);
    // copied from InventoryHelper.getSizeByInventoryItemHash
    let parent_item = bsg_items_root.get(item._tpl.as_str()).unwrap();
    let parsed_parent_item = load_item(parent_item.to_string().as_str()).unwrap();

    let out_x = parsed_parent_item._props.width;
    let out_y = parsed_parent_item._props.height;
    let mut size_up = 0;
    let mut size_down = 0;
    let mut size_left = 0;
    let mut size_right = 0;
    let mut forced_up = 0;
    let mut forced_down = 0;
    let mut forced_left = 0;
    let mut forced_right = 0;

    all_children.iter().for_each(|c| {
        let bsg_item = bsg_items_root.get(c).unwrap();
        let parsed_bsg_item = load_item(bsg_item.to_string().as_str()).unwrap();

        if parsed_bsg_item._props.extra_size_force_add {
            forced_up += parsed_bsg_item._props.extra_size_up;
            forced_down += parsed_bsg_item._props.extra_size_down;
            forced_left += parsed_bsg_item._props.extra_size_left;
            forced_right += parsed_bsg_item._props.extra_size_right;
        } else {
            size_up = if size_up < parsed_bsg_item._props.extra_size_up {
                parsed_bsg_item._props.extra_size_up
            } else {
                size_up
            };
            size_down = if size_down < parsed_bsg_item._props.extra_size_down {
                parsed_bsg_item._props.extra_size_down
            } else {
                size_down
            };
            size_left = if size_left < parsed_bsg_item._props.extra_size_left {
                parsed_bsg_item._props.extra_size_left
            } else {
                size_left
            };
            size_right = if size_right < parsed_bsg_item._props.extra_size_right {
                parsed_bsg_item._props.extra_size_right
            } else {
                size_right
            };
        }
    });

    let size_x = out_x + size_left + size_right + forced_left + forced_right;
    let size_y = out_y + size_up + size_down + forced_up + forced_down;
    (size_x, size_y)
}

fn find_all_items_from_parent(
    parent_id: &str,
    items: &[spt::spt_profile_serializer::Item],
) -> Vec<String> {
    let mut result = Vec::new();

    for i in items {
        if i.parent_id.is_some() && i.parent_id.as_ref().unwrap() == parent_id {
            if i.slot_id.is_some() && i.slot_id.as_ref().unwrap().starts_with("mod_") {
                result.push(i._tpl.to_string());
            }

            let sub_items = find_all_items_from_parent(&i._id, items);
            result.extend(sub_items);
        }
    }

    result
}

fn get_bsg_item(
    item: &spt::spt_profile_serializer::Item,
    bsg_items_root: &HashMap<String, Value>,
) -> spt::spt_bsg_items_serializer::Item {
    let parent_item = bsg_items_root.get(item._tpl.as_str()).unwrap();
    load_item(parent_item.to_string().as_str()).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::spt::spt_profile_serializer::{load_profile, Item};
    use crate::ui_profile::ui_profile_serializer::calculate_item_size;
    use serde_json::Value;
    use std::collections::HashMap;

    #[test]
    fn should_calculate_size_with_attachments() {
        let item = Item {
            _id: "6c116ae6e3e795d2e508a5f0".to_string(),
            _tpl: "5926bb2186f7744b1c6c6e60".to_string(),
            parent_id: None,
            location: None,
            slot_id: None,
            upd: None,
        };

        let bsg_items_root: HashMap<String, Value> = serde_json::from_str(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/Aki_Data/Server/database/templates/items.json"
            ))
            .as_ref(),
        )
        .unwrap();

        let tarkov_profile = load_profile(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/user/profiles/af01e654f9af416ee4684a2c.json"
            ))
            .as_ref(),
        )
        .unwrap();

        // mp5 with large magazine and silencer
        let (size_x, size_y) = calculate_item_size(
            &item,
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
        );
        assert_eq!(size_x, 3);
        assert_eq!(size_y, 2);
    }
}
