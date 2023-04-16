use std::collections::BTreeMap;

use candid::{CandidType, Deserialize};
use environment::{EnvironmentValue, EnvironmentIndex, EnvironmentUID};
use errors::GenericResult;
use gateway::{GatewayPrincipalId, InitializedGatewayIndex, InitializedGatewayValue};
use http::{IpChallengeIndex, IpChallengeValue, IpChallengeValueResult};
use serde::Serialize;
use virtual_persona::{VirtualPersonaPrincipalId, VirtualPersonaIndex, VirtualPersonaValue};
use std::fmt::Debug;

pub mod virtual_persona;
pub mod environment;
pub mod gateway;
pub mod errors;
pub mod http;
pub mod updates;
pub mod device;

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

impl CrudMap<InitializedGatewayIndex, InitializedGatewayValue> {
    pub fn is_gateway_initialized(&self, initialized_gateway_index: InitializedGatewayIndex) -> bool {
        // check existance in initialized gateways
        match self.read(&initialized_gateway_index) {
            Ok(_) => true,
            Err(_) => false
        }
    }
}

impl CrudMap<EnvironmentIndex, EnvironmentValue> {
    pub fn insert_gateway_principal_id_in_env(&mut self, environment_index: EnvironmentIndex, gateway_principal_id: GatewayPrincipalId) -> GenericResult<EnvironmentValue> {
        let environment_value = self.read(&environment_index)?;
        let mut updatable_environment_value = environment_value.clone();
        updatable_environment_value.env_gateways_principals_ids.insert(gateway_principal_id, ());
        self.update(environment_index, updatable_environment_value)
    }

    pub fn insert_user_principal_id_in_env(&mut self, environment_index: EnvironmentIndex, virtual_persona_principal_id: VirtualPersonaPrincipalId) -> GenericResult<EnvironmentValue> {
        let environment_value = self.read(&environment_index)?;
        let mut updatable_environment_value = environment_value.clone();
        updatable_environment_value.env_users_principals_ids.insert(virtual_persona_principal_id, ());
        self.update(environment_index, updatable_environment_value)
    }

    pub fn remove_user_principal_id_in_env(&mut self, environment_index: EnvironmentIndex, virtual_persona_principal_id: VirtualPersonaPrincipalId) -> GenericResult<EnvironmentValue> {
        let environment_value = self.read(&environment_index)?;
        let mut updatable_environment_value = environment_value.clone();
        updatable_environment_value.env_users_principals_ids.remove(&virtual_persona_principal_id);
        self.update(environment_index, updatable_environment_value)
    }
}

impl CrudMap<VirtualPersonaIndex, VirtualPersonaValue> {
    pub fn insert_env_in_virtual_persona_as_user(&mut self, virtual_persona_index: VirtualPersonaIndex, environment_uid: EnvironmentUID) -> GenericResult<VirtualPersonaValue> {
        let virtual_persona_value = match self.read(&virtual_persona_index) {
            Ok(virtual_persona_value) => Ok(virtual_persona_value.clone()),
            Err(e) => Err(e),
        }?;
        let updated_virtual_persona = VirtualPersonaValue {
            user_env_uid: Some(environment_uid),
            ..virtual_persona_value.to_owned()
        };
        self.update(virtual_persona_index, updated_virtual_persona)
    }

    pub fn remove_env_in_virtual_persona_as_user(&mut self, virtual_persona_index: VirtualPersonaIndex) -> GenericResult<VirtualPersonaValue> {
        let virtual_persona_value = match self.read(&virtual_persona_index) {
            Ok(virtual_persona_value) => Ok(virtual_persona_value.clone()),
            Err(e) => Err(e),
        }?;
        let updated_virtual_persona = VirtualPersonaValue {
            user_env_uid: None,
            ..virtual_persona_value.to_owned()
        };
        self.update(virtual_persona_index, updated_virtual_persona)
    }

    pub fn insert_env_in_virtual_persona_as_manager(&mut self, virtual_persona_index: VirtualPersonaIndex, environment_uid: EnvironmentUID) -> GenericResult<VirtualPersonaValue> {
        let virtual_persona_value = match self.read(&virtual_persona_index) {
            Ok(virtual_persona_value) => Ok(virtual_persona_value.clone()),
            Err(e) => Err(e),
        }?;
        let updated_virtual_persona = VirtualPersonaValue {
            manager_env_uid: Some(environment_uid),
            ..virtual_persona_value.to_owned()
        };
        self.update(virtual_persona_index, updated_virtual_persona)
    }
}