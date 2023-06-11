#!/bin/bash

source .env

# we need to pass the database canister id to omnia_backend canister and
# the omnia_backend canister id to database
# also, we need to pass the id of the ledger canister when it is running locally
# first we create the empty canisters locally in order to get their ids, then we deploy

if [ "$DATABASE_CANISTER_ID" = "" ]; then
  echo "Database canister ID not provided, extracting it from `dfx canister create database` command..."
  # it's weird that the dfx command outputs to stderr...
  export DATABASE_CANISTER_ID=$(dfx canister create database --no-wallet 2>&1 | sed -n '2s/^.*id: //p')
  echo "Database canister ID: $DATABASE_CANISTER_ID"
fi

if [ "$OMNIA_BACKEND_CANISTER_ID" = "" ]; then
  echo "Omnia Backend canister ID not provided, extracting it from `dfx canister create omnia_backend` command..."
  # it's weird that the dfx command outputs to stderr...
  export OMNIA_BACKEND_CANISTER_ID=$(dfx canister create omnia_backend --no-wallet 2>&1 | sed -n '2s/^.*id: //p')
  echo "Omnia Backend canister ID: $OMNIA_BACKEND_CANISTER_ID"
fi

if [ "$LEDGER_CANISTER_ID" = "" ]; then
  echo "Ledger canister ID not provided, extracting it from `dfx canister create ledger` command..."
  # it's weird that the dfx command outputs to stderr...
  export LEDGER_CANISTER_ID=$(dfx canister create ledger --no-wallet 2>&1 | sed -n '2s/^.*id: //p')
  echo "Ledger canister ID: $LEDGER_CANISTER_ID"

  # if the ledger canister is still not running, we need to start it
  # create a minter identity for the local ledger canister
  # (we don't need to secure the PK, since it's only used locally)
  # (if the identity already exists, the command will fail, but we don't care)
  dfx identity new minter --storage-mode plaintext || true
  dfx identity use minter
  export MINT_ACC=$(dfx ledger account-id)

  dfx identity use default
  export OMNIA_BACKEND_ACC=$(cargo run --bin principal_2_account "$OMNIA_BACKEND_CANISTER_ID" | tail -n 1)
  echo "Omnia Backend ledger account: $OMNIA_BACKEND_ACC"

  # deploy the ledger canister, and give some tokens to Omnia Backend
  dfx deploy ledger --argument '(record {minting_account = "'${MINT_ACC}'"; initial_values = vec { record { "'${OMNIA_BACKEND_ACC}'"; record { e8s=100_000_000_000 } }; }; send_whitelist = vec {}})'

fi

echo "Deploying canisters..."

dfx deploy --no-wallet --argument "(\"$OMNIA_BACKEND_CANISTER_ID\", \"$DATABASE_CANISTER_ID\", \"$LEDGER_CANISTER_ID\")"
