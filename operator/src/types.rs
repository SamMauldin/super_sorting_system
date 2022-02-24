use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cmp::{max, min};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::ops::Add;
use std::{fmt::Display, hash::Hasher};

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
    pub fn contained_by(&self, a: Vec2, b: Vec2) -> bool {
        let min_x = min(a.x, b.x) - 1;
        let max_x = max(a.x, b.x) + 1;
        let min_z = min(a.z, b.z) - 1;
        let max_z = max(a.z, b.z) + 1;

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
    pub nbt: Value,
    pub stack_size: u32,
}

impl UnhashedItem {
    pub fn into_item(self) -> Item {
        let stackable_hash = self.stackable_hash();

        Item {
            item_id: self.item_id,
            count: self.count,
            metadata: self.metadata,
            nbt: self.nbt,
            stack_size: self.stack_size,
            stackable_hash,
        }
    }

    fn stackable_hash(&self) -> String {
        let mut s = DefaultHasher::new();

        self.item_id.hash(&mut s);
        self.metadata.hash(&mut s);
        let serialized_nbt = serde_json::to_string(&self.nbt).unwrap();
        serialized_nbt.hash(&mut s);

        s.finish().to_string()
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Item {
    pub item_id: u32,
    pub count: u32,
    pub metadata: u32,
    pub nbt: Value,
    pub stack_size: u32,
    pub stackable_hash: String,
}

impl Display for Item {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_fmt(format_args!("{} x{}", self.item_id, self.count))
    }
}

#[derive(Serialize)]
pub struct Inventory {
    pub slots: Vec<Option<Item>>,
    pub scanned_at: DateTime<Utc>,
}
