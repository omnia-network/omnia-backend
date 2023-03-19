use std::collections::BTreeMap;

use candid::{CandidType, Deserialize};
use serde::Serialize;

pub mod virtual_persona;
pub mod environment;
pub mod gateway;
pub mod errors;
pub mod http;

#[derive(Default, CandidType, Serialize, Deserialize)]
pub struct CrudMap<I: Ord, V> {
    map: BTreeMap<I, V>,
}

impl<I: Ord, V> CrudMap<I, V> {
    pub fn default() -> Self {
        Self {
            map: BTreeMap::<I, V>::default()
        }
    }

    pub fn create(&mut self, index: I, value: V) {
        self.map.insert(index, value);
    }

    pub fn read(&self, index: &I) -> Option<&V>{
        self.map.get(index)
    }

    pub fn update(&mut self, index: I, value: V) {
        self.map.insert(index, value);
    }

    pub fn delete(&mut self, index: &I) -> Option<V> {
        self.map.remove(index)
    }
}