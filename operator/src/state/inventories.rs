use serde::Deserialize;
use std::collections::{hash_map::Iter, HashMap};

use crate::{
    data::McData,
    types::{Inventory, Item, Location, UnhashedItem},
};

lazy_static::lazy_static! {
    static ref MC_DATA: McData = McData::init();
}

pub struct InventoryState {
    inventory_map: HashMap<Location, Inventory>,
}

impl Default for InventoryState {
    fn default() -> Self {
        InventoryState {
            inventory_map: Default::default(),
        }
    }
}

impl InventoryState {
    pub fn set_inventory_at(&mut self, location: Location, inventory: Inventory) {
        self.inventory_map.insert(location, inventory);
    }

    pub fn inventory_contents_at(&self, location: &Location) -> Option<&Inventory> {
        self.inventory_map.get(location)
    }

    pub fn iter_inventories(&self) -> Iter<Location, Inventory> {
        self.inventory_map.iter()
    }

    pub fn iter_slots(&self) -> impl Iterator<Item = (Location, usize, &Option<Item>)> {
        self.iter_inventories().flat_map(|(&loc, inv)| {
            inv.slots
                .iter()
                .enumerate()
                .map(move |(slot, item)| (loc, slot, item))
        })
    }

    pub fn get_listing(&self) -> Vec<Item> {
        let mut item_map = HashMap::<String, Item>::new();

        let mut insert_item = |item: &Item| {
            let mapped_item = item_map.get_mut(&item.stackable_hash);

            if let Some(mapped_item) = mapped_item {
                (*mapped_item).count += item.count;
            } else {
                item_map.insert(item.stackable_hash.clone(), item.clone());
            }
        };

        for (_loc, _slot, item) in self.iter_slots() {
            if let Some(item) = item {
                if let Some(shulker_data) = item.shulker_data() {
                    let should_unpack = shulker_data.name.is_none() && shulker_data.contained_items.len() > 0;

                    if should_unpack {
                        for contained_item in shulker_data.contained_items {
                            insert_item(&contained_item);
                        }
                    } else {
                        insert_item(item);
                    }
                } else {
                    insert_item(item);
                }
            }
        }

        item_map.into_values().collect()
    }
}

struct ShulkerData {
    name: Option<String>,
    color: Option<String>,
    contained_items: Vec<Item>,
}

#[derive(Deserialize)]
struct NbtNameStr {
    text: String,
}

impl Item {
    fn shulker_data(&self) -> Option<ShulkerData> {
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

        let contained_items = contained_items_nbt.map(|items_list| {
            items_list
                .iter()
                .map(|nbt_item| {
                    let item_mc_name = nbt_item.pointer("/id/value").unwrap().as_str().unwrap().strip_prefix("minecraft:").unwrap();
                    let mc_data_item = MC_DATA
                        .items_by_name
                        .get(item_mc_name)?;
                    let count = nbt_item.pointer("/Count/value").unwrap().as_u64().unwrap();
                    let nbt = nbt_item
                        .pointer("/tag")
                        .map(|tag| tag.clone())
                        .unwrap_or(serde_json::Value::Null);

                    Some(UnhashedItem {
                        item_id: mc_data_item.id,
                        stack_size: mc_data_item.stack_size,
                        nbt,

                        count: count as u32,
                        metadata: 0,
                    }
                    .into_item())
                })
                .flatten()
                .collect::<Vec<Item>>()
        })?;

        Some(ShulkerData {
            name,
            color,
            contained_items,
        })
    }
}
