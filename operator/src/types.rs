use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cmp::{max, min};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::ops::Add;
use std::{fmt::Display, hash::Hasher, sync::Arc};
use thiserror::Error;

use crate::data::McData;
use crate::state::{holds::Hold, State};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Vec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Dimension {
    TheNether,
    Overworld,
    TheEnd,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Location {
    pub vec3: Vec3,
    pub dim: Dimension,
}

impl Location {
    pub fn distance_heuristic(&self, other: &Location) -> i32 {
        if other.dim != self.dim {
            return 1000;
        }

        self.vec3.dist(other.vec3) as i32
    }
}

impl Vec3 {
    pub fn dist(&self, other: Vec3) -> f64 {
        let x_diff = (other.x - self.x) as i64;
        let y_diff = (other.y - self.y) as i64;
        let z_diff = (other.z - self.z) as i64;

        ((x_diff.pow(2) + y_diff.pow(2) + z_diff.pow(2)) as f64).sqrt()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, b: Vec3) -> Self {
        Vec3 {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_fmt(format_args!("({}, {}, {})", self.x, self.y, self.z))
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Vec2 {
    pub x: i32,
    pub z: i32,
}

impl Display for Vec2 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_fmt(format_args!("({}, {})", self.x, self.z))
    }
}

impl From<Vec3> for Vec2 {
    fn from(vec3: Vec3) -> Self {
        Vec2 {
            x: vec3.x,
            z: vec3.z,
        }
    }
}

impl Vec2 {
    pub fn contained_by(&self, a: Vec2, b: Vec2, margin: i32) -> bool {
        let min_x = min(a.x, b.x) - margin;
        let max_x = max(a.x, b.x) + margin;
        let min_z = min(a.z, b.z) - margin;
        let max_z = max(a.z, b.z) + margin;

        if min_x > self.x || max_x < self.x || min_z > self.z || max_z < self.z {
            return false;
        }

        true
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct UnhashedItem {
    pub item_id: u32,
    pub count: u32,
    pub metadata: u32,
    pub nbt: Arc<Value>,
    pub stack_size: u32,
}

lazy_static::lazy_static! {
    static ref MC_DATA: McData = McData::init();
}

impl UnhashedItem {
    pub fn into_item(self) -> Item {
        let stackable_hash = self.stackable_hash();
        let shulker_data = self.shulker_data();

        Item {
            item_id: self.item_id,
            count: self.count,
            metadata: self.metadata,
            nbt: self.nbt,
            stack_size: self.stack_size,
            stackable_hash,
            shulker_data,
        }
    }

    fn stackable_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();

        self.item_id.hash(&mut s);
        self.metadata.hash(&mut s);
        let serialized_nbt = serde_json::to_string(&self.nbt).unwrap();
        serialized_nbt.hash(&mut s);

        s.finish()
    }

    pub fn shulker_data(&self) -> Option<Box<ShulkerData>> {
        let mc_data_item = MC_DATA.items_by_id.get(&self.item_id)?;
        if !mc_data_item.name.ends_with("shulker_box") {
            return None;
        }

        let color = mc_data_item
            .name
            .strip_suffix("_shulker_box")
            .map(|color| color.to_string());

        let name = self
            .nbt
            .pointer("/value/display/value/Name/value")
            .and_then(|nbt_val| nbt_val.as_str())
            .and_then(|nbt_str| serde_json::from_str::<NbtNameStr>(nbt_str).ok())
            .map(|name| name.text);

        let contained_items_nbt = self
            .nbt
            .pointer("/value/BlockEntityTag/value/Items/value/value")
            .and_then(|nbt_val| nbt_val.as_array());

        let empty = contained_items_nbt
            .map(|items_list| items_list.len() == 0)
            .unwrap_or(true);

        let contained_items = contained_items_nbt
            .map(|items_list| {
                items_list
                    .iter()
                    .map(|nbt_item| {
                        let item_mc_name = nbt_item
                            .pointer("/id/value")
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .strip_prefix("minecraft:")
                            .unwrap();
                        let mc_data_item = MC_DATA.items_by_name.get(item_mc_name)?;
                        let count = nbt_item.pointer("/Count/value").unwrap().as_u64().unwrap();

                        const USE_VIA_VERSION_FIX: bool = true;

                        let mut nbt = nbt_item.pointer("/tag");

                        if USE_VIA_VERSION_FIX {
                            // ViaVersion appears to add this NBT tag to everything, regardless of
                            // actual presence
                            match nbt {
                                Some(Value::Object(tag_map)) => {
                                    if tag_map.len() == 2
                                        && tag_map
                                            .get("type")
                                            .map_or(false, |val| val.as_str() == Some("compound"))
                                        && tag_map.get("value").map_or(false, |val| {
                                            val.as_object().unwrap().len() == 0
                                        })
                                    {
                                        nbt = None
                                    }
                                }
                                _ => {}
                            }
                        }

                        let nbt = nbt
                            .map(|tag| match tag {
                                Value::Object(tag_map) => {
                                    let mut tag_map = tag_map.clone();

                                    tag_map.insert(
                                        String::from("name"),
                                        Value::String(String::from("")),
                                    );

                                    Value::Object(tag_map)
                                }
                                _ => tag.clone(),
                            })
                            .unwrap_or(serde_json::Value::Null);

                        Some(
                            UnhashedItem {
                                item_id: mc_data_item.id,
                                stack_size: mc_data_item.stack_size,
                                nbt: Arc::new(nbt),

                                count: count as u32,
                                metadata: 0,
                            }
                            .into_item(),
                        )
                    })
                    .flatten()
                    .collect::<Vec<Item>>()
            })
            .unwrap_or_else(|| vec![]);

        Some(Box::new(ShulkerData {
            name,
            color,
            contained_items,
            empty,
        }))
    }
}

mod string {
    use std::fmt::Display;
    use std::str::FromStr;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct ShulkerData {
    pub name: Option<String>,
    pub color: Option<String>,
    pub contained_items: Vec<Item>,
    pub empty: bool,
}

#[derive(Deserialize)]
struct NbtNameStr {
    text: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Item {
    pub item_id: u32,
    pub count: u32,
    pub metadata: u32,
    pub nbt: Arc<Value>,
    pub stack_size: u32,

    #[serde(with = "string")]
    pub stackable_hash: u64,

    #[serde(skip)]
    pub shulker_data: Option<Box<ShulkerData>>,
}

impl Display for Item {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_fmt(format_args!("{} x{}", self.item_id, self.count))
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum ItemMatchCriteria {
    StackableHash {
        #[serde(with = "string")]
        stackable_hash: u64,
    },
}

impl ItemMatchCriteria {
    pub fn matches_item(&self, item: &Item) -> bool {
        match self {
            Self::StackableHash { stackable_hash } => item.stackable_hash == *stackable_hash,
        }
    }
}

#[derive(Error, Debug, Serialize)]
pub enum HoldMatchError {
    #[error("The slot at the requested position was already held")]
    AlreadyHeld,
    #[error("No match was found for the requested criteria")]
    NoMatch,
}

#[derive(Deserialize, Debug, Clone)]
pub enum HoldRequestFilter {
    EmptySlot,
    ItemMatch {
        match_criteria: ItemMatchCriteria,
        total: u64,
    },
    SlotLocation {
        location: Location,
        slot: u32,
        open_from: Vec3,
    },
}

impl HoldRequestFilter {
    pub fn attempt_match(&self, state: &mut State) -> Result<Vec<Hold>, HoldMatchError> {
        match self {
            Self::EmptySlot => {
                for (loc, slot, item, open_from) in state.inventories.iter_slots() {
                    if item.is_some() || state.holds.existing_hold(loc, slot as u32).is_some() {
                        continue;
                    }

                    let hold = state
                        .holds
                        .create(loc, slot as u32, open_from)
                        .unwrap()
                        .clone();

                    return Ok(vec![hold]);
                }

                return Err(HoldMatchError::NoMatch);
            }
            Self::ItemMatch {
                match_criteria,
                total,
            } => {
                let mut total_remaining: i64 = *total as i64;
                let mut holds = vec![];

                let mut matching_items = state
                    .inventories
                    .iter_slots()
                    .filter(|(loc, slot, item, _open_from)| {
                        item.as_ref()
                            .map(|item| {
                                match_criteria.matches_item(&item)
                                    && state.holds.existing_hold(*loc, *slot as u32).is_none()
                            })
                            .unwrap_or(false)
                    })
                    .map(|(loc, slot, item, open_from)| {
                        (loc, slot, item.as_ref().unwrap().clone(), open_from)
                    })
                    .collect::<Vec<_>>();

                matching_items
                    .sort_by(|(_, _, a, _), (_, _, b, _)| a.count.cmp(&b.count).reverse());

                for (loc, slot, item, open_from) in matching_items.iter() {
                    let hold = state
                        .holds
                        .create(*loc, *slot as u32, *open_from)
                        .unwrap()
                        .clone();
                    holds.push(hold);

                    total_remaining -= item.count as i64;
                    if total_remaining <= 0 {
                        break;
                    }
                }

                if holds.len() > 0 {
                    return Ok(holds);
                } else {
                    return Err(HoldMatchError::NoMatch);
                }
            }
            Self::SlotLocation {
                location,
                slot,
                open_from,
            } => {
                if state.holds.existing_hold(*location, *slot).is_some() {
                    return Err(HoldMatchError::AlreadyHeld);
                }

                let hold = state
                    .holds
                    .create(*location, *slot, *open_from)
                    .unwrap()
                    .clone();

                return Ok(vec![hold]);
            }
        }
    }
}

#[derive(Serialize)]
pub struct Inventory {
    pub slots: Vec<Option<Item>>,
    pub scanned_at: DateTime<Utc>,
    pub open_from: Vec3,
}
