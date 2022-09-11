#!/usr/bin/env bash

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

if [[ "$1" =~ ^.*\.(rs|sql)$ ]]; then
    touch "$SCRIPT_DIR/migrations/$(date -u '+%Y%m%d%H%M%S')_$1"
else
    mkdir "$SCRIPT_DIR/migrations/$(date -u '+%Y%m%d%H%M%S')_$1"
    cat > "$SCRIPT_DIR/migrations/$(date -u '+%Y%m%d%H%M%S')_$1/mod.rs" << EOM
use sqlx::SqliteConnection;

pub async fn execute(conn: &mut SqliteConnection) -> eyre::Result<()> {
    todo!()
}
EOM
fi
