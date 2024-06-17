use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use log::{info, warn};
use serde_json::{Map, Value};

use crate::spt::spt_profile_serializer::InventoryItem;

pub fn load_cache_icon_index_file(server_path: &str) -> Option<Map<String, Value>> {
    let cache_index_file = get_cache_index_path(server_path);
    let exists = cache_index_file.exists();
    if !exists {
        warn!(
            "Couldn't find images cache file: {}",
            cache_index_file.display()
        );
        None
    } else {
        let index_json_result = fs::read_to_string(cache_index_file);
        match index_json_result {
            Ok(index_json) => {
                let index_json_value_result: serde_json::Result<Value> =
                    serde_json::from_str(index_json.as_str());
                match index_json_value_result {
                    Ok(index_json_value) => {
                        info!("Loaded images cache file");
                        Some(index_json_value.as_object().unwrap().clone())
                    }
                    Err(e) => {
                        warn!("Couldn't parse cache file to Value: {}", e);
                        None
                    }
                }
            }
            Err(e) => {
                warn!("Error while reading cache file: {}", e);
                None
            }
        }
    }
}

pub fn load_image_from_cache(
    item: &InventoryItem,
    items: &[InventoryItem],
    bsg_items_root: &HashMap<String, Value>,
    index_cache: &Map<String, Value>,
    server_path: &str,
) -> Option<String> {
    let hash = get_item_hash(item, items, bsg_items_root);
    match index_cache.get(hash.to_string().as_str()) {
        Some(index) => load_image(server_path, index.as_u64().unwrap().to_string().as_str()),
        None => {
            warn!(
                "Couldn't find hash {} for item {} in cache index.json",
                hash, item._id
            );
            None
        }
    }
}

fn get_cache_index_path(server_path: &str) -> PathBuf {
    get_cache_path(server_path).join("index.json")
}

fn get_cache_path(server_path: &str) -> PathBuf {
    Path::new(server_path)
        .join("user")
        .join("sptappdata")
        .join("live")
}

fn load_image(server_path: &str, index_name: &str) -> Option<String> {
    let image_path = get_cache_path(server_path).join(index_name.to_owned() + ".png");
    let mut file_content = Vec::new();
    match File::open(image_path) {
        Ok(mut file) => {
            file.read_to_end(&mut file_content).ok()?;
            let base64_encoded = BASE64_STANDARD.encode(file_content);
            let image_url = format!("data:image/png;base64,{}", base64_encoded);
            Some(image_url)
        }
        Err(e) => {
            warn!("Couldn't load image [{}.png]: {}", index_name, e);
            None
        }
    }
}

fn get_item_hash(
    item: &InventoryItem,
    items: &[InventoryItem],
    bsg_items_root: &HashMap<String, Value>,
) -> i32 {
    let mut hash = 17;
    smethod_0(item, items, bsg_items_root, 1)
        .iter()
        .for_each(|h| hash ^= h);

    if is_ammo_item(&item._tpl, bsg_items_root) {
        hash ^= 27 * 56;
    }

    hash
}

fn smethod_0(
    top_level_item: &InventoryItem,
    items: &[InventoryItem],
    bsg_items_root: &HashMap<String, Value>,
    mut hash_seed: i32,
) -> Vec<i32> {
    let mut hashes = Vec::new();

    hashes.push(smethod_1(top_level_item, items, bsg_items_root).wrapping_mul(hash_seed));

    if is_hide_entrails(top_level_item._tpl.as_str(), bsg_items_root) {
        return hashes;
    }

    hash_seed = hash_seed.wrapping_mul(6529);
    let children_items = get_children(top_level_item, items);
    if !children_items.is_empty() {
        children_items.iter().for_each(|child| {
            let mut num = 0;
            let mut num2 = hash_seed ^ get_hash_sum(child, items);
            if is_slack_slot(top_level_item._tpl.as_str(), bsg_items_root) {
                let num3 = num + 1;
                num = num3;
                num2 ^= 2879 * num;
            }
            hashes.extend(smethod_0(child, items, bsg_items_root, num2));
        })
    }

    hashes.sort();
    hashes
}

fn get_hash_sum(item: &InventoryItem, items: &[InventoryItem]) -> i32 {
    let parent_item = get_parent_item(item, items);
    let container_id = item.slot_id.as_ref();
    let mut num =
        2777_i32.wrapping_mul(get_deterministic_hash_code(container_id.unwrap().as_str()));
    num = num.wrapping_add(
        7901_i32.wrapping_mul(get_deterministic_hash_code(parent_item._tpl.as_str())),
    );
    num
}

fn is_slack_slot(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    let container = bsg_items_root.get(tpl).unwrap();

    container
        .get("_props")
        .unwrap()
        .get("Cartridges")
        .and_then(|i| i.as_array())
        .map(|i| !i.is_empty())
        .unwrap_or(false)
}

fn smethod_1(
    item: &InventoryItem,
    items: &[InventoryItem],
    bsg_items_root: &HashMap<String, Value>,
) -> i32 {
    let mut hash = 0;
    hash ^= get_deterministic_hash_code(item._tpl.as_str());

    // something crazy for togglable items...
    let node = bsg_items_root.get(item._tpl.as_str()).unwrap();
    if let Some(has_hinge) = node.pointer("/_props/HasHinge") {
        if has_hinge.as_bool().unwrap() {
            let is_toggled = item
                .upd
                .as_ref()
                .and_then(|a| a.togglable.clone())
                .map(|a| a.on)
                .unwrap_or(true);
            hash ^= 23 + (if is_toggled { 1 } else { 0 });
        }
    }

    if is_foldable_item(&item._tpl, bsg_items_root) {
        let is_folded: bool = item
            .upd
            .as_ref()
            .and_then(|a| a.foldable.clone())
            .map(|a| a.folded)
            .unwrap_or(false);
        hash ^= (23 + (if is_folded { 1 } else { 0 })) << 1;
    }

    if is_magazine_item(&item._tpl, bsg_items_root) {
        let all_ammo_inside_magazine = items
            .iter()
            .filter(|i| i.parent_id.is_some() && i.parent_id.as_ref().unwrap() == item._id.as_str())
            .fold(0, |acc, i| {
                if i.upd.is_some() {
                    acc + i.upd.as_ref().unwrap().stack_objects_count.unwrap_or(1)
                } else {
                    acc
                }
            });

        let max_visible_ammo = get_max_visible_ammo(
            all_ammo_inside_magazine as u16,
            bsg_items_root
                .get(&item._tpl)
                .unwrap()
                .get("_props")
                .unwrap()
                .get("VisibleAmmoRangesString")
                .unwrap()
                .as_str()
                .unwrap(),
        );
        hash ^= (23 + max_visible_ammo as i32) << 2;
    }
    hash
}

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

fn is_magazine_item(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    find_parent_by_name(bsg_items_root, tpl, "Magazine").is_some()
}

fn is_foldable_item(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    let node = bsg_items_root.get(tpl).unwrap();
    node.get("_props")
        .and_then(|p| p.get("Foldable").and_then(|fold| fold.as_bool()))
        .unwrap_or_default()
}

fn get_parent_item(item: &InventoryItem, items: &[InventoryItem]) -> InventoryItem {
    items
        .iter()
        .find(|i| i._id.as_str() == item.parent_id.as_ref().unwrap())
        .unwrap()
        .clone()
}

fn is_hide_entrails(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    let node = bsg_items_root.get(tpl).unwrap();
    node.get("_props")
        .and_then(|p| p.get("HideEntrails").and_then(|hide| hide.as_bool()))
        .unwrap_or_default()
}

fn get_children(item: &InventoryItem, items: &[InventoryItem]) -> Vec<InventoryItem> {
    items
        .iter()
        .filter(|i| i.parent_id.is_some() && i.parent_id.as_ref().unwrap() == item._id.as_str())
        .cloned()
        .collect()
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
    use std::collections::HashMap;

    use serde_json::Value;

    use crate::spt::spt_profile_serializer::load_profile;
    use crate::utils::cache_utils::{
        get_deterministic_hash_code, get_item_hash, get_max_visible_ammo,
        get_max_visible_ammo_ranges, is_ammo_item,
    };

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
            get_deterministic_hash_code("5448ba0b4bdc2d02308b456c"),
            1091773418
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
    fn should_get_hash_from_ammo() {
        let tarkov_profile = load_profile(
            String::from_utf8_lossy(include_bytes!("../../../example/user/profiles/cache.json"))
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

        let item = tarkov_profile
            .characters
            .pmc
            .inventory
            .items
            .iter()
            .find(|item| item._id == "38e4048691dc0bce5e36a02f")
            .unwrap();

        let hash = get_item_hash(
            item,
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
        );
        assert_eq!(hash, -323410316)
    }

    #[test]
    fn should_get_hash_from_magazine_empty() {
        let tarkov_profile = load_profile(
            String::from_utf8_lossy(include_bytes!("../../../example/user/profiles/cache.json"))
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

        let item = tarkov_profile
            .characters
            .pmc
            .inventory
            .items
            .iter()
            .find(|item| item._id == "19c2209a26048a4038c11508")
            .unwrap();

        let hash = get_item_hash(
            item,
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
        );
        assert_eq!(hash, -1339329798)
    }

    #[test]
    fn should_get_hash_from_magazine_with_1_bullet() {
        let tarkov_profile = load_profile(
            String::from_utf8_lossy(include_bytes!("../../../example/user/profiles/cache.json"))
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

        let item = tarkov_profile
            .characters
            .pmc
            .inventory
            .items
            .iter()
            .find(|item| item._id == "734fd64215f99a7ffe28d2e2")
            .unwrap();

        let hash = get_item_hash(
            item,
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
        );
        assert_eq!(hash, -1812714602)
    }

    #[test]
    fn should_get_hash_from_magazine_with_15_bullet() {
        let tarkov_profile = load_profile(
            String::from_utf8_lossy(include_bytes!("../../../example/user/profiles/cache.json"))
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

        let item = tarkov_profile
            .characters
            .pmc
            .inventory
            .items
            .iter()
            .find(|item| item._id == "12d43ea5f8cc38b062e51d02")
            .unwrap();

        let hash = get_item_hash(
            item,
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
        );
        assert_eq!(hash, -1812714654)
    }

    #[test]
    fn should_get_hash_from_APB_pistol_with_APS() {
        let tarkov_profile = load_profile(
            String::from_utf8_lossy(include_bytes!("../../../example/user/profiles/cache.json"))
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

        let item = tarkov_profile
            .characters
            .pmc
            .inventory
            .items
            .iter()
            .find(|item| item._id == "9bfff772bf10230806c6f31a")
            .unwrap();

        let hash = get_item_hash(
            item,
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
        );
        assert_eq!(hash, -985595754)
    }

    #[test]
    fn should_get_hash_from_APB_pistol_full() {
        let tarkov_profile = load_profile(
            String::from_utf8_lossy(include_bytes!("../../../example/user/profiles/cache.json"))
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

        let item = tarkov_profile
            .characters
            .pmc
            .inventory
            .items
            .iter()
            .find(|item| item._id == "034a21c7945c435e47467760")
            .unwrap();

        let hash = get_item_hash(
            item,
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
        );
        assert_eq!(hash, -655496610)
    }

    #[test]
    fn should_get_hash_from_AK74M_not_folded() {
        let tarkov_profile = load_profile(
            String::from_utf8_lossy(include_bytes!("../../../example/user/profiles/cache.json"))
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

        let item = tarkov_profile
            .characters
            .pmc
            .inventory
            .items
            .iter()
            .find(|item| item._id == "f01ff72491f7ab474fc552e2")
            .unwrap();

        let hash = get_item_hash(
            item,
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
        );
        assert_eq!(hash, 1318880480)
    }

    #[test]
    fn should_get_hash_from_AK74M_folded() {
        let tarkov_profile = load_profile(
            String::from_utf8_lossy(include_bytes!("../../../example/user/profiles/cache.json"))
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

        let item = tarkov_profile
            .characters
            .pmc
            .inventory
            .items
            .iter()
            .find(|item| item._id == "86954af93a5c62d177e95ecf")
            .unwrap();

        let hash = get_item_hash(
            item,
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
        );
        assert_eq!(hash, 1318880510)
    }

    #[test]
    fn should_get_hash_from_exfil() {
        let tarkov_profile = load_profile(
            String::from_utf8_lossy(include_bytes!("../../../example/user/profiles/cache.json"))
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

        let item = tarkov_profile
            .characters
            .pmc
            .inventory
            .items
            .iter()
            .find(|item| item._id == "3c180f98cc8a8dafd53b5c81")
            .unwrap();

        let hash = get_item_hash(
            item,
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
        );
        assert_eq!(hash, 1991976902)
    }

    #[test]
    fn should_get_hash_from_gpnvg() {
        let tarkov_profile = load_profile(
            String::from_utf8_lossy(include_bytes!("../../../example/user/profiles/cache.json"))
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

        let item = tarkov_profile
            .characters
            .pmc
            .inventory
            .items
            .iter()
            .find(|item| item._id == "066450d9409fe3d72804d77b")
            .unwrap();

        let hash = get_item_hash(
            item,
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
        );
        assert_eq!(hash, 1444116778);
    }

    #[test]
    fn should_get_hash_from_exfil_with_thermal_on() {
        let tarkov_profile = load_profile(
            String::from_utf8_lossy(include_bytes!("../../../example/user/profiles/cache.json"))
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

        let item = tarkov_profile
            .characters
            .pmc
            .inventory
            .items
            .iter()
            .find(|item| item._id == "e9ce822f5f533067a2de9dce")
            .unwrap();

        let hash = get_item_hash(
            item,
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
        );
        assert_eq!(hash, -96006026)
    }

    #[test]
    fn should_get_hash_from_exfil_with_thermal_off() {
        let tarkov_profile = load_profile(
            String::from_utf8_lossy(include_bytes!("../../../example/user/profiles/cache.json"))
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

        let item = tarkov_profile
            .characters
            .pmc
            .inventory
            .items
            .iter()
            .find(|item| item._id == "1abb8072c666c0b5ec8765a9")
            .unwrap();

        let hash = get_item_hash(
            item,
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
        );
        assert_eq!(hash, 1165599878)
    }
}
