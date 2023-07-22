use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Error;

#[derive(Deserialize, Serialize, Debug)]
pub struct TarkovProfile {
    pub info: Info,
    pub characters: Characters,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Info {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Characters {
    pub pmc: PMC,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PMC {
    pub Inventory: Inventory,
    pub Info: PMCInfo,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PMCInfo {
    pub Nickname: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub stash: String,
}

#[derive(Serialize, Debug)]
pub struct Item {
    pub _id: String,
    pub _tpl: String,
    pub parentId: Option<String>,
    pub location: Option<Location>,
    pub slotId: Option<String>,
    pub upd: Option<UPD>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Location {
    Number(u16),
    LocationInStash(LocationInStash),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LocationInStash {
    pub x: u16,
    pub y: u16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UPD {
    pub StackObjectsCount: Option<u32>,
}

impl<'de> Deserialize<'de> for Item {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ItemHelper {
            _id: String,
            _tpl: String,
            parentId: Option<String>,
            location: Option<Location>,
            // Deserialize this field as an option
            slotId: Option<String>,
            upd: Option<UPD>,
        }

        let helper: ItemHelper = ItemHelper::deserialize(deserializer)?;

        let cartridges = "cartridges".to_string();

        // Choose the appropriate variant for the Location enum based on slotId
        let location = match (helper.slotId.as_ref(), helper.location) {
            (Some(cartridges), Some(Location::Number(number))) => {
                // If slotId is "cartridges" and location is a number, use Loc ation::Number
                Some(Location::Number(number))
            }
            (_, Some(location)) => Some(location), // Otherwise, use the provided location variant
            _ => None, // If location is not provided or there is no valid combination, set it to None
        };

        Ok(Item {
            _id: helper._id,
            _tpl: helper._tpl,
            parentId: helper.parentId,
            location,
            slotId: helper.slotId,
            upd: helper.upd,
        })
    }
}

pub fn load_profile(profile_json: &str) -> Result<TarkovProfile, Error> {
    serde_json::from_str(profile_json)
}
