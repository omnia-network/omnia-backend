#!/bin/bash

if [ "$1" = "" ]; then
  echo "Generating Typescript types for all canisters..."
  dfx generate
  exit 0
fi

if [ "$1" = "--backend" ]; then

  echo "Generating Typescript types for omnia_backend canister..."

  dfx generate omnia_backend

fi
