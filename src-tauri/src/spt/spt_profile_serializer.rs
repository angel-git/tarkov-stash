pub use crate::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct TarkovProfile {
    pub info: Info,
    pub characters: Characters,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Info {
    pub id: String,
    pub username: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Characters {
    pub pmc: PMC,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PMC {
    #[serde(rename = "Inventory")]
    pub inventory: Inventory,
    #[serde(rename = "Info")]
    pub info: PMCInfo,
    #[serde(rename = "Bonuses")]
    pub bonuses: Vec<Bonuses>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PMCInfo {
    #[serde(rename = "Nickname")]
    pub nickname: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Inventory {
    pub items: Vec<InventoryItem>,
    pub stash: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct InventoryItem {
    #[serde(rename = "id")]
    pub _id: String,
    #[serde(rename = "tpl")]
    pub _tpl: String,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    pub location: Option<Location>,
    #[serde(rename = "slotId")]
    pub slot_id: Option<String>,
    pub upd: Option<UPD>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Bonuses {
    #[serde(rename = "type")]
    pub t: String,
    #[serde(rename = "value")]
    pub value: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Location {
    Number(u16),
    LocationInStash(LocationInStash),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LocationInStash {
    pub x: u16,
    pub y: u16,
    #[serde(deserialize_with = "deserialize_rotation")]
    pub r: String, // "Horizontal" | "Vertical" | u16
}

#[derive(Deserialize, Serialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct UPD {
    #[serde(rename = "StackObjectsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_objects_count: Option<u32>,
    #[serde(rename = "SpawnedInSession")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spawned_in_session: Option<bool>,
    #[serde(rename = "FoodDrink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub food_drink: Option<FoodDrink>,
    #[serde(rename = "MedKit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub med_kit: Option<MedKit>,
    #[serde(rename = "Resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Resource>,
    #[serde(rename = "Repairable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repairable: Option<Repairable>,
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<Key>,
    #[serde(rename = "Foldable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foldable: Option<Foldable>,
    #[serde(rename = "Togglable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub togglable: Option<Togglable>,
    #[serde(rename = "FireMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fire_mode: Option<FireMode>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct FoodDrink {
    #[serde(rename = "HpPercent")]
    pub hp_percent: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct MedKit {
    #[serde(rename = "HpResource")]
    #[serde(deserialize_with = "deserialize_to_integer")]
    pub hp_resource: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct Resource {
    #[serde(rename = "Value")]
    #[serde(deserialize_with = "deserialize_to_integer")]
    pub value: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct Key {
    #[serde(rename = "NumberOfUsages")]
    pub number_of_usages: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct Foldable {
    #[serde(rename = "Folded")]
    pub folded: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct Togglable {
    #[serde(rename = "On")]
    pub on: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct FireMode {
    #[serde(rename = "FireMode")]
    pub fire_mode: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct Repairable {
    #[serde(rename = "Durability")]
    #[serde(deserialize_with = "deserialize_to_integer")]
    pub durability: u16,
    #[serde(rename = "MaxDurability")]
    #[serde(deserialize_with = "deserialize_to_integer")]
    pub max_durability: u16,
}

fn deserialize_rotation<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let rotation_value: serde_json::Value = Deserialize::deserialize(deserializer)?;

    match rotation_value {
        serde_json::Value::Number(num) if num.as_u64() == Some(0) => Ok("Horizontal".to_string()),
        serde_json::Value::Number(num) if num.as_u64() == Some(1) => Ok("Vertical".to_string()),
        _ => Ok(rotation_value.as_str().unwrap().to_string()),
    }
}

fn deserialize_to_integer<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let value: f64 = Deserialize::deserialize(deserializer)?;
    Ok(value as u16)
}

impl<'de> Deserialize<'de> for InventoryItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ItemHelper {
            _id: String,
            _tpl: String,
            #[serde(rename = "parentId")]
            parent_id: Option<String>,
            location: Option<Location>,
            #[serde(rename = "slotId")]
            slot_id: Option<String>,
            upd: Option<UPD>,
        }

        let helper: ItemHelper = ItemHelper::deserialize(deserializer)?;

        // Choose the appropriate variant for the Location enum based on slotId
        let location = match (helper.slot_id.as_ref(), helper.location) {
            (_, Some(Location::Number(number))) => {
                // If slotId is "cartridges" and location is a number, use Location::Number
                Some(Location::Number(number))
            }
            (_, Some(location)) => Some(location), // Otherwise, use the provided location variant
            _ => None, // If location is not provided or there is no valid combination, set it to None
        };

        Ok(InventoryItem {
            _id: helper._id,
            _tpl: helper._tpl,
            parent_id: helper.parent_id,
            location,
            slot_id: helper.slot_id,
            upd: helper.upd,
        })
    }
}

pub fn load_profile(profile_json: &str) -> Result<TarkovProfile, Error> {
    serde_json::from_str(profile_json)
}
