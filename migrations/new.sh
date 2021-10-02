#!/usr/bin/env bash

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

touch "$SCRIPT_DIR/migrations/$(date -u '+%Y%m%d%H%M%S')_$1.sql"
