use crate::ui_profile::ui_profile_serializer::Item;
use crate::utils::hash_utils::generate;
use crate::utils::item_utils::get_upd_props_from_item;
use serde::{Deserialize, Serialize};
use serde_json::{json, Error, Value};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct NewItem {
    pub id: String,
    #[serde(rename = "locationX")]
    pub location_x: u16,
    #[serde(rename = "locationY")]
    pub location_y: u16,
}

pub fn update_item_amount(
    file_content: &str,
    item: &Item,
    _bsg_items: &HashMap<String, Value>,
) -> Result<String, Error> {
    let item_id = item.id.as_str();
    let stack_objects_count = item.amount;
    let mut root: Value = serde_json::from_str(file_content)?;

    let optional_items = get_inventory_items(&mut root);

    if let Some(items) = optional_items {
        if let Some(item) = items
            .iter_mut()
            .find(|i| i.get("_id").unwrap().as_str().unwrap() == item_id)
        {
            if let Some(upd) = item.get_mut("upd") {
                if let Some(value) = upd.get_mut("StackObjectsCount") {
                    *value = Value::from(stack_objects_count);
                }
            }
        }
    }

    serde_json::to_string(&root)
}

pub fn update_spawned_in_session(
    file_content: &str,
    item: &Item,
    _bsg_items: &HashMap<String, Value>,
) -> Result<String, Error> {
    let item_id = item.id.as_str();
    let mut root: Value = serde_json::from_str(file_content).unwrap();

    let optional_items = get_inventory_items(&mut root);

    if let Some(items) = optional_items {
        for i in items.iter_mut() {
            let id = i.get("_id").unwrap().as_str().unwrap();
            if id == item_id {
                if let Some(upd) = i.get_mut("upd") {
                    let upd_map = upd.as_object_mut().unwrap();
                    upd_map.insert("SpawnedInSession".to_string(), true.into());
                } else {
                    let empty_upd = json!({"SpawnedInSession": true});
                    if let Some(item) = i.as_object_mut() {
                        item.insert("upd".to_string(), empty_upd);
                    }
                }
            }
        }
    }

    serde_json::to_string(&root)
}

pub fn update_durability(
    file_content: &str,
    item: &Item,
    bsg_items: &HashMap<String, Value>,
) -> Result<String, Error> {
    let item_id = item.id.as_str();
    let mut root: Value = serde_json::from_str(file_content).unwrap();

    let optional_items = get_inventory_items(&mut root);

    if let Some(items) = optional_items {
        for i in items.iter_mut() {
            let id = i.get("_id").unwrap().as_str().unwrap();
            if id == item_id {
                if let Some(upd) = i.get_mut("upd") {
                    // restore keys
                    if let Some(key) = upd.get_mut("Key") {
                        let upd_map = key.as_object_mut().unwrap();
                        upd_map.insert("NumberOfUsages".to_string(), 0.into());
                    }

                    // restore HpResource
                    if let Some(key) = upd.get_mut("MedKit") {
                        let upd_map = key.as_object_mut().unwrap();
                        let max_hp = bsg_items
                            .get(item.tpl.as_str())
                            .unwrap()
                            .get("_props")
                            .unwrap()
                            .get("MaxHpResource")
                            .unwrap()
                            .as_u64();
                        upd_map.insert("HpResource".to_string(), max_hp.into());
                    }

                    // restore Resource
                    if let Some(key) = upd.get_mut("Resource") {
                        let upd_map = key.as_object_mut().unwrap();
                        let max_resource = bsg_items
                            .get(item.tpl.as_str())
                            .unwrap()
                            .get("_props")
                            .unwrap()
                            .get("MaxResource")
                            .unwrap()
                            .as_u64();
                        upd_map.insert("Value".to_string(), max_resource.into());
                    }

                    // restore FoodDrink
                    if let Some(key) = upd.get_mut("FoodDrink") {
                        let upd_map = key.as_object_mut().unwrap();
                        let max_resource = bsg_items
                            .get(item.tpl.as_str())
                            .unwrap()
                            .get("_props")
                            .unwrap()
                            .get("MaxResource")
                            .unwrap()
                            .as_u64();
                        upd_map.insert("HpPercent".to_string(), max_resource.into());
                    }

                    // restore Durability
                    if let Some(key) = upd.get_mut("Repairable") {
                        let upd_map = key.as_object_mut().unwrap();
                        let max_resource = bsg_items
                            .get(item.tpl.as_str())
                            .unwrap()
                            .get("_props")
                            .unwrap()
                            .get("MaxDurability")
                            .unwrap()
                            .as_u64();
                        upd_map.insert("Durability".to_string(), max_resource.into());
                        upd_map.insert("MaxDurability".to_string(), max_resource.into());
                    }
                }
            }
        }
    }

    serde_json::to_string(&root)
}

pub fn add_new_item(
    profile_content: &str,
    template_id: &str,
    location_x: u16,
    location_y: u16,
    bsg_items: &HashMap<String, Value>,
) -> Result<String, Error> {
    let mut root: Value = serde_json::from_str(profile_content).unwrap();

    let stash = root
        .pointer("/characters/pmc/Inventory/stash")
        .expect("Stash missing");
    let items_option = root
        .pointer("/characters/pmc/Inventory/items")
        .expect("Items missing");

    if let Some(items) = items_option.as_array() {
        // Clone the items array to make it mutable
        let mut cloned_items = items.clone();

        let item_id = generate();
        // TODO check if id already exists

        let bsg_item = bsg_items.get(template_id).expect("No item!");
        let upd = get_upd_props_from_item(bsg_item);
        let new_item_json = json!(
            {
                "_id": item_id,
                "_tpl": template_id,
                "location": {
                    "isSearched": true,
                    "r": "Horizontal",
                    "x": location_x,
                    "y": location_y,
                },
                "upd": upd,
                "parentId": stash,
                "slotId": "hideout"
            }
        );
        cloned_items.push(new_item_json);
        if let Some(root_items) = root.pointer_mut("/characters/pmc/Inventory/items") {
            *root_items = Value::Array(cloned_items);
        }
    }

    serde_json::to_string(&root)
}

pub fn delete_item(
    file_content: &str,
    item: &Item,
    _bsg_items: &HashMap<String, Value>,
) -> Result<String, Error> {
    let item_id = item.id.as_str();
    let mut root: Value = serde_json::from_str(file_content).unwrap();

    if let Some(inventory_items) = get_inventory_items(&mut root) {
        inventory_items.retain(|i| {
            let id = i.get("_id").and_then(|v| v.as_str());
            let parent_id = i.get("parentId").and_then(|v| v.as_str());
            // Keep items where the id is not equal to item_id and parent_id is not equal to item_id
            id != Some(item_id) && parent_id != Some(item_id)
        });
    }

    if let Some(insured_items) = get_insured_items(&mut root) {
        insured_items.retain(|i| {
            let id = i.get("itemId").and_then(|v| v.as_str());
            // Keep items where the id is not equal to item_id
            id != Some(item_id)
        });
    }

    serde_json::to_string(&root)
}

fn get_inventory_items(root: &mut Value) -> Option<&mut Vec<Value>> {
    root.get_mut("characters")
        .and_then(|v| v.get_mut("pmc"))
        .and_then(|v| v.get_mut("Inventory"))
        .and_then(|v| v.get_mut("items"))
        .and_then(|v| v.as_array_mut())
}

fn get_insured_items(root: &mut Value) -> Option<&mut Vec<Value>> {
    root.get_mut("characters")
        .and_then(|v| v.get_mut("pmc"))
        .and_then(|v| v.get_mut("Inventory"))
        .and_then(|v| v.get_mut("InsuredItems"))
        .and_then(|v| v.as_array_mut())
}

#[cfg(test)]
mod tests {
    use crate::stash::stash_utils::{
        delete_item, update_durability, update_item_amount, update_spawned_in_session,
    };
    use crate::ui_profile::ui_profile_serializer::Item;
    use serde_json::Value;
    use std::collections::HashMap;

    #[test]
    fn should_update_json_with_new_currency() {
        let json = r#"
            {
  "characters": {
    "pmc": {
      "Inventory": {
        "equipment": "5fe49a0e2694b0755a50476c",
        "fastPanel": {
          "Item4": "7b0724ada2f7f2e45204fb70"
        },
        "items": [
          {
            "_id": "41d413738e53a1eaa78d3112",
            "_tpl": "5449016a4bdc2d6f028b456f",
            "location": {
              "isSearched": true,
              "r": "Horizontal",
              "x": 2,
              "y": 2
            },
            "parentId": "3e267b00df2b047e9d037cfd",
            "slotId": "main",
            "upd": {
              "StackObjectsCount": 53098
            }
          },
          {
            "_id": "3eb9393a58f797c0c5f0e38e",
            "_tpl": "590c37d286f77443be3d7827",
            "location": {
              "isSearched": true,
              "r": "Horizontal",
              "x": 6,
              "y": 60
            },
            "parentId": "5fe49a0e2694b0755a504876",
            "slotId": "hideout",
            "upd": {
              "SpawnedInSession": true
            }
          }
        ]
      }
    }
  }
}"#;

        let item = Item {
            id: "41d413738e53a1eaa78d3112".to_string(),
            tpl: "5449016a4bdc2d6f028b456f".to_string(),
            x: 0,
            y: 0,
            size_x: 0,
            size_y: 0,
            amount: 66,
            stack_max_size: 1,
            is_stockable: false,
            is_fir: false,
            r: "Horitzontal".to_string(),
            resource: None,
            max_resource: None,
            background_color: "".to_string(),
            is_container: false,
            grid_items: None,
        };
        let updated = update_item_amount(json, &item, &HashMap::new()).unwrap();
        let root: Value = serde_json::from_str(updated.as_str()).unwrap();
        let mut test_ok = false;

        let optional_items = root
            .get("characters")
            .and_then(|v| v.get("pmc"))
            .and_then(|v| v.get("Inventory"))
            .and_then(|v| v.get("items"))
            .and_then(|v| v.as_array());

        if let Some(items) = optional_items {
            let item_41d413738e53a1eaa78d3112 = items
                .iter()
                .find(|i| i.get("_id").unwrap().eq("41d413738e53a1eaa78d3112"))
                .unwrap();
            let stack_41d413738e53a1eaa78d3112 = item_41d413738e53a1eaa78d3112
                .get("upd")
                .unwrap()
                .get("StackObjectsCount")
                .unwrap();
            assert_eq!(
                stack_41d413738e53a1eaa78d3112.as_u64().unwrap(),
                Value::from(66).as_u64().unwrap()
            );
            test_ok = true;
        }

        assert!(test_ok);
    }

    #[test]
    fn should_update_json_with_fir() {
        let json = r#"
            {
  "characters": {
    "pmc": {
      "Inventory": {
        "equipment": "5fe49a0e2694b0755a50476c",
        "fastPanel": {
          "Item4": "7b0724ada2f7f2e45204fb70"
        },
        "items": [
          {
            "_id": "41d413738e53a1eaa78d3112",
            "_tpl": "5449016a4bdc2d6f028b456f",
            "location": {
              "isSearched": true,
              "r": "Horizontal",
              "x": 2,
              "y": 2
            },
            "parentId": "3e267b00df2b047e9d037cfd",
            "slotId": "main"
          },
          {
            "_id": "3eb9393a58f797c0c5f0e38e",
            "_tpl": "590c37d286f77443be3d7827",
            "location": {
              "isSearched": true,
              "r": "Horizontal",
              "x": 6,
              "y": 60
            },
            "parentId": "5fe49a0e2694b0755a504876",
            "slotId": "hideout",
            "upd": {
              "SpawnedInSession": false
            }
          }
        ]
      }
    }
  }
}"#;

        let item_41d413738e53a1eaa78d3112 = Item {
            id: "41d413738e53a1eaa78d3112".to_string(),
            tpl: "5449016a4bdc2d6f028b456f".to_string(),
            x: 0,
            y: 0,
            size_x: 0,
            size_y: 0,
            amount: 66,
            stack_max_size: 1,
            is_stockable: false,
            is_fir: false,
            r: "Horitzontal".to_string(),
            resource: None,
            max_resource: None,
            background_color: "".to_string(),
            is_container: false,
            grid_items: None,
        };
        let item_3eb9393a58f797c0c5f0e38e = Item {
            id: "3eb9393a58f797c0c5f0e38e".to_string(),
            tpl: "590c37d286f77443be3d7827".to_string(),
            x: 0,
            y: 0,
            size_x: 0,
            size_y: 0,
            amount: 66,
            stack_max_size: 1,
            is_stockable: false,
            is_fir: false,
            r: "Horitzontal".to_string(),
            resource: None,
            max_resource: None,
            background_color: "".to_string(),
            is_container: false,
            grid_items: None,
        };
        // update both items
        let update_41d413738e53a1eaa78d3112 =
            update_spawned_in_session(json, &item_41d413738e53a1eaa78d3112, &HashMap::new())
                .unwrap();
        let update_3eb9393a58f797c0c5f0e38e = update_spawned_in_session(
            update_41d413738e53a1eaa78d3112.as_str(),
            &item_3eb9393a58f797c0c5f0e38e,
            &HashMap::new(),
        )
        .unwrap();

        // verify all have `SpawnedInSession` as true
        let root: Value = serde_json::from_str(update_3eb9393a58f797c0c5f0e38e.as_str()).unwrap();
        let mut test_ok = false;

        let upds: Option<Vec<&Value>> = root
            .get("characters")
            .and_then(|v| v.get("pmc"))
            .and_then(|v| v.get("Inventory"))
            .and_then(|v| v.get("items"))
            .and_then(|v| v.as_array())
            .and_then(|v| v.iter().map(|i| i.get("upd")).collect());

        upds.unwrap().iter().for_each(|upd| {
            let fir = upd.get("SpawnedInSession").unwrap().as_bool().unwrap();
            assert!(fir);
            test_ok = true;
        });

        assert!(test_ok);
    }

    #[test]
    fn should_update_food_drink() {
        let json = r#"
            {
  "characters": {
    "pmc": {
      "Inventory": {
        "equipment": "5fe49a0e2694b0755a50476c",
        "fastPanel": {
          "Item4": "7b0724ada2f7f2e45204fb70"
        },
        "items": [
          {
            "_id": "49c6ab1c3e2c828a32c60307",
            "_tpl": "57513f07245977207e26a311",
            "location": {
              "isSearched": true,
              "r": "Horizontal",
              "x": 9,
              "y": 0
            },
            "parentId": "5fe49a0e2694b0755a504876",
            "slotId": "hideout",
            "upd": {
               "FoodDrink": {
                "HpPercent": 20
              }
            }
          }
        ]
      }
    }
  }
}"#;

        let item = Item {
            id: "49c6ab1c3e2c828a32c60307".to_string(),
            tpl: "57513f07245977207e26a311".to_string(),
            x: 0,
            y: 0,
            size_x: 0,
            size_y: 0,
            amount: 0,
            stack_max_size: 0,
            is_stockable: false,
            is_fir: false,
            r: "".to_string(),
            resource: None,
            max_resource: None,
            background_color: "".to_string(),
            is_container: false,
            grid_items: None,
        };
        let bsg_items_root: HashMap<String, Value> = serde_json::from_str(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/Aki_Data/Server/database/templates/items.json"
            ))
            .as_ref(),
        )
        .unwrap();

        let updated = update_durability(json, &item, &bsg_items_root).unwrap();

        let root: Value = serde_json::from_str(updated.as_str()).unwrap();

        let usages: Option<u64> = root
            .get("characters")
            .and_then(|v| v.get("pmc"))
            .and_then(|v| v.get("Inventory"))
            .and_then(|v| v.get("items"))
            .and_then(|v| v.get(0))
            .and_then(|v| v.get("upd"))
            .and_then(|v| v.get("FoodDrink"))
            .and_then(|v| v.get("HpPercent").unwrap().as_u64());

        assert_eq!(usages.unwrap(), 40);
    }
    #[test]
    fn should_update_med_kit() {
        let json = r#"
            {
  "characters": {
    "pmc": {
      "Inventory": {
        "equipment": "5fe49a0e2694b0755a50476c",
        "fastPanel": {
          "Item4": "7b0724ada2f7f2e45204fb70"
        },
        "items": [
          {
            "_id": "49c6ab1c3e2c828a32c60307",
            "_tpl": "590c661e86f7741e566b646a",
            "location": {
              "isSearched": true,
              "r": "Horizontal",
              "x": 9,
              "y": 0
            },
            "parentId": "5fe49a0e2694b0755a504876",
            "slotId": "hideout",
            "upd": {
              "MedKit": {
                "HpResource": 10
              }
            }
          }
        ]
      }
    }
  }
}"#;

        let item = Item {
            id: "49c6ab1c3e2c828a32c60307".to_string(),
            tpl: "590c661e86f7741e566b646a".to_string(),
            x: 0,
            y: 0,
            size_x: 0,
            size_y: 0,
            amount: 0,
            stack_max_size: 0,
            is_stockable: false,
            is_fir: false,
            r: "".to_string(),
            resource: None,
            max_resource: None,
            background_color: "".to_string(),
            is_container: false,
            grid_items: None,
        };
        let bsg_items_root: HashMap<String, Value> = serde_json::from_str(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/Aki_Data/Server/database/templates/items.json"
            ))
            .as_ref(),
        )
        .unwrap();

        let updated = update_durability(json, &item, &bsg_items_root).unwrap();

        let root: Value = serde_json::from_str(updated.as_str()).unwrap();

        let usages: Option<u64> = root
            .get("characters")
            .and_then(|v| v.get("pmc"))
            .and_then(|v| v.get("Inventory"))
            .and_then(|v| v.get("items"))
            .and_then(|v| v.get(0))
            .and_then(|v| v.get("upd"))
            .and_then(|v| v.get("MedKit"))
            .and_then(|v| v.get("HpResource").unwrap().as_u64());

        assert_eq!(usages.unwrap(), 220);
    }
    #[test]
    fn should_update_resource() {
        let json = r#"
            {
  "characters": {
    "pmc": {
      "Inventory": {
        "equipment": "5fe49a0e2694b0755a50476c",
        "fastPanel": {
          "Item4": "7b0724ada2f7f2e45204fb70"
        },
        "items": [
          {
            "_id": "49c6ab1c3e2c828a32c60307",
            "_tpl": "5d1b36a186f7742523398433",
            "location": {
              "isSearched": true,
              "r": "Horizontal",
              "x": 9,
              "y": 0
            },
            "parentId": "5fe49a0e2694b0755a504876",
            "slotId": "hideout",
            "upd": {
              "Resource": {
                "Value": 0
              }
            }
          }
        ]
      }
    }
  }
}"#;

        let item = Item {
            id: "49c6ab1c3e2c828a32c60307".to_string(),
            tpl: "5d1b36a186f7742523398433".to_string(),
            x: 0,
            y: 0,
            size_x: 0,
            size_y: 0,
            amount: 0,
            stack_max_size: 0,
            is_stockable: false,
            is_fir: false,
            r: "".to_string(),
            resource: None,
            max_resource: None,
            background_color: "".to_string(),
            is_container: false,
            grid_items: None,
        };
        let bsg_items_root: HashMap<String, Value> = serde_json::from_str(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/Aki_Data/Server/database/templates/items.json"
            ))
            .as_ref(),
        )
        .unwrap();

        let updated = update_durability(json, &item, &bsg_items_root).unwrap();

        let root: Value = serde_json::from_str(updated.as_str()).unwrap();

        let usages: Option<u64> = root
            .get("characters")
            .and_then(|v| v.get("pmc"))
            .and_then(|v| v.get("Inventory"))
            .and_then(|v| v.get("items"))
            .and_then(|v| v.get(0))
            .and_then(|v| v.get("upd"))
            .and_then(|v| v.get("Resource"))
            .and_then(|v| v.get("Value").unwrap().as_u64());

        assert_eq!(usages.unwrap(), 100);
    }
    #[test]
    fn should_update_repairable() {
        let json = r#"
            {
  "characters": {
    "pmc": {
      "Inventory": {
        "equipment": "5fe49a0e2694b0755a50476c",
        "fastPanel": {
          "Item4": "7b0724ada2f7f2e45204fb70"
        },
        "items": [
          {
            "_id": "49c6ab1c3e2c828a32c60307",
            "_tpl": "606dae0ab0e443224b421bb7",
            "location": {
              "isSearched": true,
              "r": "Horizontal",
              "x": 9,
              "y": 0
            },
            "parentId": "5fe49a0e2694b0755a504876",
            "slotId": "hideout",
            "upd": {
               "Repairable": {
                "Durability": 53.5770874,
                "MaxDurability": 92
              }
            }
          }
        ]
      }
    }
  }
}"#;

        let item = Item {
            id: "49c6ab1c3e2c828a32c60307".to_string(),
            tpl: "606dae0ab0e443224b421bb7".to_string(),
            x: 0,
            y: 0,
            size_x: 0,
            size_y: 0,
            amount: 0,
            stack_max_size: 0,
            is_stockable: false,
            is_fir: false,
            r: "".to_string(),
            resource: None,
            max_resource: None,
            background_color: "".to_string(),
            is_container: false,
            grid_items: None,
        };
        let bsg_items_root: HashMap<String, Value> = serde_json::from_str(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/Aki_Data/Server/database/templates/items.json"
            ))
            .as_ref(),
        )
        .unwrap();

        let updated = update_durability(json, &item, &bsg_items_root).unwrap();

        let root: Value = serde_json::from_str(updated.as_str()).unwrap();

        let repairable = root
            .get("characters")
            .and_then(|v| v.get("pmc"))
            .and_then(|v| v.get("Inventory"))
            .and_then(|v| v.get("items"))
            .and_then(|v| v.get(0))
            .and_then(|v| v.get("upd"))
            .and_then(|v| v.get("Repairable"));

        let durability: Option<u64> =
            repairable.and_then(|v| v.get("Durability").unwrap().as_u64());
        let max_durability: Option<u64> =
            repairable.and_then(|v| v.get("MaxDurability").unwrap().as_u64());

        assert_eq!(durability.unwrap(), 100);
        assert_eq!(max_durability.unwrap(), 100);
    }

    #[test]
    fn should_update_key() {
        let json = r#"
            {
  "characters": {
    "pmc": {
      "Inventory": {
        "equipment": "5fe49a0e2694b0755a50476c",
        "fastPanel": {
          "Item4": "7b0724ada2f7f2e45204fb70"
        },
        "items": [
          {
            "_id": "49c6ab1c3e2c828a32c60307",
            "_tpl": "5780cf942459777df90dcb72",
            "location": {
              "isSearched": true,
              "r": "Horizontal",
              "x": 9,
              "y": 0
            },
            "parentId": "5fe49a0e2694b0755a504876",
            "slotId": "hideout",
            "upd": {
              "Key": {
                "NumberOfUsages": 10
              }
            }
          }
        ]
      }
    }
  }
}"#;

        let item = Item {
            id: "49c6ab1c3e2c828a32c60307".to_string(),
            tpl: "".to_string(),
            x: 0,
            y: 0,
            size_x: 0,
            size_y: 0,
            amount: 0,
            stack_max_size: 0,
            is_stockable: false,
            is_fir: false,
            r: "".to_string(),
            resource: None,
            max_resource: None,
            background_color: "".to_string(),
            is_container: false,
            grid_items: None,
        };

        let bsg_items_root: HashMap<String, Value> = serde_json::from_str(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/Aki_Data/Server/database/templates/items.json"
            ))
            .as_ref(),
        )
        .unwrap();
        let updated = update_durability(json, &item, &bsg_items_root).unwrap();

        let root: Value = serde_json::from_str(updated.as_str()).unwrap();

        let usages: Option<u64> = root
            .get("characters")
            .and_then(|v| v.get("pmc"))
            .and_then(|v| v.get("Inventory"))
            .and_then(|v| v.get("items"))
            .and_then(|v| v.get(0))
            .and_then(|v| v.get("upd"))
            .and_then(|v| v.get("Key"))
            .and_then(|v| v.get("NumberOfUsages").unwrap().as_u64());

        assert_eq!(usages.unwrap(), 0);
    }

    #[test]
    fn should_delete_item() {
        let json = r#"
            {
  "characters": {
    "pmc": {
      "Inventory": {
        "InsuredItems": [
            {
			    "tid": "54cb50c76803fa8b248b4571",
				"itemId": "e87f54571e3fd5233c878f75"
            }
        ],
        "items": [
        {
            "_id": "random item",
            "_tpl": "",
            "parentId": "",
            "slotId": ""
          },
          {
            "_id": "e87f54571e3fd5233c878f75",
            "_tpl": "5ba26586d4351e44f824b340",
            "parentId": "1a0a1821a961b3f54ba4edaf",
            "slotId": "mod_magazine"
          },
          {
			"_id": "b3d6147f5f858786cd1f3e89",
			"_tpl": "5ba26844d4351e00334c9475",
			"parentId": "e87f54571e3fd5233c878f75",
			"slotId": "cartridges",
			"location": 0,
			"upd": {
				"StackObjectsCount": 27
				}
			}
        ]
      }
    }
  }
}"#;

        let item = Item {
            id: "e87f54571e3fd5233c878f75".to_string(),
            tpl: "".to_string(),
            x: 0,
            y: 0,
            size_x: 0,
            size_y: 0,
            amount: 0,
            stack_max_size: 0,
            is_stockable: false,
            is_fir: false,
            r: "".to_string(),
            resource: None,
            max_resource: None,
            background_color: "".to_string(),
            is_container: false,
            grid_items: None,
        };

        let bsg_items_root: HashMap<String, Value> = serde_json::from_str(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/Aki_Data/Server/database/templates/items.json"
            ))
            .as_ref(),
        )
        .unwrap();
        let updated = delete_item(json, &item, &bsg_items_root).unwrap();

        assert_eq!(
            updated,
            r#"{"characters":{"pmc":{"Inventory":{"InsuredItems":[],"items":[{"_id":"random item","_tpl":"","parentId":"","slotId":""}]}}}}"#
        );
    }
}
