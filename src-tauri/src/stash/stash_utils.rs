use serde_json::Error;
use serde_json::Value;

pub fn update_item_amount(
    file_content: &str,
    item_id: &str,
    stack_objects_count: u32,
) -> Result<String, Error> {
    let mut root: Value = serde_json::from_str(file_content)?;

    let optional_items = root
        .get_mut("characters")
        .and_then(|v| v.get_mut("pmc"))
        .and_then(|v| v.get_mut("Inventory"))
        .and_then(|v| v.get_mut("items"))
        .and_then(|v| v.as_array_mut());

    if let Some(items) = optional_items {
        if let Some(item) = items
            .iter_mut()
            .find(|i| i.get("_id").unwrap().as_str().unwrap() == item_id)
        {
            if let Some(upd) = item.get_mut("upd") {
                if let Some(value) = upd.get_mut("StackObjectsCount") {
                    // *value = stack_objects_count.into();
                    *value = Value::from(stack_objects_count);
                }
            }
        }
    }

    serde_json::to_string(&root)
}

#[cfg(test)]
mod tests {
    use crate::stash::stash_utils::update_item_amount;
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

        let updated = update_item_amount(json, "41d413738e53a1eaa78d3112", 66).unwrap();
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
}
