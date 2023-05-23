# RDF database
Omnia Backend embeds an [RDF](https://www.w3.org/TR/rdf11-concepts/) database where devices' **metadata** (the Environment they belong to, their affordances, etc.) are stored. It's implemented using the [omnia-network/ic-oxigraph](https://github.com/omnia-network/ic-oxigraph) library.

A [SPARQL](https://www.w3.org/TR/sparql11-overview/) endpoint is available through both the Backend canister's HTTPS endpoint and the candid methods `executeRdfDbQuery` and `executeRdfDbQueryAsUpdate`.