#!/bin/bash

# we need to pass the database canister principal id to omnia_backend canister
# dfx deploy --argument '(null, "<database-canister-principal-id>")'
# the first null argument is needed by internet_identity canister
# TODO: read database canister id from .dfx/canister_ids.json automatically (maybe use a node script for that)

if [ "$1" = "" ]; then
  echo "Please provide a database canister id"
  exit 1
fi

if [ "$1" = "--backend" ]; then

  if [ "$2" = "" ]; then
    echo "Please provide a database canister id"
    exit 1
  fi

  echo "Deploying only BACKEND canisters..."

  dfx deploy omnia_backend --argument "(null, \"$2\")"
else
  echo "Deploying ALL canisters..."

  dfx deploy --argument "(null, \"$1\")"
fi
