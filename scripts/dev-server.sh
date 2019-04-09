#!/usr/bin/env bash

python -m SimpleHTTPServer &
SERVER_PID=$!
echo "got $SERVER_PID"

sigint()
{
  echo "SIGINT received, killing $SERVER_PID" > /dev/tty
  kill "$SERVER_PID" || true
  exit
}

trap 'sigint' INT

# main thread work -- builds.

while true; do \
    inotifywait ./src/*.rs ./src/**/*.rs -e modify -e move -e create -e delete -e unmount; \
    make build-dev; \
done

