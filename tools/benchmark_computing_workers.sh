#!/usr/bin/env bash

# This script is meant to be run on Unix/Linux based systems
set -e

echo "*** Run benchmark for pallet-computing_workers ***"

target/production/node benchmark pallet \
  --pallet=pallet_computing_workers \
  --extrinsic="*" \
  --chain=dev \
  --steps=50 \
  --repeat=50 \
  --execution=wasm \
  --wasm-execution=compiled \
  --heap-pages=4096 \
  --output=./pallets/computing_workers/src/weights.rs \
  --template=./templates/pallet-weight-template.hbs