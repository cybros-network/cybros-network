#!/usr/bin/env sh

APP_PATH=$(dirname "$(readlink -f "$0")")

deno run \
  --allow-run \
  --allow-net \
  --allow-write="$APP_PATH/data,$APP_PATH/tmp,$APP_PATH/log" \
  --allow-read="$APP_PATH/data,$APP_PATH/tmp,$APP_PATH/log" \
  "$APP_PATH"/main.ts "$@" --work-path "$APP_PATH"
