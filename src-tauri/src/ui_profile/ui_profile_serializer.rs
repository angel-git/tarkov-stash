pub use crate::prelude::*;
use crate::spt::spt_profile_serializer::TarkovProfile;
use crate::utils::cache_utils::{load_cache_icon_index_file, load_image_from_cache};
use serde_json::Map;
use std::collections::{HashMap, HashSet};

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
    pub locale: HashMap<String, Value>,
    #[serde(rename = "presetItems")]
    pub preset_items: Vec<PresetItem>,
    #[serde(rename = "userPresets")]
    pub user_presets: Vec<UserPresetItem>,
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
    pub resource: Option<u32>,
    #[serde(rename = "maxResource")]
    pub max_resource: Option<u32>,
    #[serde(rename = "backgroundColor")]
    pub background_color: String,
    #[serde(rename = "isContainer")]
    pub is_container: bool,
    #[serde(rename = "gridItems")]
    pub grid_items: Option<Vec<GridItem>>,
    #[serde(rename = "slotItems")]
    pub slot_items: Option<HashSet<SlotItem>>,
    #[serde(rename = "presetImageId")]
    pub preset_image_id: Option<String>,
    #[serde(rename = "cacheImage")]
    pub cache_image: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PresetItem {
    pub id: String,
    pub encyclopedia: Option<String>,
    pub items: Vec<spt_profile_serializer::InventoryItem>,
    pub width: u16,
    pub height: u16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserPresetItem {
    pub id: String,
    pub name: String,
    #[serde(rename = "cacheImage")]
    pub cache_image: Option<String>,
    pub items: Vec<spt_profile_serializer::InventoryItem>,
    pub width: u16,
    pub height: u16,
}

#[derive(Deserialize, Serialize, Debug, Hash, Eq, PartialEq)]
pub struct SlotItem {
    pub id: String,
    pub tpl: String,
    #[serde(rename = "slotId")]
    pub slot_id: String,
    pub upd: Option<spt_profile_serializer::UPD>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GridItem {
    #[serde(rename = "name")]
    pub _name: String,
    #[serde(rename = "cellsH")]
    pub cells_h: i16,
    #[serde(rename = "cellsV")]
    pub cells_v: i16,
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
    #[serde(rename = "stackMaxSize")]
    pub stack_max_size: Value,
}

pub fn convert_profile_to_ui(
    tarkov_profile: TarkovProfile,
    bsg_items_root: &HashMap<String, Value>,
    locale_root: &HashMap<String, Value>,
    globals: &HashMap<String, Value>,
    load_image_cache: bool,
    server_path: &str,
) -> Result<UIProfile, String> {
    let stash = &tarkov_profile.characters.pmc.inventory.stash;
    let (stash_size_x, stash_size_y) = calculate_stash_size(&tarkov_profile, bsg_items_root);

    let cache_icon_index_file = if load_image_cache {
        load_cache_icon_index_file(server_path)
    } else {
        None
    };

    let items: Vec<Item> = parse_items(
        &tarkov_profile.characters.pmc.inventory.items,
        bsg_items_root,
        stash.as_str(),
        "hideout",
        globals,
        &cache_icon_index_file,
        server_path,
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
                stack_max_size: props
                    .and_then(|p| p.get("StackMaxSize"))
                    .cloned()
                    .unwrap_or(Value::from(0)),
            },
        );
    });

    Ok(UIProfile {
        name: tarkov_profile.characters.pmc.info.nickname,
        size_x: stash_size_x,
        size_y: stash_size_y,
        items,
        bsg_items,
        spt_version: None,
        locale: locale_root.clone(),
        preset_items: preset_utils::find_all_item_presets(globals, bsg_items_root),
        user_presets: preset_utils::find_all_user_weapons_builds(
            &tarkov_profile.user_builds,
            bsg_items_root,
            &cache_icon_index_file,
            server_path,
        ),
    })
}

fn parse_items(
    profile_items: &Vec<spt_profile_serializer::InventoryItem>,
    bsg_items_root: &HashMap<String, Value>,
    parent_slot: &str,
    parent_item_slot: &str,
    globals: &HashMap<String, Value>,
    index_cache: &Option<Map<String, Value>>,
    server_path: &str,
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
        let location_in_stash =
            if let spt_profile_serializer::Location::LocationInStash(xy) = location.unwrap() {
                xy
            } else {
                panic!("oh no, wrong item: {}", item._id);
            };

        let bsg_item = get_bsg_item(item, bsg_items_root);

        // if it's a container
        let mut grid_items: Option<Vec<GridItem>> = None;
        let is_container =
            bsg_item._props.grids.is_some() && !bsg_item._props.grids.as_ref().unwrap().is_empty();
        if is_container {
            grid_items = Some(Vec::new());

            for grid in &bsg_item._props.grids.unwrap() {
                let grid_name = &grid._name;

                let items_inside_container = parse_items(
                    profile_items,
                    bsg_items_root,
                    item._id.as_str(),
                    grid_name,
                    globals,
                    index_cache,
                    server_path,
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

        let mut amount = 1;
        let mut spawned_in_session = false;
        let mut resource: Option<u32> = None;
        // TODO calculate max resource from slots
        let mut max_resource = None
            .or(bsg_item._props.max_resource)
            .or(bsg_item._props.max_hp_resource)
            .or(bsg_item._props.maximum_number_of_usages)
            .or(bsg_item._props.max_durability);

        // calculate amount, it might be coming from cartridges
        let slots = find_all_slots_from_parent(item._id.as_str(), profile_items, "cartridges");
        if !slots.is_empty() {
            amount = slots.iter().fold(0, |acc, slot| {
                if let Some(stack_objects_count) = slot
                    .upd
                    .as_ref()
                    .and_then(|u| u.stack_objects_count.as_ref())
                {
                    acc + stack_objects_count
                } else {
                    acc
                }
            });
        }

        if udp_option.is_some() {
            if let Some(udp) = udp_option {
                if udp.stack_objects_count.is_some() && amount == 1 {
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
                    resource = udp.repairable.as_ref().unwrap().durability;
                    // we are showing the real max durability, not the current repaired one, uncomment the following code to show that
                    // max_resource = Some(udp.repairable.as_ref().unwrap().max_durability);
                }
                if udp.key.is_some() {
                    let bsg_maximum_number_of_usages =
                        bsg_item._props.maximum_number_of_usages.unwrap();
                    if bsg_maximum_number_of_usages == 0 {
                        resource = Some(1);
                    } else {
                        resource = Some(
                            bsg_maximum_number_of_usages
                                - udp.key.as_ref().unwrap().number_of_usages,
                        );
                    };
                }
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
                    profile_items,
                    bsg_t._name.as_str(),
                );

                slot_items.as_mut().unwrap().extend(all_slots_from_item);
            });

            if let Some(ref slot_items_set) = slot_items {
                let mut slot_resource = 0;
                let mut slot_max_resource = 0;

                slot_items_set.iter().for_each(|slot_item| {
                    if let Some(slot_upd) = slot_item.upd.as_ref() {
                        if let Some(repairable) = slot_upd.repairable.as_ref() {
                            if let Some(durability) = repairable.durability {
                                slot_resource += durability;
                            }
                            if let Some(max_durability) = repairable.max_durability {
                                slot_max_resource += max_durability;
                            }
                        }
                    }
                });

                resource = Some(slot_resource);
                max_resource = Some(slot_max_resource);
            }
        }

        let (size_x, size_y) =
            item_utils::calculate_item_size(item, profile_items, bsg_items_root, is_container);

        let stack_max_size = bsg_item._props.stack_max_size;
        let background_color = bsg_item._props.background_color;
        let preset_image_id =
            preset_utils::find_id_from_encyclopedia(item._tpl.as_str(), globals, bsg_items_root);

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
            preset_image_id,
            cache_image: if index_cache.is_some() {
                load_image_from_cache(
                    item,
                    profile_items,
                    bsg_items_root,
                    index_cache.as_ref().unwrap(),
                    server_path,
                )
            } else {
                None
            },
        };
        items.push(i)
    }

    Ok(items)
}

fn find_all_slots_from_parent(
    parent_id: &str,
    items: &[spt_profile_serializer::InventoryItem],
    slot_id: &str,
) -> HashSet<SlotItem> {
    let mut result: HashSet<SlotItem> = HashSet::new();

    for i in items {
        if i.parent_id.is_some() && i.parent_id.as_ref().unwrap() == parent_id {
            if i.slot_id.is_some() && i.slot_id.as_ref().unwrap().starts_with(slot_id) {
                let id = i._id.to_string();
                let tpl = i._tpl.to_string();
                let slot_id = i.slot_id.as_ref().unwrap().to_string();
                let upd = i.upd.as_ref().cloned();
                result.insert(SlotItem {
                    id,
                    tpl,
                    slot_id,
                    upd,
                });
            }

            let sub_items = find_all_slots_from_parent(&i._id, items, "");
            result.extend(sub_items);
        }
    }

    result
}

fn get_bsg_item(
    item: &spt_profile_serializer::InventoryItem,
    bsg_items_root: &HashMap<String, Value>,
) -> spt_bsg_items_serializer::BsgItem {
    let parent_item = bsg_items_root.get(item._tpl.as_str());
    if parent_item.is_none() {
        panic!(
            "Error finding the item template [{}] in the database",
            item._tpl
        );
    }
    match spt_bsg_items_serializer::load_item(parent_item.unwrap().to_string().as_str()) {
        Ok(bsg_item) => bsg_item,
        Err(e) => {
            panic!("Error while parsing item template [{}]: {}", item._tpl, e);
        }
    }
}

fn calculate_stash_size(
    tarkov_profile: &TarkovProfile,
    bsg_items_root: &HashMap<String, Value>,
) -> (u16, u16) {
    let grids = tarkov_profile
        .characters
        .pmc
        .bonuses
        .iter()
        .filter(|b| b.t.eq("StashSize"))
        .filter_map(|b| bsg_items_root.get(b.template_id.as_ref().unwrap().as_str()))
        .filter_map(|bonus_items| bonus_items.pointer("/_props/Grids").unwrap().as_array());

    let stash_size_y: u16 = grids
        .clone()
        .flatten()
        .filter_map(|stash_bonuses_grid| {
            stash_bonuses_grid
                .pointer("/_props/cellsV")
                .unwrap()
                .as_u64()
        })
        .max()
        .unwrap() as u16;

    let stash_size_x: u16 = grids
        .clone()
        .flatten()
        .filter_map(|stash_bonuses_grid| {
            stash_bonuses_grid
                .pointer("/_props/cellsH")
                .unwrap()
                .as_u64()
        })
        .max()
        .unwrap() as u16;

    let extra_rows = match &tarkov_profile
        .characters
        .pmc
        .bonuses
        .iter()
        .find(|b| b.t.eq("StashRows"))
        .map(|b| b.value.as_ref().unwrap().as_u64().unwrap() as u16)
    {
        Some(value) => *value,
        _ => 0,
    };

    (stash_size_x, stash_size_y + extra_rows)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::prelude::convert_profile_to_ui;
    use serde_json::Value;

    use crate::spt::spt_profile_serializer::load_profile;
    use crate::ui_profile::ui_profile_serializer::parse_items;

    #[test]
    fn should_not_crash_if_values_are_float() {
        let tarkov_profile = load_profile(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/user/profiles/af01e654f9af416ee4684a2c.json"
            ))
            .as_ref(),
        );

        assert!(tarkov_profile.is_ok())
    }

    #[test]
    fn should_calculate_item_stash_bonuses() {
        let tarkov_profile = load_profile(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/user/profiles/380-bonus-stash.json"
            ))
            .as_ref(),
        );
        let bsg_items_root: HashMap<String, Value> = serde_json::from_str(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/Aki_Data/Server/database/templates/items.json"
            ))
            .as_ref(),
        )
        .unwrap();
        assert!(tarkov_profile.is_ok());
        let profile_ui = convert_profile_to_ui(
            tarkov_profile.unwrap(),
            &bsg_items_root,
            &HashMap::new(),
            &HashMap::new(),
            false,
            "",
        );
        assert!(profile_ui.is_ok());
        assert_eq!(profile_ui.as_ref().unwrap().size_y, 70);
        assert_eq!(profile_ui.as_ref().unwrap().size_x, 10);
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
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
            stash.as_str(),
            "hideout",
            &HashMap::new(),
            &None,
            "",
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
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
            stash.as_str(),
            "hideout",
            &HashMap::new(),
            &None,
            "",
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
