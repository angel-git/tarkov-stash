use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::spt;
use crate::spt::spt_bsg_items_serializer::load_item;
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
    #[serde(rename = "locale")]
    locale: HashMap<String, Value>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Item {
    pub id: String,
    pub tpl: String,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
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
    #[serde(rename = "isContainer")]
    pub is_container: bool,
    #[serde(rename = "gridItems")]
    pub grid_items: Option<Vec<GridItem>>,
    #[serde(rename = "slotItems")]
    pub slot_items: Option<HashSet<SlotItem>>,
}

#[derive(Deserialize, Serialize, Debug, Hash, Eq, PartialEq)]
pub struct SlotItem {
    pub id: String,
    pub tpl: String,
    #[serde(rename = "slotId")]
    pub slot_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GridItem {
    #[serde(rename = "name")]
    pub _name: String,
    #[serde(rename = "cellsH")]
    pub cells_h: u16,
    #[serde(rename = "cellsV")]
    pub cells_v: u16,
    pub items: Vec<Item>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BsgItem {
    pub id: String,
    #[serde(rename = "parentId")]
    pub parent_id: Option<Value>,
    pub width: Option<Value>,
    pub height: Option<Value>,
    #[serde(rename = "hideEntrails")]
    pub hide_entrails: Option<Value>,
    pub unlootable: Option<Value>,
    pub unbuyable: Option<Value>,
    #[serde(rename = "type")]
    pub r#type: Option<Value>,
    #[serde(rename = "backgroundColor")]
    pub background_color: Option<Value>,
    #[serde(rename = "Slots")]
    pub slots: Option<Value>,
    pub ergonomics: Value,
    #[serde(rename = "deviationMax")]
    pub deviation_max: Value,
    #[serde(rename = "deviationCurve")]
    pub deviation_curve: Value,
    #[serde(rename = "sightingRange")]
    pub sighting_range: Value,
    pub recoil: Value,
    #[serde(rename = "recoilForceBack")]
    pub recoil_force_back: Value,
    #[serde(rename = "recoilForceUp")]
    pub recoil_force_up: Value,
    #[serde(rename = "centerOfImpact")]
    pub center_of_impact: Value,
    pub velocity: Value,
    #[serde(rename = "initialSpeed")]
    pub initial_speed: Value,
    #[serde(rename = "ammoAccr")]
    pub ammo_accr: Value,
    #[serde(rename = "accuracy")]
    pub accuracy: Value,
}

pub fn convert_profile_to_ui(
    tarkov_profile: TarkovProfile,
    bsg_items_root: &HashMap<String, Value>,
    locale_root: &HashMap<String, Value>,
) -> Result<UIProfile, String> {
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

    let items: Vec<Item> = parse_items(
        tarkov_profile.characters.pmc.inventory.items,
        bsg_items_root,
        stash.as_str(),
        "hideout",
    )?;

    let mut bsg_items: HashMap<String, BsgItem> = HashMap::new();
    bsg_items_root.keys().for_each(|k| {
        let item = bsg_items_root.get(k).unwrap();
        let id = item.get("_id").unwrap().as_str().unwrap();
        let props = item.get("_props");
        bsg_items.insert(
            id.to_string(),
            BsgItem {
                id: id.to_string(),
                parent_id: item.get("_parent").cloned(),
                width: props.and_then(|p| p.get("Width")).cloned(),
                height: props.and_then(|p| p.get("Height")).cloned(),
                unlootable: props.and_then(|p| p.get("Unlootable")).cloned(),
                unbuyable: props.and_then(|p| p.get("IsUnbuyable")).cloned(),
                hide_entrails: props.and_then(|p| p.get("HideEntrails")).cloned(),
                r#type: item.get("_type").cloned(),
                background_color: props.and_then(|p| p.get("BackgroundColor")).cloned(),
                slots: props.and_then(|p| p.get("Slots")).cloned(),
                ergonomics: props
                    .and_then(|p| p.get("Ergonomics"))
                    .cloned()
                    .unwrap_or(Value::from(0)),
                deviation_max: props
                    .and_then(|p| p.get("DeviationMax"))
                    .cloned()
                    .unwrap_or(Value::from(0)),
                deviation_curve: props
                    .and_then(|p| p.get("DeviationCurve"))
                    .cloned()
                    .unwrap_or(Value::from(0)),
                sighting_range: props
                    .and_then(|p| p.get("SightingRange"))
                    .cloned()
                    .unwrap_or(Value::from(0)),
                recoil: props
                    .and_then(|p| p.get("Recoil"))
                    .cloned()
                    .unwrap_or(Value::from(0)),
                recoil_force_back: props
                    .and_then(|p| p.get("RecoilForceBack"))
                    .cloned()
                    .unwrap_or(Value::from(0)),
                recoil_force_up: props
                    .and_then(|p| p.get("RecoilForceUp"))
                    .cloned()
                    .unwrap_or(Value::from(0)),
                initial_speed: props
                    .and_then(|p| p.get("InitialSpeed"))
                    .cloned()
                    .unwrap_or(Value::from(0)),
                velocity: props
                    .and_then(|p| p.get("Velocity"))
                    .cloned()
                    .unwrap_or(Value::from(0)),
                center_of_impact: props
                    .and_then(|p| p.get("CenterOfImpact"))
                    .cloned()
                    .unwrap_or(Value::from(0)),
                ammo_accr: props
                    .and_then(|p| p.get("ammoAccr"))
                    .cloned()
                    .unwrap_or(Value::from(0)),
                accuracy: props
                    .and_then(|p| p.get("Accuracy"))
                    .cloned()
                    .unwrap_or(Value::from(0)),
            },
        );
    });

    Ok(UIProfile {
        name: tarkov_profile.characters.pmc.info.nickname,
        size_x: 10,
        size_y: stash_size_y,
        items,
        bsg_items,
        spt_version: None,
        locale: locale_root.clone(),
    })
}

fn parse_items(
    profile_items: Vec<spt::spt_profile_serializer::Item>,
    bsg_items_root: &HashMap<String, Value>,
    parent_slot: &str,
    parent_item_slot: &str,
) -> Result<Vec<Item>, String> {
    let mut items: Vec<Item> = Vec::new();

    for item in profile_items.iter() {
        let parent_id = item.parent_id.as_ref();
        let item_slot = item.slot_id.as_ref();
        let location = item.location.as_ref();
        let udp_option = item.upd.as_ref();
        if parent_id.is_none() || parent_id.unwrap() != parent_slot {
            continue;
        };
        if item_slot.is_none() || item_slot.unwrap() != parent_item_slot {
            continue;
        };
        if location.is_none() {
            return Err(format!(
                "Item with id [{}] has no location, can't parse this profile :(",
                item._id
            ));
        }
        let location_in_stash = if let Location::LocationInStash(xy) = location.unwrap() {
            xy
        } else {
            panic!("oh no, wrong item: {}", item._id);
        };

        let bsg_item_option = get_bsg_item(item, bsg_items_root);
        if bsg_item_option.is_none() {
            return Err(format!(
                "Item with id [{}] and template [{}] can't be read",
                item._id, item._tpl
            ));
        }
        let bsg_item = bsg_item_option.unwrap();

        // if it's a container
        let mut grid_items: Option<Vec<GridItem>> = None;
        let is_container =
            bsg_item._props.grids.is_some() && !bsg_item._props.grids.as_ref().unwrap().is_empty();
        if is_container {
            grid_items = Some(Vec::new());

            for grid in &bsg_item._props.grids.unwrap() {
                let grid_name = &grid._name;

                let items_inside_container = parse_items(
                    profile_items.clone(),
                    bsg_items_root,
                    item._id.as_str(),
                    grid_name,
                )?;

                let grid_item = GridItem {
                    _name: grid_name.clone(),
                    cells_v: grid._props.cells_v,
                    cells_h: grid._props.cells_h,
                    items: items_inside_container,
                };

                grid_items.as_mut().unwrap().insert(0, grid_item);
            }
        }

        let has_slots =
            bsg_item._props.slots.is_some() && !bsg_item._props.slots.as_ref().unwrap().is_empty();

        let mut slot_items: Option<HashSet<SlotItem>> = None;
        if has_slots {
            slot_items = Some(HashSet::new());
            bsg_item._props.slots.unwrap().iter().for_each(|bsg_t| {
                let all_slots_from_item = find_all_slots_from_parent(
                    item._id.as_str(),
                    &profile_items,
                    bsg_t._name.as_str(),
                );

                slot_items.as_mut().unwrap().extend(all_slots_from_item);
            })
        }

        let mut amount = 1;
        let mut spawned_in_session = false;
        let mut resource = None;

        let max_resource = None
            .or(bsg_item._props.max_resource)
            .or(bsg_item._props.max_hp_resource)
            .or(bsg_item._props.maximum_number_of_usages)
            .or(bsg_item._props.max_durability);

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
                    // we are showing the real max durability, not the current repaired one, uncomment the following code to show that
                    // max_resource = Some(udp.repairable.as_ref().unwrap().max_durability);
                }
                if udp.key.is_some() {
                    resource = Some(
                        bsg_item._props.maximum_number_of_usages.unwrap()
                            - udp.key.as_ref().unwrap().number_of_usages,
                    );
                }
            }
        }

        let (size_x, size_y) =
            calculate_item_size(item, &profile_items, bsg_items_root, is_container);

        let stack_max_size = bsg_item._props.stack_max_size;
        let background_color = bsg_item._props.background_color;

        let i = Item {
            id: item._id.to_string(),
            tpl: item._tpl.to_string(),
            parent_id: item.parent_id.clone(),
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
            is_container,
            grid_items,
            slot_items,
        };
        items.push(i)
    }
    Ok(items)
}

fn calculate_item_size(
    item: &spt::spt_profile_serializer::Item,
    items: &[spt::spt_profile_serializer::Item],
    bsg_items_root: &HashMap<String, Value>,
    is_container: bool,
) -> (u16, u16) {
    let all_children: Vec<(String, String)> = if !is_container {
        find_all_ids_and_tpl_from_parent(item._id.as_str(), items, "mod_")
    } else {
        Vec::new()
    };
    // copied from InventoryHelper.getSizeByInventoryItemHash
    let parent_item = bsg_items_root.get(item._tpl.as_str()).unwrap();
    let parsed_parent_item = load_item(parent_item.to_string().as_str()).unwrap();

    let mut out_x = parsed_parent_item._props.width;
    let out_y = parsed_parent_item._props.height;
    let mut size_up = 0;
    let mut size_down = 0;
    let mut size_left = 0;
    let mut size_right = 0;
    let mut forced_up = 0;
    let mut forced_down = 0;
    let mut forced_left = 0;
    let mut forced_right = 0;

    let foldable = parsed_parent_item._props.foldable;
    let folded_slot = parsed_parent_item._props.folded_slot;
    if foldable.is_some()
        && foldable.unwrap()
        && folded_slot.is_some()
        && folded_slot.unwrap() == ""
        && item.upd.as_ref().unwrap().foldable.is_some()
        && item.upd.as_ref().unwrap().foldable.as_ref().unwrap().folded
    {
        out_x -= parsed_parent_item._props.size_reduced_right.unwrap();
    }

    all_children.iter().for_each(|(_id, tpl)| {
        let bsg_item = bsg_items_root.get(tpl).unwrap();
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

fn find_all_ids_and_tpl_from_parent(
    parent_id: &str,
    items: &[spt::spt_profile_serializer::Item],
    slot_id: &str,
) -> Vec<(String, String)> {
    let mut result: Vec<(String, String)> = Vec::new();

    for i in items {
        if i.parent_id.is_some() && i.parent_id.as_ref().unwrap() == parent_id {
            if i.slot_id.is_some() && i.slot_id.as_ref().unwrap().starts_with(slot_id) {
                result.push((i._id.to_string(), i._tpl.to_string()));
            }

            let sub_items = find_all_ids_and_tpl_from_parent(&i._id, items, slot_id);
            result.extend(sub_items);
        }
    }

    result
}

fn find_all_slots_from_parent(
    parent_id: &str,
    items: &[spt::spt_profile_serializer::Item],
    slot_id: &str,
) -> HashSet<SlotItem> {
    let mut result: HashSet<SlotItem> = HashSet::new();

    for i in items {
        if i.parent_id.is_some() && i.parent_id.as_ref().unwrap() == parent_id {
            if i.slot_id.is_some() && i.slot_id.as_ref().unwrap().starts_with(slot_id) {
                let id = i._id.to_string();
                let tpl = i._tpl.to_string();
                let slot_id = i.slot_id.as_ref().unwrap().to_string();
                result.insert(SlotItem { id, tpl, slot_id });
            }

            let sub_items = find_all_slots_from_parent(&i._id, items, "");
            result.extend(sub_items);
        }
    }

    result
}

fn get_bsg_item(
    item: &spt::spt_profile_serializer::Item,
    bsg_items_root: &HashMap<String, Value>,
) -> Option<spt::spt_bsg_items_serializer::BsgItem> {
    let parent_item = bsg_items_root.get(item._tpl.as_str())?;
    load_item(parent_item.to_string().as_str()).ok()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::Value;

    use crate::spt::spt_profile_serializer::{load_profile, Foldable, Item, UPD};
    use crate::ui_profile::ui_profile_serializer::{
        calculate_item_size, get_bsg_item, parse_items,
    };

    #[test]
    fn should_not_crash_if_template_is_not_found() {
        let item = Item {
            _id: "6c116ae6e3e795d2e508a5f0".to_string(),
            _tpl: "FAKE".to_string(),
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

        let bsg_item = get_bsg_item(&item, &bsg_items_root);
        assert!(bsg_item.is_none())
    }

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
            false,
        );
        assert_eq!(size_x, 3);
        assert_eq!(size_y, 2);
    }

    #[test]
    fn should_calculate_size_folded() {
        let item = Item {
            _id: "be82550094e077141e097192".to_string(),
            _tpl: "57d14d2524597714373db789".to_string(),
            parent_id: None,
            location: None,
            slot_id: None,
            upd: Some(UPD {
                stack_objects_count: None,
                spawned_in_session: None,
                food_drink: None,
                med_kit: None,
                resource: None,
                repairable: None,
                key: None,
                foldable: Some(Foldable { folded: true }),
                togglable: None,
                fire_mode: None,
            }),
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

        // folded weapon
        let (size_x, size_y) = calculate_item_size(
            &item,
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
            false,
        );
        assert_eq!(size_x, 2);
        assert_eq!(size_y, 2);
    }

    #[test]
    fn should_calculate_size_for_containers_with_attachments_inside() {
        let item = Item {
            _id: "fa17c8765e58fb25a97d7bba".to_string(),
            _tpl: "5b6d9ce188a4501afc1b2b25".to_string(),
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
            String::from_utf8_lossy(include_bytes!("../../../example/user/profiles/thicc.json"))
                .as_ref(),
        )
        .unwrap();

        // thicc weapon case with weapons inside
        let (size_x, size_y) = calculate_item_size(
            &item,
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
            true,
        );
        assert_eq!(size_x, 5);
        assert_eq!(size_y, 2);
    }

    #[test]
    fn should_handle_backpack_within_backpack() {
        let tarkov_profile = load_profile(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/user/profiles/af01e654f9af416ee4684a2c.json"
            ))
            .as_ref(),
        )
        .unwrap();
        let bsg_items_root: HashMap<String, Value> = serde_json::from_str(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/Aki_Data/Server/database/templates/items.json"
            ))
            .as_ref(),
        )
        .unwrap();
        let stash = &tarkov_profile.characters.pmc.inventory.stash;

        let items = parse_items(
            tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
            stash.as_str(),
            "hideout",
        )
        .ok()
        .unwrap();

        let backpack = items
            .iter()
            .find(|i| i.id == "e7d8a69bdd2554dca61cd984")
            .unwrap();
        assert!(backpack.is_container);
        let backpack_content = backpack.grid_items.as_ref().unwrap().get(0).unwrap();
        assert_eq!(backpack_content.cells_v, 5);
        assert_eq!(backpack_content.cells_h, 4);

        let nested_backpack = backpack_content.items.get(0).unwrap();
        assert!(nested_backpack.is_container);
    }

    #[test]
    fn should_calculate_all_attachments() {
        let tarkov_profile = load_profile(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/user/profiles/angel-git.json"
            ))
            .as_ref(),
        )
        .unwrap();
        let bsg_items_root: HashMap<String, Value> = serde_json::from_str(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/Aki_Data/Server/database/templates/items.json"
            ))
            .as_ref(),
        )
        .unwrap();
        let stash = &tarkov_profile.characters.pmc.inventory.stash;

        let items = parse_items(
            tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
            stash.as_str(),
            "hideout",
        )
        .ok()
        .unwrap();

        let vpo = items
            .iter()
            .find(|i| i.id == "1d0832b091d9e1e36e17666b")
            .unwrap();
        let slot_items = vpo.slot_items.as_ref().unwrap().len();
        assert_eq!(slot_items, 14);
    }
}
