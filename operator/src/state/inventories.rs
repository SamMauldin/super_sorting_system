use std::collections::{hash_map::Iter, HashMap};

use crate::types::{Inventory, Item, Location};

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

        for (_loc, _slot, item) in self.iter_slots() {
            if let Some(item) = item {
                let mapped_item = item_map.get_mut(&item.stackable_hash);

                if let Some(mapped_item) = mapped_item {
                    (*mapped_item).count += item.count;
                } else {
                    item_map.insert(item.stackable_hash.clone(), item.clone());
                }
            }
        }

        item_map.into_values().collect()
    }
}
