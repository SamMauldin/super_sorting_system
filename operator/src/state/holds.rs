use crate::types::{Location, Vec3};
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub struct Hold {
    pub id: Uuid,
    pub location: Location,
    pub slot: u32,
    pub open_from: Vec3,
    pub valid_until: DateTime<Utc>,
}

pub struct HoldState {
    holds: HashMap<Uuid, Hold>,
}

impl Default for HoldState {
    fn default() -> HoldState {
        HoldState {
            holds: Default::default(),
        }
    }
}

#[derive(Error, Debug, Serialize)]
pub enum HoldError {
    #[error("A hold is already present for that slot")]
    AlreadyHeld,
}

impl HoldState {
    pub fn iter(&self) -> impl Iterator<Item = &Hold> {
        self.holds.iter().map(|(_id, hold)| hold)
    }

    pub fn get(&self, id: Uuid) -> Option<&Hold> {
        self.holds.get(&id)
    }

    pub fn remove(&mut self, id: Uuid) -> Option<Hold> {
        self.holds.remove(&id)
    }

    pub fn takeover(&mut self, id: Uuid) -> Option<&Hold> {
        let mut previous_hold = self.holds.remove(&id)?;

        let new_id = Uuid::new_v4();
        previous_hold.id = new_id;
        self.holds.insert(new_id, previous_hold);
        self.renew(new_id);

        Some(self.holds.get(&new_id).unwrap())
    }

    pub fn existing_hold(&self, location: Location, slot: u32) -> Option<&Hold> {
        self.holds
            .iter()
            .find(|(_id, hold)| hold.location == location && hold.slot == slot)
            .map(|(_id, hold)| hold)
    }

    pub fn create(
        &mut self,
        location: Location,
        slot: u32,
        open_from: Vec3,
    ) -> Result<&Hold, HoldError> {
        let id = Uuid::new_v4();

        let existing_hold = self.existing_hold(location, slot);
        if let Some(_) = existing_hold {
            return Err(HoldError::AlreadyHeld);
        }

        self.holds.insert(
            id,
            Hold {
                id,
                location,
                slot,
                valid_until: Utc::now() + Duration::minutes(5),
                open_from,
            },
        );

        Ok(self.holds.get(&id).unwrap())
    }

    pub fn renew(&mut self, id: Uuid) -> Option<&Hold> {
        self.holds.get_mut(&id).map(|hold| {
            hold.valid_until = Utc::now() + Duration::minutes(5);

            &*hold
        })
    }
}
