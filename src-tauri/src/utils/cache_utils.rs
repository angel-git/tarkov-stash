use std::collections::{HashMap, HashSet};

use serde_json::Value;

use crate::spt::spt_profile_serializer::InventoryItem;
use crate::ui_profile::ui_profile_serializer::{find_all_slots_from_parent, SlotItem};

// struct ContainerCollection {
//     _id: String,
//     _name: String,
// }

pub fn get_item_hash(
    item: &InventoryItem,
    items: &[InventoryItem],
    bsg_items_root: &HashMap<String, Value>,
) -> i32 {
    let mut hash = 17;
    smethod_0(item, items, bsg_items_root, 1)
        .iter()
        .for_each(|h| hash ^= h);

    if is_ammo_item(&item._tpl, bsg_items_root) {
        println!("is_ammo_item");
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

    let children_items = get_children(top_level_item, items);

    if !children_items.is_empty() {
        hash_seed = hash_seed.wrapping_mul(6529);
        println!("has_cartridges_or_slots {:?}", children_items);
        let mut num = 0;
        children_items.iter().for_each(|child| {
            // TODO get_has_sum is different for: Grid and AmmoBox

            // TODO get current address

            // TODO no idea how to figure out what hash_sum function to call here, none work properly :)

            let mut num2 = 0;
            // if is_slack_slot(top_level_item._tpl.as_str(), bsg_items_root) {
            //     println!("parent is_slack_slot");
            //
            //     num2 = hash_seed ^ get_hash_sum_slack_slot(child, items, bsg_items_root);
            // }
            // if is_ammo_box(top_level_item._tpl.as_str(), bsg_items_root) {
            //     println!("parent is ammo box");
            //     num2 = hash_seed ^ get_hash_sum_ammo_box(child, items, bsg_items_root);
            // }
            // else {
            // println!("WARNING NO AMMO BOX");
            num2 = hash_seed ^ get_hash_sum(child, items, bsg_items_root);
            // }

            // TODO GClass2579 is StackSlot
            // this is true for magazines
            // if slot is StackSlot {
            if is_slack_slot(top_level_item._tpl.as_str(), bsg_items_root) {
                let num3 = num + 1;
                num = num3;
                num2 ^= 2879 * num;
            }

            // }
            hashes.extend(smethod_0(child, items, bsg_items_root, num2));
        })
    }

    hashes
}

fn get_hash_sum_slack_slot(
    item: &InventoryItem,
    items: &[InventoryItem],
    bsg_items_root: &HashMap<String, Value>,
) -> i32 {
    // int num = 17;
    // num = 391 + ParentItem.Id.GetHashCode();
    // num = num * 23 + ID.GetHashCode();
    // foreach (Item item in _items)
    // {
    //     num = num * 23 + item.GetHashSum();
    // }
    // return num;

    let container = get_parent_item(item, items);
    let mut num = get_base_item_hash_code(&container);
    num = num
        .wrapping_mul(23)
        .wrapping_mul(get_base_item_hash_code(item));
    let sieblings = get_all_sieblings_from_same_parent(&container, items);
    sieblings.iter().for_each(|s| {
        num = (num.wrapping_mul(23)).wrapping_add(get_base_item_hash_code(s));
    });

    num
}
fn get_hash_sum_ammo_box(
    item: &InventoryItem,
    items: &[InventoryItem],
    bsg_items_root: &HashMap<String, Value>,
) -> i32 {
    let container = get_parent_item(item, items);
    let mut num = get_base_item_hash_code(&container);
    let sieblings = get_all_sieblings_from_same_parent(&container, items);
    sieblings.iter().for_each(|s| {
        num = (num.wrapping_mul(23)).wrapping_add(get_base_item_hash_code(s));
    });

    num
}

fn get_hash_sum_stack_slot(
    item: &InventoryItem,
    items: &[InventoryItem],
    bsg_items_root: &HashMap<String, Value>,
) -> i32 {
    let container = get_parent_item(item, items);
    let mut num = get_base_item_hash_code(&container);
    let sieblings = get_all_sieblings_from_same_parent(&container, items);
    sieblings.iter().for_each(|s| {
        num = (num.wrapping_mul(23)).wrapping_add(get_base_item_hash_code(s));
    });

    num
}

fn get_hash_sum(
    item: &InventoryItem,
    items: &[InventoryItem],
    bsg_items_root: &HashMap<String, Value>,
) -> i32 {
    let parent_item = get_parent_item(item, items);
    println!("parent_item {:?}", parent_item);
    // TODO who is container id??
    // let container_id = find_slot_or_cartridge_id_by_name(
    //     parent_item._tpl.as_str(),
    //     item.slot_id.as_ref().unwrap().as_str(),
    //     bsg_items_root,
    // );
    // let container_id = Some(parent_item._id);
    // let container_id = Some(item._id.to_string());
    let container_id = item.slot_id.as_ref();
    if container_id.is_none() {
        println!("WARNING container_id.is_none");
        return 1;
    }
    println!(
        "found container_id {}",
        container_id.clone().unwrap().as_str()
    );
    let mut num =
        2777_i32.wrapping_mul(get_deterministic_hash_code(container_id.unwrap().as_str()));
    // -1739552888
    println!("num1 {}", num);
    num = num.wrapping_add(
        7901_i32.wrapping_mul(get_deterministic_hash_code(parent_item._tpl.as_str())),
        // -1250640548
    );
    println!("num2 {}", num);

    // int num = 2777 * Container.ID.GetHashCode();
    // string text = Container.ParentItem?.TemplateId;
    // if (text != null)
    // {
    //     num += 7901 * text.GetHashCode();
    // }
    // return num;

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

fn is_ammo_box(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    // TODO
    false
}

fn find_slot_or_cartridge_id_by_name(
    tpl: &str,
    name: &str,
    bsg_items_root: &HashMap<String, Value>,
) -> Option<String> {
    println!("find_slot_or_cartrigde_id_by_name {} {}", tpl, name);

    let container = bsg_items_root.get(tpl).unwrap();
    let has_slots = container
        .get("_props")
        .unwrap()
        .get("Slots")
        .and_then(|i| i.as_array());

    if has_slots.is_some() && !has_slots.unwrap().is_empty() {
        return has_slots
            .unwrap()
            .iter()
            .find(|i| i.get("_name").unwrap().as_str().unwrap() == name)
            .map(|i| i.get("_id").unwrap().as_str().unwrap().to_string());
    }

    let has_cartridges = container
        .get("_props")
        .unwrap()
        .get("Cartridges")
        .and_then(|i| i.as_array());

    if has_cartridges.is_some() && !has_cartridges.unwrap().is_empty() {
        return has_cartridges
            .unwrap()
            .iter()
            .find(|i| i.get("_name").unwrap().as_str().unwrap() == name)
            .map(|i| i.get("_id").unwrap().as_str().unwrap().to_string());
    }

    None
}

// fn find_slot_or_cartrigde_id_by_name_2(
//     item: &InventoryItem,
//     name: &str,
//     bsg_items_root: &HashMap<String, Value>,
// ) -> Option<String> {
//     println!("find_slot_or_cartrigde_id_by_name {} {}", tpl, name);
//
//     let container = bsg_items_root.get(tpl).unwrap();
//     let has_slots = container
//         .get("_props")
//         .unwrap()
//         .get("Slots")
//         .and_then(|i| i.as_array());
//
//     if has_slots.is_some() && !has_slots.unwrap().is_empty() {
//         return has_slots
//             .unwrap()
//             .iter()
//             .find(|i| i.get("_name").unwrap().as_str().unwrap() == name)
//             .map(|i| i.get("_id").unwrap().as_str().unwrap().to_string());
//     }
//
//     let has_cartridges = container
//         .get("_props")
//         .unwrap()
//         .get("Cartridges")
//         .and_then(|i| i.as_array());
//
//     if has_cartridges.is_some() && !has_cartridges.unwrap().is_empty() {
//         return has_cartridges
//             .unwrap()
//             .iter()
//             .find(|i| i.get("_name").unwrap().as_str().unwrap() == name)
//             .map(|i| i.get("_id").unwrap().as_str().unwrap().to_string());
//     }
//
//     None
// }

fn smethod_1(
    item: &InventoryItem,
    items: &[InventoryItem],
    bsg_items_root: &HashMap<String, Value>,
) -> i32 {
    let mut hash = 0;
    hash ^= get_deterministic_hash_code(item._tpl.as_str());
    // TogglableComponent itemComponent = item.GetItemComponent<TogglableComponent>();
    // if (itemComponent != null)
    // {
    //     num ^= 23 + (itemComponent.On ? 1 : 0);
    // }
    // FoldableComponent itemComponent2 = item.GetItemComponent<FoldableComponent>();
    // if (itemComponent2 != null)
    // {
    //     num ^= 23 + (itemComponent2.Folded ? 1 : 0) << 1;
    // }
    // if let Some(foldable_item) = get_foldable_item(item, items, bsg_items_root) {
    //     let is_folded = foldable_item.upd.and_then(|a| a.foldable).map(|a| a.folded).unwrap_or(false);
    //     hash ^= 23 + (if is_folded {1} else {0}) << 1;
    // }

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
        println!("is magazine item, id: {}", &item._id);
        // TODO do we have to calculate here all bullets inside??? i don't think so...
        let all_ammo_inside_magazine = items
            .iter()
            .filter(|i| i.parent_id.is_some() && i.parent_id.as_ref().unwrap() == item._id.as_str())
            .fold(0, |acc, i| {
                acc + i.upd.as_ref().unwrap().stack_objects_count.unwrap()
            });

        println!("all ammo in magazine {}", all_ammo_inside_magazine);

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

        println!("max_visible_ammo {}", max_visible_ammo);

        hash ^= (23 + max_visible_ammo as i32) << 2;
    }
    hash
}

fn get_base_item_hash_code(item: &InventoryItem) -> i32 {
    let stack_objects_count = item.upd.as_ref().unwrap().stack_objects_count.unwrap_or(1);
    println!("stack_objects_count {} {}", item._id, stack_objects_count);
    // TODO this might be item._tpl ?
    (16_i32 + stack_objects_count as i32) * 23_i32 + get_deterministic_hash_code(item._id.as_str())
}
///////////////////

// https://github.com/RatScanner/RatStash/blob/master/RatStash/CacheHashIndexParser.cs
pub fn calculate_item_cache_hashBACKUP(
    item: &InventoryItem,
    items: &[InventoryItem],
    bsg_items_root: &HashMap<String, Value>,
) -> i32 {
    let mut hash = 17;
    hash ^= get_single_item_hash(item, items, bsg_items_root);
    println!("hash 1 {}", hash);
    if is_hide_entrails(&item._tpl, bsg_items_root) {
        return hash;
    }

    let children = get_children(item, items);
    if !children.is_empty() {
        println!("has_cartridges_or_slots {:?}", children);
        // get_cartridges_or_slots_hash(containers)
    }

    // if is_compound_without_hide_entrails(&item._tpl, bsg_items_root) {
    //     println!("is_compound_without_hide_entrails");
    //     hash = get_container_hash(&hash, item, items, bsg_items_root);
    // }
    if is_ammo_item(&item._tpl, bsg_items_root) {
        println!("is_ammo_item");
        hash ^= 27 * 56;
    }
    if is_magazine_item(&item._tpl, bsg_items_root) {
        println!("is_magazine_item");
        hash = get_container_hash(&hash, item, items, bsg_items_root);
    }
    // else {
    // TODO
    // if (item is CompoundItem { HideEntrails: false } compoundItem)
    // {
    //     hash = GetContainerHash(hash, compoundItem.Slots);
    // }
    //
    // if (item is Magazine magazine)
    // {
    //     hash = GetContainerHash(hash, magazine.Cartridges);
    // }
    //
    // if (item is AmmoContainer ammoContainer)
    // {
    //     hash = GetContainerHash(hash, ammoContainer.StackSlots);
    // }
    //
    // if (item is Ammo)
    // {
    //     // Ammo is never used when rendered in the inventory
    //     // hash ^= 27 * (ammo.IsUsed ? 42 : 56);
    hash
}

fn get_single_item_hash(
    item: &InventoryItem,
    items: &[InventoryItem],
    bsg_items_root: &HashMap<String, Value>,
) -> i32 {
    let mut hash = 0;
    // if (item == null) return hash;
    hash ^= get_deterministic_hash_code(&item._tpl);

    if is_weapon_item(&item._tpl, bsg_items_root) {
        println!("is weapon item");
        // TODO folded
        hash ^= (23 + 0) << 1;
    }
    // if is_ammo_item(&item._tpl, bsg_items_root) {
    //     println!("is ammo item");
    // }
    if is_magazine_item(&item._tpl, bsg_items_root) {
        println!("is magazine item, id: {}", &item._id);
        // TODO do we have to calculate here all bullets inside??? i don't think so...
        let all_ammo_inside_magazine = items
            .iter()
            .filter(|i| i.parent_id.is_some() && i.parent_id.as_ref().unwrap() == item._id.as_str())
            .fold(0, |acc, i| {
                acc + i.upd.as_ref().unwrap().stack_objects_count.unwrap()
            });

        println!("all ammo in magazine {}", all_ammo_inside_magazine);

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

        println!("max_visible_ammo {}", max_visible_ammo);

        hash ^= (23 + max_visible_ammo as i32) << 2;
    }

    // TODO
    // switch (item)
    // {
    //     case NightVision:
    //     case ThermalVision:
    //     case ArmoredEquipment:
    //     if (item is ArmoredEquipment { HasHinge: false }) break;
    //     hash ^= 23 + (itemExtraInfo.ItemIsToggled ? 1 : 0);
    //     break;
    //     case Weapon:
    //     hash ^= 23 + (itemExtraInfo.WeaponIsFolded ? 1 : 0) << 1;
    //     break;
    //     case Magazine magazine:
    //     hash ^= 23 + magazine.GetMaxVisibleAmmo() << 2;
    //     break;
    // }
    println!("get_single_item_hash {}", hash);
    hash
}

fn get_container_hash(
    hash: &i32,
    item: &InventoryItem,
    items: &[InventoryItem],
    bsg_items_root: &HashMap<String, Value>,
) -> i32 {
    println!("item {:?}", item);
    let mut hash_clone = *hash;

    let slots_from_item: HashSet<SlotItem> = find_all_slots_from_parent(&item._id, items, "");
    // TODO remove if
    if !slots_from_item.is_empty() {
        println!("slot_items {:?}", slots_from_item);
        slots_from_item.iter().for_each(|slot| {
            let slot_item = items.iter().find(|i| i._id == slot.id).unwrap();
            hash_clone ^= get_deterministic_hash_code(slot.slot_id.as_str());
            // TODO iter for all items inside the slot item? or we already doing here?
            // foreach (var item in container.Items)
            println!("hash_clone for slot {}: {}", slot.slot_id, hash_clone);
            hash_clone ^= get_single_item_hash(slot_item, items, bsg_items_root);
            println!(
                "hash_clone for item inside slot {}: {}",
                slot.slot_id, hash_clone
            );
        });
        hash_clone
    } else {
        println!("no slot items!");
        hash_clone
    }
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

fn is_weapon_item(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    find_parent_by_name(bsg_items_root, tpl, "Weapon").is_some()
}

fn is_magazine_item(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    find_parent_by_name(bsg_items_root, tpl, "Magazine").is_some()
}

fn is_foldable_item(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    let node = bsg_items_root.get(tpl).unwrap();
    if let Some(fold) = node
        .get("_props")
        .and_then(|p| p.get("Foldable").and_then(|fold| fold.as_bool()))
    {
        fold
    } else {
        false
    }
}

// fn is_folded_item(item: &InventoryItem) -> bool {
//     item.upd
//     let node = bsg_items_root.get(tpl).unwrap();
//     if let Some(fold) = node
//         .get("_props")
//         .and_then(|p| p.get("Foldable").and_then(|fold| fold.as_bool()))
//     {
//         fold
//     } else {
//         false
//     }
// }

fn get_foldable_item(
    item: &InventoryItem,
    items: &[InventoryItem],
    bsg_items_root: &HashMap<String, Value>,
) -> Option<InventoryItem> {
    let node = bsg_items_root.get(item._tpl.as_str()).unwrap();

    if let Some(foldedSlot) = node.pointer("/_props/FoldedSlot") {
        let attachments = get_all_sieblings_from_same_parent(item, items);
        if let Some(foldable_item) = attachments.iter().find(|a| {
            a.slot_id.as_ref().is_some()
                && a.slot_id.as_ref().unwrap() == foldedSlot.as_str().unwrap()
        }) {
            Some(foldable_item.clone())
        } else {
            None
        }
    } else {
        return None;
    }
}
//
// // TODO delete?
// fn is_inside_slot(item: &InventoryItem) -> bool {
//     item.slot_id.is_some() && item.slot_id.as_ref().unwrap().as_str() != "hideout"
// }
// // TODO delete?
fn get_parent_item(item: &InventoryItem, items: &[InventoryItem]) -> InventoryItem {
    items
        .iter()
        .find(|i| i._id.as_str() == item.parent_id.as_ref().unwrap())
        .unwrap()
        .clone()
}

fn get_all_sieblings_from_same_parent(
    parent: &InventoryItem,
    items: &[InventoryItem],
) -> Vec<InventoryItem> {
    println!("get_all_sieblings_from_same_parent {}", parent._id);
    items
        .iter()
        .filter(|i| i.parent_id.is_some() && i.parent_id.as_ref().unwrap().as_str() == parent._id)
        .cloned()
        .collect()
}

fn is_hide_entrails(tpl: &str, bsg_items_root: &HashMap<String, Value>) -> bool {
    let node = bsg_items_root.get(tpl).unwrap();
    if let Some(hide) = node
        .get("_props")
        .and_then(|p| p.get("HideEntrails").and_then(|hide| hide.as_bool()))
    {
        hide
    } else {
        false
    }
}

// fn get_cartridges_or_slots(
//     tpl: &str,
//     bsg_items_root: &HashMap<String, Value>,
// ) -> Vec<ContainerCollection> {
//     let node = bsg_items_root.get(tpl).unwrap();
//     let has_slots = node
//         .get("_props")
//         .unwrap()
//         .get("Slots")
//         .and_then(|i| i.as_array());
//
//     if has_slots.is_some() && !has_slots.unwrap().is_empty() {
//         return has_slots
//             .unwrap()
//             .iter()
//             .map(|i| ContainerCollection {
//                 _id: i.get("_id").unwrap().as_str().unwrap().to_string(),
//                 _name: i.get("_name").unwrap().as_str().unwrap().to_string(),
//             })
//             .collect();
//     }
//
//     let has_cartridges = node
//         .get("_props")
//         .unwrap()
//         .get("Cartridges")
//         .and_then(|i| i.as_array());
//
//     if has_cartridges.is_some() && !has_cartridges.unwrap().is_empty() {
//         return has_cartridges
//             .unwrap()
//             .iter()
//             .map(|i| ContainerCollection {
//                 _id: i.get("_id").unwrap().as_str().unwrap().to_string(),
//                 _name: i.get("_name").unwrap().as_str().unwrap().to_string(),
//             })
//             .collect();
//     }
//
//     Vec::new()
// }
//
// fn get_cartridges_or_slots_hash(containers: &[ContainerCollection], mut hash_seed: i32) -> i32 {
//     let num = 0;
//     hash_seed *= 6529;
//     containers.iter().for_each(|c| {})
// }

fn get_children(item: &InventoryItem, items: &[InventoryItem]) -> Vec<InventoryItem> {
    items
        .iter()
        .filter(|i| i.parent_id.is_some() && i.parent_id.as_ref().unwrap() == item._id.as_str())
        .cloned()
        .collect()
}

// TODO delete?
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
    use std::collections::HashMap;

    use serde_json::Value;

    use crate::spt::spt_profile_serializer::load_profile;
    use crate::utils::cache_utils::{
        calculate_item_cache_hashBACKUP, get_deterministic_hash_code, get_item_hash,
        get_max_visible_ammo, get_max_visible_ammo_ranges, is_ammo_item,
        is_compound_without_hide_entrails,
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

    #[test]
    fn should_get_hash_from_ammo() {
        // https://assets.tarkov.dev/54527a984bdc2d4e668b4567-base-image.png

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

        let ammo = tarkov_profile
            .characters
            .pmc
            .inventory
            .items
            .iter()
            .find(|item| item._id == "38e4048691dc0bce5e36a02f")
            .unwrap();

        let hash = get_item_hash(
            ammo,
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
        );
        // index.json
        // 3
        // -323410316

        assert_eq!(hash, -323410316)
    }

    #[test]
    fn should_get_hash_from_magazine_empty() {
        // 82
        // -1339329798

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

        let ammo = tarkov_profile
            .characters
            .pmc
            .inventory
            .items
            .iter()
            .find(|item| item._id == "19c2209a26048a4038c11508")
            .unwrap();

        let hash = get_item_hash(
            ammo,
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
        );
        assert_eq!(hash, -1339329798)
    }

    #[test]
    fn should_get_hash_from_magazine_with_1_bullet() {
        // 82
        // -1339329798

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

        let ammo = tarkov_profile
            .characters
            .pmc
            .inventory
            .items
            .iter()
            .find(|item| item._id == "734fd64215f99a7ffe28d2e2")
            .unwrap();

        let hash = get_item_hash(
            ammo,
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
        );
        assert_eq!(hash, -1812714602)
    }

    #[test]
    fn should_get_hash_from_magazine_with_15_bullet() {
        // 82
        // -1339329798

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

        let ammo = tarkov_profile
            .characters
            .pmc
            .inventory
            .items
            .iter()
            .find(|item| item._id == "12d43ea5f8cc38b062e51d02")
            .unwrap();

        let hash = get_item_hash(
            ammo,
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

        let ammo = tarkov_profile
            .characters
            .pmc
            .inventory
            .items
            .iter()
            .find(|item| item._id == "9bfff772bf10230806c6f31a")
            .unwrap();

        let hash = get_item_hash(
            ammo,
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

        let ammo = tarkov_profile
            .characters
            .pmc
            .inventory
            .items
            .iter()
            .find(|item| item._id == "034a21c7945c435e47467760")
            .unwrap();

        let hash = get_item_hash(
            ammo,
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

        let ammo = tarkov_profile
            .characters
            .pmc
            .inventory
            .items
            .iter()
            .find(|item| item._id == "f01ff72491f7ab474fc552e2")
            .unwrap();

        let hash = get_item_hash(
            ammo,
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

        let ammo = tarkov_profile
            .characters
            .pmc
            .inventory
            .items
            .iter()
            .find(|item| item._id == "86954af93a5c62d177e95ecf")
            .unwrap();

        let hash = get_item_hash(
            ammo,
            &tarkov_profile.characters.pmc.inventory.items,
            &bsg_items_root,
        );
        assert_eq!(hash, 1318880510)
    }
}
