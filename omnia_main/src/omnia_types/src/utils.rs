use candid::{CandidType, Deserialize};

#[derive(Default, CandidType, Deserialize, Clone, Debug)]
pub struct RdfDatabaseConnection {
    pub base_url: String,
    /// TODO: this si insecure, because it can be read by malicious replicas
    /// TODO: a secure way would be to sign HTTP requests using the shared chain key and verify the signature on the RDF database side
    pub api_key: String,
}