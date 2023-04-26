#!/bin/bash

source .env

# we need to pass the database canister principal id to omnia_backend canister
# dfx deploy --argument '(null, "<database-canister-principal-id>")'
# the first null argument is needed by internet_identity canister
# TODO: read database canister id from .dfx/canister_ids.json automatically (maybe use a node script for that)

if [ "$DATABASE_CANISTER_PRINCIPAL_ID" = "" ]; then
  echo "Database canister principal ID not provided, extracting it from `dfx canister create database` command..."
  # it's weird that the dfx command outputs to stderr...
  export DATABASE_CANISTER_PRINCIPAL_ID=$(dfx canister create database --no-wallet 2>&1 | sed -n '2s/^.*id: //p')
  echo "Database canister principal ID: $DATABASE_CANISTER_PRINCIPAL_ID"
fi

if [ "$RDF_DATABASE_BASE_URL" = "" ]; then
  echo "Please provide an RDF database base URL"
  exit 1
fi

if [ "$RDF_DATABASE_API_KEY" = "" ]; then
  echo "Please provide an RDF database API key"
  exit 1
fi

if [ "$1" = "--backend" ]; then

  echo "Deploying only BACKEND canisters..."

  # this command will deploy also the database canister
  dfx deploy omnia_backend --no-wallet --argument "(null, \"$DATABASE_CANISTER_PRINCIPAL_ID\", \"$RDF_DATABASE_BASE_URL\", \"$RDF_DATABASE_API_KEY\")"
else
  echo "Deploying ALL canisters..."

  dfx deploy --no-wallet --argument "(null, \"$DATABASE_CANISTER_PRINCIPAL_ID\", \"$RDF_DATABASE_BASE_URL\", \"$RDF_DATABASE_API_KEY\")"
fi
