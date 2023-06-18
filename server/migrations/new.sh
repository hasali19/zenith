#!/usr/bin/env bash

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

filename=""

if [[ "$1" =~ ^.*\.(rs|sql)$ ]]; then
    filename="$SCRIPT_DIR/migrations/$(date -u '+%Y%m%d%H%M%S')_$1"
    touch "$filename"
else
    filename="$SCRIPT_DIR/migrations/$(date -u '+%Y%m%d%H%M%S')_$1/mod.rs"
    mkdir "$SCRIPT_DIR/migrations/$(date -u '+%Y%m%d%H%M%S')_$1"
    cat > "$filename" << EOM
use sqlx::SqliteConnection;

pub async fn execute(conn: &mut SqliteConnection) -> eyre::Result<()> {
    todo!()
}
EOM
fi

echo "created $filename"
