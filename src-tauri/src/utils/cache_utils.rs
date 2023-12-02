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

#[cfg(test)]
mod tests {
    use crate::utils::cache_utils::{get_max_visible_ammo, get_max_visible_ammo_ranges};
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

        // https://assets.tarkov.dev/5a17fb03fcdbcbcae668728f-base-image.png
    }
}
