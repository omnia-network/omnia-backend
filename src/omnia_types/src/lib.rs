use candid::{CandidType, Deserialize};
use device::DeviceUid;
use environment::{
    EnvironmentIndex, EnvironmentUID, EnvironmentUidIndex, EnvironmentUidValue, EnvironmentValue,
};
use errors::GenericResult;
use gateway::{
    GatewayPrincipalId, InitializedGatewayIndex, InitializedGatewayValue, RegisteredGatewayIndex,
    RegisteredGatewayValue,
};
use http::{IpChallengeIndex, IpChallengeNonce, IpChallengeValue};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fmt::Debug;
use virtual_persona::{VirtualPersonaIndex, VirtualPersonaPrincipalId, VirtualPersonaValue};

pub mod device;
pub mod environment;
pub mod errors;
pub mod gateway;
pub mod http;
pub mod updates;
pub mod virtual_persona;

#[derive(Default, CandidType, Serialize, Deserialize)]
pub struct CrudMap<I: Ord + Debug, V> {
    map: BTreeMap<I, V>,
}

impl<I: Ord + Debug, V> CrudMap<I, V> {
    pub fn create(&mut self, index: I, value: V) -> GenericResult<()> {
        match self.map.contains_key(&index) {
            false => {
                self.map.insert(index, value);
                Ok(())
            }
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

    pub fn read(&self, index: &I) -> GenericResult<&V> {
        match self.map.get(index) {
            Some(value) => Ok(value),
            None => {
                let err = format!("Entry with index {:?} does not exist", index);

                println!("{}", err);
                Err(err)
            }
        }
    }

    pub fn update(&mut self, index: I, value: V) -> GenericResult<V> {
        match self.map.contains_key(&index) {
            true => Ok(self
                .map
                .insert(index, value)
                .expect("should contain previous value")),
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
                let err = format!("Entry with index {:?} does not exist", index);

                println!("{}", err);
                Err(err)
            }
        }
    }
}

impl CrudMap<IpChallengeIndex, IpChallengeValue> {
    pub fn validate_ip_challenge_by_nonce(
        &mut self,
        nonce: IpChallengeNonce,
    ) -> GenericResult<IpChallengeValue> {
        let ip_challenge_index = IpChallengeIndex { nonce };
        let ip_challenge_value = self.delete(&ip_challenge_index)?;
        Ok(ip_challenge_value)
    }
}

impl CrudMap<InitializedGatewayIndex, InitializedGatewayValue> {
    pub fn is_gateway_initialized(
        &self,
        initialized_gateway_index: InitializedGatewayIndex,
    ) -> bool {
        // check existance in initialized gateways
        self.read(&initialized_gateway_index).is_ok()
    }
}

impl CrudMap<EnvironmentUidIndex, EnvironmentUidValue> {
    pub fn get_environment_uid_by_ip(
        &self,
        environment_uid_index: EnvironmentUidIndex,
    ) -> GenericResult<EnvironmentUID> {
        let environment_uid_value = self.read(&environment_uid_index)?.clone();
        Ok(environment_uid_value.env_uid)
    }
}

impl CrudMap<EnvironmentIndex, EnvironmentValue> {
    pub fn insert_gateway_principal_id_in_env(
        &mut self,
        environment_index: EnvironmentIndex,
        gateway_principal_id: GatewayPrincipalId,
    ) -> GenericResult<EnvironmentValue> {
        let mut updatable_environment_value = self.read(&environment_index)?.clone();
        updatable_environment_value
            .env_gateways_principals_ids
            .insert(gateway_principal_id, ());
        self.update(environment_index, updatable_environment_value)
    }

    pub fn insert_user_principal_id_in_env(
        &mut self,
        environment_index: EnvironmentIndex,
        virtual_persona_principal_id: VirtualPersonaPrincipalId,
    ) -> GenericResult<EnvironmentValue> {
        let environment_value = self.read(&environment_index)?;
        let mut updatable_environment_value = environment_value.clone();
        updatable_environment_value
            .env_users_principals_ids
            .insert(virtual_persona_principal_id, ());
        self.update(environment_index, updatable_environment_value)
    }

    pub fn remove_user_principal_id_in_env(
        &mut self,
        environment_index: EnvironmentIndex,
        virtual_persona_principal_id: VirtualPersonaPrincipalId,
    ) -> GenericResult<EnvironmentValue> {
        let environment_value = self.read(&environment_index)?;
        let mut updatable_environment_value = environment_value.clone();
        updatable_environment_value
            .env_users_principals_ids
            .remove(&virtual_persona_principal_id);
        self.update(environment_index, updatable_environment_value)
    }
}

impl CrudMap<VirtualPersonaIndex, VirtualPersonaValue> {
    pub fn insert_env_in_virtual_persona_as_user(
        &mut self,
        virtual_persona_index: VirtualPersonaIndex,
        environment_uid: EnvironmentUID,
    ) -> GenericResult<VirtualPersonaValue> {
        let virtual_persona_value = self.read(&virtual_persona_index)?.clone();
        let updated_virtual_persona = VirtualPersonaValue {
            user_env_uid: Some(environment_uid),
            ..virtual_persona_value
        };
        self.update(virtual_persona_index, updated_virtual_persona)
    }

    pub fn remove_env_in_virtual_persona_as_user(
        &mut self,
        virtual_persona_index: VirtualPersonaIndex,
    ) -> GenericResult<VirtualPersonaValue> {
        let virtual_persona_value = self.read(&virtual_persona_index)?.clone();
        let updated_virtual_persona = VirtualPersonaValue {
            user_env_uid: None,
            ..virtual_persona_value
        };
        self.update(virtual_persona_index, updated_virtual_persona)
    }

    pub fn insert_env_in_virtual_persona_as_manager(
        &mut self,
        virtual_persona_index: VirtualPersonaIndex,
        environment_uid: EnvironmentUID,
    ) -> GenericResult<VirtualPersonaValue> {
        let virtual_persona_value = self.read(&virtual_persona_index)?.clone();
        let updated_virtual_persona = VirtualPersonaValue {
            manager_env_uid: Some(environment_uid),
            ..virtual_persona_value
        };
        self.update(virtual_persona_index, updated_virtual_persona)
    }
}

impl CrudMap<RegisteredGatewayIndex, RegisteredGatewayValue> {
    pub fn insert_device_uid_in_gateway(
        &mut self,
        registered_gateway_index: RegisteredGatewayIndex,
        device_uid: DeviceUid,
    ) -> GenericResult<RegisteredGatewayValue> {
        let mut updatable_registered_gateway_value = self.read(&registered_gateway_index)?.clone();
        updatable_registered_gateway_value
            .gat_registered_device_uids
            .insert(device_uid, ());
        self.update(registered_gateway_index, updatable_registered_gateway_value)
    }
}
