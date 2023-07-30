use serde_json::Error;
use serde_json::Value;

pub fn update_item_amount(
    file_content: &str,
    item_id: &str,
    stack_objects_count: u32,
) -> Result<String, Error> {
    // TODO check item is compatible with the following code
    let mut root: Value = serde_json::from_str(file_content)?;

    // TODO check error

    let optional_items = root
        .get_mut("characters")
        .and_then(|v| v.get_mut("pmc"))
        .and_then(|v| v.get_mut("Inventory"))
        .and_then(|v| v.get_mut("items"))
        .and_then(|v| v.as_array_mut());

    if let Some(items) = optional_items {
        if let Some(item) = items
            .into_iter()
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
