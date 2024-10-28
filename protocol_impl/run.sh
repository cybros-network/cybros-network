#!/usr/bin/env sh
set -e

DENO_PATH=$(which deno)
APP_PATH=$(dirname "$(readlink -f "$0")")
EXECUTOR_PATH="${EXECUTOR_PATH:-"$APP_PATH/examples/simple_echo"}"

deno run \
  --allow-run="$DENO_PATH" \
  --allow-env \
  --allow-net \
  --allow-sys \
  --allow-write="$APP_PATH/data,$APP_PATH/tmp,$APP_PATH/log" \
  --allow-read \
  "$APP_PATH"/main.ts --job-executor-path "$EXECUTOR_PATH" "$@"
