#!/usr/bin/env sh
set -e

while (true) do
   ./run.sh "$@"

   # show result
   exitcode=$?
   echo "exit code of command is $exitcode"
done
