#!/bin/bash

# if one of the arguments is --clean, we clean the .env file
# we don't know the position of the --clean argument, so we need to check all of them
if [[ "$@" == *"--clean"* ]]; then
  echo "Cleaning .env file..."
  echo -n "" > .env
fi

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
  export ARCHIVE_CONTROLLER=$(dfx identity get-principal)
  export OMNIA_BACKEND_ACC=$(cargo run --bin principal_to_account "$OMNIA_BACKEND_CANISTER_ID" | tail -n 1)
  echo "Omnia Backend ledger account: $OMNIA_BACKEND_ACC"

  # first deploy the ledger with private did
  npx json -I -f dfx.json -e 'this.canisters.ledger.candid = "icp-ledger/ledger.private.did"'

  # deploy the ledger canister with archiving enabled, and give some tokens to Omnia Backend
  dfx deploy ledger --argument '(record {minting_account = "'${MINT_ACC}'"; initial_values = vec { record { "'${OMNIA_BACKEND_ACC}'"; record { e8s=100_000_000_000 } }; }; send_whitelist = vec {}; archive_options = opt record { trigger_threshold = 20; num_blocks_to_archive = 10; controller_id = principal "'${ARCHIVE_CONTROLLER}'" }})'

  # then set the interface to public did
  npx json -I -f dfx.json -e 'this.canisters.ledger.candid = "icp-ledger/ledger.public.did"'

  # add the ledger canister id to the .env file
  echo -e "\n\nLEDGER_CANISTER_ID='$LEDGER_CANISTER_ID'" >> .env
fi

echo "Deploying canisters..."

declare -a CANISTERS_TO_DEPLOY=("ledger" "database" "omnia_backend")

# if the --tests argument is provided, we also deploy the application_placeholder canister
if [[ "$@" == *"--tests"* ]]; then
  echo "TESTS deployment. Deploying also the application_placeholder canister..."
  CANISTERS_TO_DEPLOY+=("application_placeholder")
fi

for canister in "${CANISTERS_TO_DEPLOY[@]}"
do
  echo "Deploying $canister..."
  dfx deploy $canister --no-wallet --argument "(\"$OMNIA_BACKEND_CANISTER_ID\", \"$DATABASE_CANISTER_ID\", \"$LEDGER_CANISTER_ID\")"
done
