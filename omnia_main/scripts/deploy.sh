#!/bin/sh

# we need to pass the database canister principal id to omnia_backend canister
# dfx deploy --argument '(null, "<database-canister-principal-id>")'
# the first null argument is needed by internet_identity canister
# TODO: read db canister id from .dfx/canister_ids.json (maybe use a node script for that)

if [ "$1" = "--backend" ]; then
  echo "Deploying only BACKEND canisters..."

  dfx deploy omnia_backend --argument '(null, "rrkah-fqaaa-aaaaa-aaaaq-cai")'
else
  echo "Deploying ALL canisters..."

  dfx deploy --argument '(null, "rrkah-fqaaa-aaaaa-aaaaq-cai")'
fi
