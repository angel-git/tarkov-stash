use serde_json::Value;
use std::collections::HashMap;

pub fn find_id_from_encyclopedia(
    encyclopedia_id: &str,
    globals: &HashMap<String, Value>,
) -> Option<String> {
    globals
        .get("ItemPresets")
        .and_then(|item_presets| item_presets.as_object())
        .and_then(|objs| {
            objs.values().find(|obj| {
                let option = obj.get("_encyclopedia");
                if let Some(encyclopedia) = option {
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

#[cfg(test)]
mod tests {
    use crate::utils::global_utils::find_id_from_encyclopedia;
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
}
