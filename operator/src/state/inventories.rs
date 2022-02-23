use std::collections::{hash_map::Iter, HashMap};

use crate::types::{Inventory, Item, Vec3};

pub struct InventoryState {
    inventory_map: HashMap<Vec3, Inventory>,
}

impl Default for InventoryState {
    fn default() -> Self {
        InventoryState {
            inventory_map: Default::default(),
        }
    }
}

impl InventoryState {
    pub fn set_inventory_at(&mut self, location: Vec3, inventory: Inventory) {
        self.inventory_map.insert(location, inventory);
    }

    pub fn inventory_contents_at(&self, location: &Vec3) -> Option<&Inventory> {
        self.inventory_map.get(location)
    }

    pub fn iter_inventories(&self) -> Iter<Vec3, Inventory> {
        self.inventory_map.iter()
    }

    pub fn iter_slots(&self) -> impl Iterator<Item = (Vec3, usize, &Option<Item>)> {
        self.iter_inventories().flat_map(|(&loc, inv)| {
            inv.slots
                .iter()
                .enumerate()
                .map(move |(slot, item)| (loc, slot, item))
        })
    }
}
