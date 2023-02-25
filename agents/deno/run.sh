#!/usr/bin/env sh

DENO_PATH=$(which deno)
APP_PATH=$(dirname "$(readlink -f "$0")")

deno run \
  --allow-run="$DENO_PATH" \
  --allow-net \
  --allow-write="$APP_PATH/data,$APP_PATH/tmp,$APP_PATH/log" \
  --allow-read="$APP_PATH/data,$APP_PATH/tmp,$APP_PATH/log,$DENO_PATH" \
  "$APP_PATH"/main.ts "$@" --work-path "$APP_PATH"
