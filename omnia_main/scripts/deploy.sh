#!/bin/sh

# we need to pass the database canister principal id to omnia_backend canister
# dfx deploy --argument '("<database-canister-principal-id>")'
# TODO: read db canister id from .dfx/canister_ids.json (maybe use a node script for that)

if [ "$1" = "--backend" ]; then
  echo "Deploying only BACKEND canisters..."

  dfx deploy omnia_backend --argument '("rrkah-fqaaa-aaaaa-aaaaq-cai")'
else
  echo "Deploying ALL canisters..."

  dfx deploy --argument '("rrkah-fqaaa-aaaaa-aaaaq-cai")'
fi
