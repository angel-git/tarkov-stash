use std::collections::HashMap;

use crate::prelude::*;

pub fn find_id_from_encyclopedia(
    encyclopedia_id: &str,
    globals: &HashMap<String, Value>,
) -> Option<String> {
    globals
        .get("ItemPresets")
        .and_then(|item_presets| item_presets.as_object())
        .and_then(|objs| {
            objs.values().find(|obj| {
                if let Some(encyclopedia) = obj.get("_encyclopedia") {
                    encyclopedia.as_str().unwrap() == encyclopedia_id
                } else {
                    false
                }
            })
        })
        .and_then(|preset| preset.as_object().unwrap().get("_id"))
        .and_then(|id| id.as_str())
        .map(|s| s.to_string())
}

pub fn find_all_item_presets(
    globals: &HashMap<String, Value>,
    bsg_items_root: &HashMap<String, Value>,
) -> Vec<PresetItem> {
    let items: Option<Vec<PresetItem>> = globals
        .get("ItemPresets")
        .and_then(|item_presets| item_presets.as_object())
        .map(|objs| {
            objs.values()
                .map(|obj| {
                    let mut width = 0;
                    let mut height = 0;
                    let items: Vec<spt_profile_serializer::InventoryItem> =
                        serde_json::from_value(obj.get("_items").unwrap().clone()).unwrap();

                    let mut encyclopedia = None;
                    if let Some(enc) = obj.get("_encyclopedia") {
                        encyclopedia = Some(enc.as_str().unwrap().to_string());
                        let main_item = items
                            .iter()
                            .find(|i| i._tpl == encyclopedia.clone().unwrap())
                            .unwrap();

                        (width, height) = item_utils::calculate_item_size(
                            main_item,
                            &items,
                            bsg_items_root,
                            false,
                        );
                    }

                    PresetItem {
                        id: obj.get("_id").unwrap().as_str().unwrap().to_string(),
                        items,
                        width,
                        height,
                        encyclopedia,
                    }
                })
                .filter(|i| i.encyclopedia.is_some())
                .collect::<Vec<PresetItem>>()
        });

    items.unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use crate::utils::global_utils::{find_all_item_presets, find_id_from_encyclopedia};
    use serde_json::Value;
    use std::collections::HashMap;

    #[test]
    fn should_find_id_from_a_known_encyclopedia() {
        let globals: HashMap<String, Value> = serde_json::from_str(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/Aki_Data/Server/database/globals.json"
            ))
            .as_ref(),
        )
        .unwrap();
        let id = find_id_from_encyclopedia("5cadc190ae921500103bb3b6", &globals);
        assert_eq!(id, Some("5d3f0bc986f7743cb332abdc".to_string()));
    }

    #[test]
    fn should_not_find_id_from_a_unknown_encyclopedia() {
        let globals: HashMap<String, Value> = serde_json::from_str(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/Aki_Data/Server/database/globals.json"
            ))
            .as_ref(),
        )
        .unwrap();
        let id = find_id_from_encyclopedia("fake", &globals);
        assert_eq!(id, None);
    }

    #[test]
    fn should_find_all_presets_with_encyclopedia() {
        let globals: HashMap<String, Value> = serde_json::from_str(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/Aki_Data/Server/database/globals.json"
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

        let all_presets = find_all_item_presets(&globals, &bsg_items_root);
        assert_eq!(all_presets.len(), 122);
    }

    #[test]
    fn should_calculate_correct_size_of_preset() {
        let globals: HashMap<String, Value> = serde_json::from_str(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/Aki_Data/Server/database/globals.json"
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

        let all_presets = find_all_item_presets(&globals, &bsg_items_root);
        let aks = all_presets
            .iter()
            .find(|i| i.id == "584147ed2459775a77263501")
            .unwrap();
        assert_eq!(aks.height, 2);
        assert_eq!(aks.width, 4);
    }
}
