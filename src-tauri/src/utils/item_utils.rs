use std::collections::HashMap;

use crate::prelude::*;

pub fn calculate_item_size(
    item: &spt_profile_serializer::InventoryItem,
    items: &[spt_profile_serializer::InventoryItem],
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
    let parsed_parent_item =
        spt_bsg_items_serializer::load_item(parent_item.to_string().as_str()).unwrap();

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
        && item.upd.is_some()
        && item.upd.as_ref().unwrap().foldable.is_some()
        && item.upd.as_ref().unwrap().foldable.as_ref().unwrap().folded
    {
        out_x -= parsed_parent_item._props.size_reduced_right.unwrap();
    }

    all_children.iter().for_each(|(id, tpl)| {
        let bsg_item = bsg_items_root.get(tpl).unwrap_or_else(|| {
            panic!(
                "Item with id [{}] and template id [{}] can't be found",
                id, tpl
            )
        });

        let parsed_bsg_item_result =
            spt_bsg_items_serializer::load_item(bsg_item.to_string().as_str());
        if parsed_bsg_item_result.is_err() {
            panic!(
                "Item with id [{}] and template id [{}] can't be parsed: {}",
                id,
                tpl,
                parsed_bsg_item_result.err().unwrap()
            )
        }
        let parsed_bsg_item = parsed_bsg_item_result.unwrap();

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
    items: &[spt_profile_serializer::InventoryItem],
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

pub fn get_upd_props_from_item(item: &Value) -> spt_profile_serializer::UPD {
    let props = item.get("_props").expect("no _props for item");

    let mut repairable: Option<spt_profile_serializer::Repairable> = None;
    let mut togglable: Option<spt_profile_serializer::Togglable> = None;
    let mut foldable: Option<spt_profile_serializer::Foldable> = None;
    let mut fire_mode: Option<spt_profile_serializer::FireMode> = None;
    let mut med_kit: Option<spt_profile_serializer::MedKit> = None;
    let mut food_drink: Option<spt_profile_serializer::FoodDrink> = None;

    if let Some(max_durability) = props.get("MaxDurability") {
        repairable = Some(spt_profile_serializer::Repairable {
            max_durability: Some(max_durability.as_u64().unwrap() as u32),
            durability: Some(max_durability.as_u64().unwrap() as u32),
        });
    }

    if let Some(has_hinge) = props.get("HasHinge") {
        if has_hinge.as_bool().unwrap() {
            togglable = Some(spt_profile_serializer::Togglable { on: true })
        }
    }

    if let Some(is_foldable) = props.get("Foldable") {
        if is_foldable.as_bool().unwrap() {
            foldable = Some(spt_profile_serializer::Foldable { folded: false })
        }
    }

    if let Some(fire_mode_array) = props.get("weapFireType") {
        if !fire_mode_array.as_array().unwrap().is_empty() {
            // this is fullauto in SPT
            fire_mode = Some(spt_profile_serializer::FireMode {
                fire_mode: "single".to_string(),
            })
        }
    }

    if let Some(max_hp) = props.get("MaxHpResource") {
        let max_hp_64 = max_hp.as_u64().unwrap();
        if max_hp_64 > 0 {
            med_kit = Some(spt_profile_serializer::MedKit {
                hp_resource: max_hp_64 as u32,
            });

            if let Some(_food_use_time) = props.get("foodUseTime") {
                food_drink = Some(spt_profile_serializer::FoodDrink { hp_percent: 100 })
            }
        }
    }

    spt_profile_serializer::UPD {
        stack_objects_count: Some(1),
        spawned_in_session: Some(false),
        food_drink,
        med_kit,
        resource: None,
        repairable,
        key: None,
        foldable,
        togglable,
        fire_mode,
    }
}

pub fn is_headwear_item(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    find_parent_by_name(bsg_items_root, tpl, "Headwear").is_some()
}
pub fn is_vest_item(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    find_parent_by_name(bsg_items_root, tpl, "Vest").is_some()
}

pub fn is_armor_item(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    find_parent_by_name(bsg_items_root, tpl, "Armor").is_some()
}

pub fn is_ammo_item(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    find_parent_by_name(bsg_items_root, tpl, "Ammo").is_some()
}

pub fn is_ammo_box(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    find_parent_by_name(bsg_items_root, tpl, "AmmoBox").is_some()
}

pub fn is_weapon_item(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    find_parent_by_name(bsg_items_root, tpl, "Weapon").is_some()
}

pub fn is_magazine_item(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    find_parent_by_name(bsg_items_root, tpl, "Magazine").is_some()
}

fn find_parent_by_name(
    bsg_items_root: &HashMap<String, Value>,
    current_id: &str,
    target_name: &str,
) -> Option<Value> {
    if let Some(node) = bsg_items_root.get(current_id) {
        let name = node.get("_name").unwrap().as_str().unwrap();
        if name == target_name {
            Some(node.clone())
        } else if let Some(parent_id) = node.get("_parent") {
            find_parent_by_name(bsg_items_root, parent_id.as_str().unwrap(), target_name)
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::spt::spt_profile_serializer::{load_profile, Foldable, InventoryItem, UPD};
    use crate::utils::item_utils::{calculate_item_size, get_upd_props_from_item, is_armor_item};
    use serde_json::Value;
    use std::collections::HashMap;

    #[test]
    fn should_calculate_size_with_attachments() {
        let item = InventoryItem {
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
        let item = InventoryItem {
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
        let item = InventoryItem {
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
    fn should_not_be_toggable() {
        let item = serde_json::json!({
          "_id": "5926bb2186f7744b1c6c6e60",
          "_props": {
            "HasHinge": false,
          },
        });

        let upd = get_upd_props_from_item(&item);
        assert!(upd.togglable.is_none());
    }

    #[test]
    fn should_not_be_folded() {
        let item = serde_json::json!({
          "_id": "5926bb2186f7744b1c6c6e60",
          "_props": {
            "Foldable": false,
          },
        });

        let upd = get_upd_props_from_item(&item);
        assert!(upd.foldable.is_none());
    }

    #[test]
    fn should_not_be_fire_mode() {
        let item = serde_json::json!({
          "_id": "5926bb2186f7744b1c6c6e60",
          "_props": {
            "weapFireType": [],
          },
        });

        let upd = get_upd_props_from_item(&item);
        assert!(upd.fire_mode.is_none());
    }

    #[test]
    fn should_not_be_med_kit() {
        let item = serde_json::json!({
          "_id": "5926bb2186f7744b1c6c6e60",
          "_props": {
            "MaxHpResource": 0,
          },
        });

        let upd = get_upd_props_from_item(&item);
        assert!(upd.med_kit.is_none());
    }

    #[test]
    fn should_not_be_food_drink() {
        let item = serde_json::json!({
          "_id": "5926bb2186f7744b1c6c6e60",
          "_props": {
            "foodUseTime": 10,
          },
        });

        let upd = get_upd_props_from_item(&item);
        assert!(upd.med_kit.is_none());
        assert!(upd.food_drink.is_none());
    }

    #[test]
    fn should_create_upd() {
        let item = serde_json::json!({
          "_id": "5926bb2186f7744b1c6c6e60",
          "_props": {
            "MaxDurability": 100,
            "Foldable": true,
            "HasHinge": true,
            "weapFireType": [
              "single",
              "fullauto"
            ],
            "MaxHpResource": 50,
            "foodUseTime": 10,
          },
        });

        let upd = get_upd_props_from_item(&item);
        assert_eq!(upd.stack_objects_count, Some(1));
        assert_eq!(upd.spawned_in_session, Some(false));
        assert_eq!(upd.repairable.as_ref().unwrap().durability.unwrap(), 100);
        assert_eq!(
            upd.repairable.as_ref().unwrap().max_durability.unwrap(),
            100
        );
        assert!(!upd.foldable.as_ref().unwrap().folded);
        assert!(upd.togglable.as_ref().unwrap().on);
        assert_eq!(upd.fire_mode.unwrap().fire_mode, "single".to_string());
        assert_eq!(upd.med_kit.unwrap().hp_resource, 50);
        assert_eq!(upd.food_drink.unwrap().hp_percent, 100);
    }

    #[test]
    fn should_find_paca_as_armor() {
        let bsg_items_root: HashMap<String, Value> = serde_json::from_str(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/Aki_Data/Server/database/templates/items.json"
            ))
            .as_ref(),
        )
        .unwrap();
        assert!(is_armor_item("5648a7494bdc2d9d488b4583", &bsg_items_root));
    }
}
