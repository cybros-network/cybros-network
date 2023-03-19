#!/usr/bin/env sh
set -e

pushd .

# The following lines ensure we run from the project root
PROJECT_ROOT=$(dirname $(dirname "$(readlink -f "$0")"))
cd "$PROJECT_ROOT"

echo "*** Run benchmark for pallet-computing_workers ***"

./target/production/node benchmark pallet \
  --pallet=pallet_offchain_computing_workers \
  --extrinsic="*" \
  --chain=dev \
  --steps=50 \
  --repeat=50 \
  --no-storage-info \
  --no-median-slopes \
  --no-min-squares \
  --execution=wasm \
  --wasm-execution=compiled \
  --heap-pages=4096 \
  --output=./pallets/offchain_computing_workers/src/weights.rs \
  --template=./templates/pallet-weight-template.hbs

popd
