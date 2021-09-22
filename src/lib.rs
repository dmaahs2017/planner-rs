use std::collections::HashMap;
use std::path::Path;

use anyhow::{Result, Context};
use uuid::Uuid;

pub struct Planner {
    pub items: Vec<Item>
}

type Item = HashMap<String, String>;

impl Planner {
    pub fn get_keys(&self) -> Vec<String> {
        todo!("return all of the available keys")
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        todo!("Serialize the planner to the specified path")
    }

    pub fn load(path: impl AsRef<Path>) -> Result<()> {
        todo!("Deserialize the planner from the specified path")
    }

    pub fn insert(&mut self, item: Item) -> Result<Uuid> {
        todo!("insert the item into the planner. Add a UUID as well")
    }

    pub fn delete(&mut self, tags: Item) -> Result<Vec<Uuid>> {
        todo!("delete items with matching tags. Return list of Uuids which were deleted ")
    }

    pub fn select(&self, tags: Item) -> Result<Vec<&Item>> {
        todo!("retrieve the elements with the matching tags")
    }

    pub fn select_mut(&mut self, tags: Item) -> Result<Vec<&mut Item>> {
        todo!("mutably retrieve the elements with the matching tags")
    }
}
