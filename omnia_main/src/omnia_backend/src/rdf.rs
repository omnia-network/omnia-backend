use candid::Nat;
use ic_cdk::api::{
    management_canister::http_request::{
        http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
    },
    print,
};
use omnia_types::errors::GenericError;
use omnia_utils::uuid::generate_uuid;

use crate::utils::get_rdf_database_connection;

pub type Triple = (String, String, String);

const OMNIA_GRAPH: &str = "omnia:";

/// RDF database graph prefixes:
/// - **omnia**: <http://rdf.omnia-iot.com#>
/// - **rdf**: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
/// - **saref**: <https://saref.etsi.org/core/>
/// - **bot**: <https://w3id.org/bot#>
/// - **http**: <https://www.w3.org/2011/http#>
/// - **urn**: `<urn:>`
const PREFIXES: &str = r#"
# Omnia
PREFIX omnia: <http://rdf.omnia-iot.com#>
# Third parties
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX saref: <https://saref.etsi.org/core/>
PREFIX bot: <https://w3id.org/bot#>
PREFIX http: <https://www.w3.org/2011/http#>
PREFIX td: <https://www.w3.org/2019/wot/td#>
# Definitions
PREFIX urn: <urn:>
"#;

const MAX_RESPONSE_BYTES: u64 = 1024; // 1KB

fn build_query(q: &str) -> String {
    let mut query = String::from(PREFIXES);
    query.push_str(q);
    query
}

async fn send_query(q: String) -> Result<(), GenericError> {
    let rdf_base_url = get_rdf_database_connection().base_url;

    let request_headers = vec![
        HttpHeader {
            name: "Host".to_string(),
            // get only the host:port part of the URL
            value: rdf_base_url.split("://").collect::<Vec<&str>>()[1].to_string(),
        },
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "omnia_backend_canister".to_string(),
        },
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/sparql-update".to_string(),
        },
        // the Idempotent-Key is required to avoid flooding the RDF store with the same query from all the replicas
        HttpHeader {
            name: "Idempotent-Key".to_string(),
            value: generate_uuid().await,
        },
        HttpHeader {
            name: "Authorization".to_string(),
            value: format!("apikey {}", get_rdf_database_connection().api_key),
        },
    ];

    let url = format!("{}/update", rdf_base_url);

    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::POST,
        body: Some(q.as_bytes().to_vec()),
        max_response_bytes: Some(MAX_RESPONSE_BYTES),
        transform: None,
        headers: request_headers,
    };
    match http_request(request).await {
        Ok((response,)) => {
            // needed just to avoid clippy warnings
            #[allow(clippy::cmp_owned)]
            if response.status >= Nat::from(200) && response.status < Nat::from(400) {
                let message =
                    format!("The http_request resulted into success. Response: {response:?}");
                print(message);
                Ok(())
            } else {
                let message =
                    format!("The http_request resulted into error. Response: {response:?}");
                print(message.clone());
                Err(message)
            }
        }
        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");
            print(message.clone());

            Err(message)
        }
    }
}

/// Insert data in the RDF database.<br>
/// Available prefixes: [PREFIXES]<br>
/// TODO: implement nested insert and where conditions
pub async fn insert(triples: Vec<Triple>) -> Result<(), GenericError> {
    let mut query = format!("INSERT DATA {{ GRAPH {OMNIA_GRAPH} {{\n");
    for (s, p, o) in triples {
        query.push_str(format!("{s} {p} {o} .\n").as_str());
    }
    query.push_str("} }");

    query = build_query(query.as_str());

    send_query(query).await
}

/// Delete data from the RDF database.<br>
/// Available prefixes: [PREFIXES]
pub async fn delete(triples: Vec<Triple>) -> Result<(), GenericError> {
    let mut query = format!("DELETE DATA {{ GRAPH {OMNIA_GRAPH} {{\n");
    for (s, p, o) in triples {
        query.push_str(format!("{s} {p} {o} .\n").as_str());
    }
    query.push_str("} }");

    query = build_query(query.as_str());

    send_query(query).await
}
