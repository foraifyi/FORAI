#!/bin/bash

# Kill any existing validator
pkill solana-test-validator

# Start new validator
if [ -f "./program-id.json" ]; then
    PROGRAM_ID=$(cat program-id.json | tr -d '"')
    solana-test-validator \
        --reset \
        --bpf-program $PROGRAM_ID ./target/deploy/forai_solana.so \
        --quiet &
else
    solana-test-validator \
        --reset \
        --quiet &
fi

# Wait for validator to start
sleep 5

# Configure CLI to use local cluster
solana config set --url http://localhost:8899 