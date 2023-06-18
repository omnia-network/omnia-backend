#!/bin/bash

# dids are generated using `generate_candid_interface` test in each canister 
cargo test --package omnia_backend --lib -- tests::generate_candid_interface
cargo test --package database --lib -- tests::generate_candid_interface
cargo test --package application_placeholder --lib -- tests::generate_candid_interface
