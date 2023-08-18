use crate::ui_profile::ui_profile_serializer::Item;
use serde_json::{json, Error, Value};

pub fn update_item_amount(file_content: &str, item: &Item) -> Result<String, Error> {
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

pub fn update_spawned_in_session(file_content: &str, item: &Item) -> Result<String, Error> {
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

fn get_inventory_items(root: &mut Value) -> Option<&mut Vec<Value>> {
    root.get_mut("characters")
        .and_then(|v| v.get_mut("pmc"))
        .and_then(|v| v.get_mut("Inventory"))
        .and_then(|v| v.get_mut("items"))
        .and_then(|v| v.as_array_mut())
}

#[cfg(test)]
mod tests {
    use crate::stash::stash_utils::{update_item_amount, update_spawned_in_session};
    use crate::ui_profile::ui_profile_serializer::Item;
    use serde_json::Value;

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
            is_stockable: false,
            is_fir: false,
            r: "Horitzontal".to_string(),
        };
        let updated = update_item_amount(json, &item).unwrap();
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
            is_stockable: false,
            is_fir: false,
            r: "Horitzontal".to_string(),
        };
        let item_3eb9393a58f797c0c5f0e38e = Item {
            id: "3eb9393a58f797c0c5f0e38e".to_string(),
            tpl: "590c37d286f77443be3d7827".to_string(),
            x: 0,
            y: 0,
            size_x: 0,
            size_y: 0,
            amount: 66,
            is_stockable: false,
            is_fir: false,
            r: "Horitzontal".to_string(),
        };
        // update both items
        let update_41d413738e53a1eaa78d3112 =
            update_spawned_in_session(json, &item_41d413738e53a1eaa78d3112).unwrap();
        let update_3eb9393a58f797c0c5f0e38e = update_spawned_in_session(
            update_41d413738e53a1eaa78d3112.as_str(),
            &item_3eb9393a58f797c0c5f0e38e,
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
}
