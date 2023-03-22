use std::collections::BTreeMap;

use candid::{CandidType, Deserialize};
use environment::{EnvironmentValue, EnvironmentIndex};
use errors::GenericResult;
use gateway::GatewayPrincipalId;
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

    pub fn create(&mut self, index: I, value: V) -> GenericResult<()>{
        match self.map.contains_key(&index) {
            false => {
                self.map.insert(index, value);
                Ok(())
            },
            true => {
                let err = format!(
                    "Entry with index {:?} already exists, use UPDATE method instead",
                    index
                );
                
                println!("{}", err);
                Err(err)
            }
        }
    }

    pub fn read(&self, index: &I) -> GenericResult<&V>{
        match self.map.get(index) {
            Some(value) => Ok(value),
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

    pub fn update(&mut self, index: I, value: V) -> GenericResult<V> {
        match self.map.contains_key(&index) {
            true => Ok(self.map.insert(index, value).expect("should contain previous value")),
            false => {
                let err = format!(
                    "Entry with index {:?} does not exist, use CREATE method instead",
                    index
                );
                
                println!("{}", err);
                Err(err)
            }
        }
    }

    pub fn delete(&mut self, index: &I) -> GenericResult<V> {
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

impl CrudMap<EnvironmentIndex, EnvironmentValue> {
    pub fn update_env_gateway_principal_ids(&mut self, environment_index: EnvironmentIndex, gateway_principal_id: GatewayPrincipalId) -> GenericResult<EnvironmentValue> {
        let environment_value = self.read(&environment_index)?;
        let mut updatable_environment_value = environment_value.clone();
        updatable_environment_value.env_gateway_principal_ids.insert(gateway_principal_id, ());
        self.update(environment_index, updatable_environment_value)
    }
}