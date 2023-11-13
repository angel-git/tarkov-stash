use std::collections::HashMap;

use serde_json::Value;

use crate::spt::spt_profile_serializer::{
    FireMode, Foldable, FoodDrink, MedKit, Repairable, Togglable, UPD,
};

pub fn get_upd_props_from_item(item: &Value) -> UPD {
    let props = item.get("_props").expect("no _props for item");

    let mut repairable: Option<Repairable> = None;
    let mut togglable: Option<Togglable> = None;
    let mut foldable: Option<Foldable> = None;
    let mut fire_mode: Option<FireMode> = None;
    let mut med_kit: Option<MedKit> = None;
    let mut food_drink: Option<FoodDrink> = None;

    if let Some(max_durability) = props.get("MaxDurability") {
        repairable = Some(Repairable {
            max_durability: max_durability.as_u64().unwrap() as u16,
            durability: max_durability.as_u64().unwrap() as u16,
        });
    }

    if let Some(has_hinge) = props.get("HasHinge") {
        if has_hinge.as_bool().unwrap() {
            togglable = Some(Togglable { on: true })
        }
    }

    if let Some(is_foldable) = props.get("Foldable") {
        if is_foldable.as_bool().unwrap() {
            foldable = Some(Foldable { folded: false })
        }
    }

    if let Some(fire_mode_array) = props.get("weapFireType") {
        if !fire_mode_array.as_array().unwrap().is_empty() {
            // this is fullauto in SPT
            fire_mode = Some(FireMode {
                fire_mode: "single".to_string(),
            })
        }
    }

    if let Some(max_hp) = props.get("MaxHpResource") {
        let max_hp_64 = max_hp.as_u64().unwrap();
        if max_hp_64 > 0 {
            med_kit = Some(MedKit {
                hp_resource: max_hp_64 as u16,
            });

            if let Some(_food_use_time) = props.get("foodUseTime") {
                food_drink = Some(FoodDrink { hp_percent: 100 })
            }
        }
    }

    UPD {
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

pub fn is_ammo_item(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    find_parent_by_name(bsg_items_root, tpl, "Ammo").is_some()
}

pub fn is_weapon_item(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    find_parent_by_name(bsg_items_root, tpl, "Weapon").is_some()
}

pub fn is_magazine_item(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    find_parent_by_name(bsg_items_root, tpl, "Magazine").is_some()
}

pub fn is_compound_without_hide_entrails(
    tpl: &str,
    bsg_items_root: &HashMap<String, Value>,
) -> bool {
    if let Some(node) = bsg_items_root.get(tpl) {
        if let Some(hide) = node
            .get("_props")
            .and_then(|p| p.get("HideEntrails").and_then(|hide| hide.as_bool()))
        {
            if !hide {
                if let Some(slots) = node.get("_props").and_then(|p| p.get("Slots")) {
                    slots.as_array().is_some_and(|slot| !slot.is_empty())
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
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
            find_parent_by_name(bsg_items_root, &parent_id.as_str().unwrap(), target_name)
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::Value;

    use crate::utils::item_utils::{
        get_upd_props_from_item, is_ammo_item, is_compound_without_hide_entrails,
    };

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
        assert_eq!(upd.repairable.as_ref().unwrap().durability, 100);
        assert_eq!(upd.repairable.as_ref().unwrap().max_durability, 100);
        assert!(!upd.foldable.as_ref().unwrap().folded);
        assert!(upd.togglable.as_ref().unwrap().on);
        assert_eq!(upd.fire_mode.unwrap().fire_mode, "single".to_string());
        assert_eq!(upd.med_kit.unwrap().hp_resource, 50);
        assert_eq!(upd.food_drink.unwrap().hp_percent, 100);
    }

    #[test]
    fn should_item_be_ammo() {
        let bsg_items_root: HashMap<String, Value> = serde_json::from_str(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/Aki_Data/Server/database/templates/items.json"
            ))
            .as_ref(),
        )
        .unwrap();
        let is_ammo = is_ammo_item("5ea2a8e200685063ec28c05a", &bsg_items_root);
        assert!(is_ammo);
    }

    #[test]
    fn should_container_not_be_compound_without_hide_entrails() {
        let bsg_items_root: HashMap<String, Value> = serde_json::from_str(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/Aki_Data/Server/database/templates/items.json"
            ))
            .as_ref(),
        )
        .unwrap();
        let backpack =
            is_compound_without_hide_entrails("5e9dcf5986f7746c417435b3", &bsg_items_root);
        assert!(!backpack);
    }

    #[test]
    fn should_weapon_be_compound_without_hide_entrails() {
        let bsg_items_root: HashMap<String, Value> = serde_json::from_str(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/Aki_Data/Server/database/templates/items.json"
            ))
            .as_ref(),
        )
        .unwrap();
        let weapon = is_compound_without_hide_entrails("59e6152586f77473dc057aa1", &bsg_items_root);
        assert!(weapon);
    }
}
