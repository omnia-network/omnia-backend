#!/bin/bash

source .env

# we need to pass the database canister id to omnia_backend canister and
# the omnia_backend canister id to database
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

echo "Deploying canisters..."

dfx deploy --no-wallet --argument "(\"$OMNIA_BACKEND_CANISTER_ID\", \"$DATABASE_CANISTER_ID\")"
