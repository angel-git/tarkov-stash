use serde_json::Value;
use std::collections::HashMap;

fn get_max_visible_ammo(bullets: u16, visible_ammo_ranges_string: &str) -> u16 {
    let visible_ammo_ranges = get_max_visible_ammo_ranges(visible_ammo_ranges_string);

    let mut i = 0;
    while i < visible_ammo_ranges.len() {
        let (start, end) = visible_ammo_ranges[i];
        if start <= bullets && end >= bullets {
            return bullets;
        }
        if bullets >= start {
            i += 1;
        } else {
            return if i != 0 {
                visible_ammo_ranges[i - 1].1
            } else {
                start
            };
        }
    }

    visible_ammo_ranges[visible_ammo_ranges.len() - 1].1
}
fn get_max_visible_ammo_ranges(visible_ammo_ranges_string: &str) -> Vec<(u16, u16)> {
    if visible_ammo_ranges_string.is_empty() || visible_ammo_ranges_string.trim().is_empty() {
        return vec![(1, 2)];
    }

    let mut ranges = Vec::new();
    let splits: Vec<&str> = visible_ammo_ranges_string.split(';').collect();

    for split in splits {
        let range: Vec<&str> = split.split('-').collect();
        let start = range[0].parse().unwrap();
        let end = range[1].parse().unwrap();
        ranges.push((start, end));
    }

    ranges
}

fn get_deterministic_hash_code(s: &str) -> i32 {
    let mut hash1: i32 = 5381;
    let mut hash2: i32 = hash1;

    for (i, c) in s.chars().enumerate().step_by(2) {
        hash1 = hash1.wrapping_shl(5).wrapping_add(hash1) ^ c as i32;

        if i == s.len() - 1 {
            break;
        }
        let next_char = s.chars().nth(i + 1).unwrap();
        hash2 = hash2.wrapping_shl(5).wrapping_add(hash2) ^ next_char as i32
    }

    hash1.wrapping_add(hash2.wrapping_mul(1566083941))
}

fn is_ammo_item(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    find_parent_by_name(bsg_items_root, tpl, "Ammo").is_some()
}

fn is_weapon_item(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    find_parent_by_name(bsg_items_root, tpl, "Weapon").is_some()
}

fn is_magazine_item(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    find_parent_by_name(bsg_items_root, tpl, "Magazine").is_some()
}

fn is_compound_without_hide_entrails(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
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
    use crate::utils::cache_utils::{
        get_deterministic_hash_code, get_max_visible_ammo, get_max_visible_ammo_ranges,
        is_ammo_item, is_compound_without_hide_entrails,
    };
    use serde_json::Value;
    use std::collections::HashMap;

    #[test]
    fn should_get_max_visible_ammo_ranges() {
        let bsg_items_root: HashMap<String, Value> = serde_json::from_str(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/Aki_Data/Server/database/templates/items.json"
            ))
            .as_ref(),
        )
        .unwrap();

        let visible_ammo_ranges_string_5a17fb03fcdbcbcae668728f = bsg_items_root
            .get("5a17fb03fcdbcbcae668728f")
            .unwrap()
            .get("_props")
            .unwrap()
            .get("VisibleAmmoRangesString")
            .unwrap()
            .as_str()
            .unwrap();

        let visible_ammo_ranges_string_5a3501acc4a282000d72293a = bsg_items_root
            .get("5a3501acc4a282000d72293a")
            .unwrap()
            .get("_props")
            .unwrap()
            .get("VisibleAmmoRangesString")
            .unwrap()
            .as_str()
            .unwrap();

        assert_eq!(
            get_max_visible_ammo_ranges(visible_ammo_ranges_string_5a17fb03fcdbcbcae668728f).len(),
            10
        );
        assert_eq!(
            get_max_visible_ammo_ranges(visible_ammo_ranges_string_5a3501acc4a282000d72293a).len(),
            1
        );
    }

    #[test]
    fn should_get_max_visible_ammo_for_5a17fb03fcdbcbcae668728f() {
        // https://assets.tarkov.dev/5a17fb03fcdbcbcae668728f-base-image.png

        let bsg_items_root: HashMap<String, Value> = serde_json::from_str(
            String::from_utf8_lossy(include_bytes!(
                "../../../example/Aki_Data/Server/database/templates/items.json"
            ))
            .as_ref(),
        )
        .unwrap();

        let visible_ammo_ranges_string_5a17fb03fcdbcbcae668728f = bsg_items_root
            .get("5a17fb03fcdbcbcae668728f")
            .unwrap()
            .get("_props")
            .unwrap()
            .get("VisibleAmmoRangesString")
            .unwrap()
            .as_str()
            .unwrap();

        assert_eq!(
            get_max_visible_ammo(0, visible_ammo_ranges_string_5a17fb03fcdbcbcae668728f),
            1
        );

        assert_eq!(
            get_max_visible_ammo(1, visible_ammo_ranges_string_5a17fb03fcdbcbcae668728f),
            1
        );

        assert_eq!(
            get_max_visible_ammo(2, visible_ammo_ranges_string_5a17fb03fcdbcbcae668728f),
            1
        );
        assert_eq!(
            get_max_visible_ammo(3, visible_ammo_ranges_string_5a17fb03fcdbcbcae668728f),
            1
        );
        assert_eq!(
            get_max_visible_ammo(4, visible_ammo_ranges_string_5a17fb03fcdbcbcae668728f),
            4
        );
        assert_eq!(
            get_max_visible_ammo(20, visible_ammo_ranges_string_5a17fb03fcdbcbcae668728f),
            20
        );
    }

    #[test]
    fn should_get_deterministic_hash_code() {
        assert_eq!(
            get_deterministic_hash_code("f5b4f4741134ec027112e83b"),
            1795904484
        );
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
