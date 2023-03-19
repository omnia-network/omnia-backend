use std::{cell::RefCell, ops::Deref};
use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::stable::{StableReader, StableWriter};
use ic_cdk_macros::{post_upgrade, pre_upgrade};
use omnia_types::CrudMap;
use omnia_types::environment::{EnvironmentUID, Environment, Ip};
use omnia_types::errors::GenericError;
use omnia_types::gateway::{GatewayPrincipald, GatewayIp, GatewayPrincipalId, RegisteredGateway};
use omnia_types::http::{IpChallengeNonce, IpChallengeValue, IpChallengeIndex};
use omnia_types::virtual_persona::{VirtualPersonaIp, VirtualPersonaPrincipalId, VirtualPersonaEntry, VirtualPersonaIndex, VirtualPersonaValue};
use serde::Serialize;

mod environment;
mod virtual_persona;
mod uuid;
mod auth;

#[derive(Default, CandidType, Serialize, Deserialize)]
struct State {
    pub virtual_personas: CrudMap<VirtualPersonaIndex, VirtualPersonaValue>,
    // pub environments: CrudMap<EnvironmentIndex, EnvironmentValue>,
    // pub environment_uids: CrudMap<EnvironemntUidsIndex, EnvironmentUidsValue>,
    // pub registered_gateways: CrudMap<RegisteredgatewayIndex, RegisteredGatewayValue>,
    pub ip_challenges: CrudMap<IpChallengeIndex, IpChallengeValue>,
    // pub initialized_gateways: CrudMap<GatewayIp, GatewayPrincipald>,
}

impl State {
    fn default() -> Self {
        Self {
            virtual_personas: CrudMap::default(),
            // environments: CrudMap::default(),
            // environment_uids: CrudMap::default(),
            // registered_gateways: CrudMap::default(),
            ip_challenges: CrudMap::default(),
            // initialized_gateways: CrudMap::default(),
        }
    }

    pub fn validate_ip_challenge(&mut self, nonce: &IpChallengeIndex) -> Option<IpChallengeValue> {
        self.ip_challenges.delete(nonce)
    }
}


//     pub fn consume_ip_challenge(&mut self, nonce: &CanisterCallNonce) -> Option<RequesterInfo> {
//         self.initialized_nonce_to_ip.remove(nonce)
//     }

//     pub fn create_virtual_persona(&mut self, virtual_persona_principal_id: VirtualPersonaIndex, new_virtual_persona: VirtualPersonaValue) {
//         self.virtual_personas.insert(virtual_persona_principal_id, new_virtual_persona);
//     }

//     pub fn get_virtual_persona_by_principal(&mut self, virtual_persona_principal_id: &VirtualPersonaIndex) -> Option<VirtualPersonaValue> {
//         match self.virtual_personas.get(virtual_persona_principal_id) {
//             Some(virtual_persona) => Some(virtual_persona.to_owned()),
//             None => None,
//         }
//     }

//     pub fn create_environment(&mut self, environment_uid: EnvironmentUID, environment: Environment) {
//         self.environments.insert(environment_uid, environment);
//     }

//     pub fn get_environment_by_uid(&mut self, environment_uid: &EnvironmentUID) -> Result<&mut Environment, GenericError> {
//         match self
//             .environments
//             .get_mut(environment_uid)
//         {
//             Some(environment) => Ok(environment),
//             None => {
//                 let err = format!(
//                     "Environment with uid {:?} does not exist",
//                     environment_uid
//                 );
//                 Err(err)
//             },
//         }
//     }

//     pub fn get_environment_uid_from_ip(&mut self, requester_ip: &VirtualPersonaIp) -> Option<EnvironmentUID>{
//         match self.ip_to_env_uid.get(requester_ip) {
//             Some(env_uid) => Some(env_uid.clone()),
//             None => None
//         }
//     }

//     pub fn initialize_gateway_by_ip(&mut self, requester_ip: GatewayIp, gateway_principal_id: GatewayPrincipalId) {
//         self.initialized_gateways.insert(requester_ip, gateway_principal_id);
//     }

//     pub fn get_initialized_gateways_by_ip(&mut self, requester_ip: &VirtualPersonaIp) -> Result<Vec<GatewayPrincipald>, ()> {
//         match self.initialized_gateways.get(requester_ip) {
//             Some(gateway_uid) => Ok(vec![gateway_uid.to_owned()]),
//             None => Ok(vec![]),
//         }
//     }

//     pub fn consume_initialized_gateway(&mut self, requester_ip: &VirtualPersonaIp) -> Option<GatewayPrincipalId>{
//         self.initialized_gateways.remove(requester_ip)
//     }

//     pub fn create_ip_to_uid_environment_mapping(&mut self, requester_ip: VirtualPersonaIp, environment_uid: EnvironmentUID) {
//         self.ip_to_env_uid.insert(requester_ip, environment_uid);
//     }

//     pub fn create_registered_gateway(&mut self, gateway_principal_id: GatewayPrincipalId, registered_gateway: RegisteredGateway) {
//         self.registered_gateways.insert(gateway_principal_id, registered_gateway);
//     }

//     pub fn get_registered_gateway_by_principal_id(&mut self, gateway_principal_id: &GatewayPrincipalId) -> Option<RegisteredGateway> {
//         match self.registered_gateways.get(gateway_principal_id) {
//             Some(gateway) => Some(gateway.clone()),
//             None => None,
//         }
//     }
// }

thread_local! {
    static STATE: RefCell<State>  = RefCell::new(State::default());
}

#[pre_upgrade]
fn pre_upgrade() {
    STATE.with(|state| {
        ciborium::ser::into_writer(state.borrow().deref(), StableWriter::default())
            .expect("failed to encode state")
    })
}

#[post_upgrade]
fn post_upgrade() {
    STATE.with(|cell| {
        *cell.borrow_mut() =
            ciborium::de::from_reader(StableReader::default()).expect("failed to decode state");
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::export_service;

    #[test]
    fn save_candid() {
        use omnia_types::environment::*;
        use omnia_types::gateway::*;
        use omnia_types::virtual_persona::*;
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::current_dir().unwrap());
        export_service!();
        write(dir.join("database.did"), __export_service()).expect("Write failed.");
    }
}
