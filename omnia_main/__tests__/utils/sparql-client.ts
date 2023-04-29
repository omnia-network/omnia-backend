import SimpleClient from 'sparql-http-client/SimpleClient';
import dotenv from 'dotenv';

dotenv.config();

const {
  RDF_DATABASE_BASE_URL = '',
} = process.env;

export const sparqlClient = new SimpleClient({
  endpointUrl: `${RDF_DATABASE_BASE_URL}/query`,
});

// taken from rust code
// TODO: have a single source of truth for prefixes
export const PREFIXES = `
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
`;
