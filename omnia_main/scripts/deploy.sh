#!/bin/bash

source .env

# we need to pass the database canister id to omnia_backend canister

if [ "$DATABASE_CANISTER_ID" = "" ]; then
  echo "Database canister ID not provided, extracting it from `dfx canister create database` command..."
  # it's weird that the dfx command outputs to stderr...
  export DATABASE_CANISTER_ID=$(dfx canister create database --no-wallet 2>&1 | sed -n '2s/^.*id: //p')
  echo "Database canister ID: $DATABASE_CANISTER_ID"
fi

if [ "$1" = "--backend" ]; then

  echo "Deploying only BACKEND canisters..."

  # this command will deploy also the database canister, because it's a dependency of omnia_backend. See the dfx.json file.
  dfx deploy omnia_backend --no-wallet --argument "(null, \"$DATABASE_CANISTER_ID\")"
else
  echo "Deploying ALL canisters..."

  dfx deploy --no-wallet --argument "(null, \"$DATABASE_CANISTER_ID\")"
fi
