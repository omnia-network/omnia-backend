use ic_cdk::api::trap;
use ic_oxigraph::model::NamedNode;
use ic_oxigraph::sparql::QueryResults;
use omnia_types::errors::GenericError;
use sparesults::{QueryResultsFormat, QueryResultsSerializer};

use crate::RDF_DB;

// RDF available prefixes and nodes

/// http://rdf.omnia-iot.com#
pub const OMNIA_PREFIX: &str = "http://rdf.omnia-iot.com#";
pub struct OmniaNode;
impl OmniaNode {
    pub fn from(name: &str) -> NamedNode {
        match NamedNode::new(format!("{}{}", OMNIA_PREFIX, name)) {
            Ok(node) => node,
            Err(_) => trap("Error creating OmniaNode"),
        }
    }
}

/// https://saref.etsi.org/core/
pub const SAREF_PREFIX: &str = "https://saref.etsi.org/core/";
pub struct SarefNode;
impl SarefNode {
    pub fn from(name: &str) -> NamedNode {
        match NamedNode::new(format!("{}{}", SAREF_PREFIX, name)) {
            Ok(node) => node,
            Err(_) => trap("Error creating SarefNode"),
        }
    }

    /// Expects a string like saref:OnCommand or OnCommand as input and returns the SAREF named node
    pub fn from_prefixed(name: &str) -> NamedNode {
        if let Some(stripped) = name.strip_prefix("saref:") {
            return SarefNode::from(stripped);
        }
        SarefNode::from(name)
    }
}

/// https://w3id.org/bot#
pub const BOT_PREFIX: &str = "https://w3id.org/bot#";
pub struct BotNode;
impl BotNode {
    pub fn from(name: &str) -> NamedNode {
        match NamedNode::new(format!("{}{}", BOT_PREFIX, name)) {
            Ok(node) => node,
            Err(_) => trap("Error creating BotNode"),
        }
    }
}

/// https://www.w3.org/2011/http#
pub const HTTP_PREFIX: &str = "https://www.w3.org/2011/http#";
pub struct HttpNode;
impl HttpNode {
    pub fn from(name: &str) -> NamedNode {
        match NamedNode::new(format!("{}{}", HTTP_PREFIX, name)) {
            Ok(node) => node,
            Err(_) => trap("Error creating HttpNode"),
        }
    }
}

/// https://www.w3.org/2019/wot/td#
pub const TD_PREFIX: &str = "https://www.w3.org/2019/wot/td#";
pub struct TdNode;
impl TdNode {
    pub fn from(name: &str) -> NamedNode {
        match NamedNode::new(format!("{}{}", TD_PREFIX, name)) {
            Ok(node) => node,
            Err(_) => trap("Error creating TdNode"),
        }
    }
}

/// urn:
pub const URN_PREFIX: &str = "urn:";
pub struct UrnNode;
impl UrnNode {
    /// Creates a urn:uuid: node
    pub fn new_uuid(name: &str) -> NamedNode {
        match NamedNode::new(format!("{}uuid:{}", URN_PREFIX, name)) {
            Ok(node) => node,
            Err(_) => trap("Error creating UrnNode (uuid)"),
        }
    }
}

pub fn execute_sparql_query(query: String) -> Result<Vec<u8>, GenericError> {
    RDF_DB.with(|store| {
        let rdf_db = store.borrow();

        if let QueryResults::Solutions(solutions) = rdf_db
            .query(&query)
            .map_err(|e| format!("Error executing SPARQL query: {:?} (query: {})", e, query))?
        {
            let json_serializer = QueryResultsSerializer::from_format(QueryResultsFormat::Json);

            let mut solutions_writer = json_serializer
                .solutions_writer(Vec::new(), solutions.variables().to_vec())
                .map_err(|e| format!("Error serializing SPARQL query variables: {:?}", e))?;

            for solution in solutions {
                solutions_writer
                    .write(&solution.map_err(|e| format!("Error getting solution: {:?}", e))?)
                    .map_err(|e| format!("Error serializing SPARQL query results: {:?}", e))?;
            }

            return solutions_writer
                .finish()
                .map_err(|e| format!("Error serializing SPARQL query results: {:?}", e));
        }

        Err("Error executing SPARQL query".to_string())
    })
}
