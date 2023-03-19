use std::collections::BTreeMap;

use candid::{CandidType, Deserialize};
use errors::GenericError;
use http::{IpChallengeIndex, IpChallengeValue, IpChallengeValueResult};
use serde::Serialize;
use std::fmt::Debug;

pub mod virtual_persona;
pub mod environment;
pub mod gateway;
pub mod errors;
pub mod http;

#[derive(Default, CandidType, Serialize, Deserialize)]
pub struct CrudMap<I: Ord + Debug, V> {
    map: BTreeMap<I, V>,
}

impl<I: Ord + Debug, V> CrudMap<I, V> {
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

    pub fn delete(&mut self, index: &I) -> Result<V, GenericError> {
        match self.map.remove(index) {
            Some(deleted_value) => Ok(deleted_value),
            None => {
                let err = format!(
                    "Entry with index {:?} does not exist",
                    index
                );
                
                println!("{}", err);
                Err(err)
            }
        }
    }
}

impl CrudMap<IpChallengeIndex, IpChallengeValue> {
    pub fn validate_ip_challenge(&mut self, nonce: &IpChallengeIndex) -> IpChallengeValueResult {
        Ok(self.delete(nonce)?)
    }
}