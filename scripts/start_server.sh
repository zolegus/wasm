#!/usr/bin/env bash
set -eu
script_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )
cd "$script_path/.."

PORT=80

echo "ensuring basic-http-server is installed…"
cargo install basic-http-server

echo "starting server…"
echo "serving at http://0.0.0.0:${PORT}"

(cd web && basic-http-server --addr 0.0.0.0:${PORT} .)
