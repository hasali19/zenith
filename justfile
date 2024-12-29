set positional-arguments

clippy:
    cargo clippy --workspace --all-features -- -Dwarnings

test *args='':
    cargo nextest run --features mocks "$@"

coverage:
    cargo llvm-cov nextest --features mocks --json --hide-instantiations --ignore-filename-regex external | llvm-cov-pretty

new-migration name:
    ./server/db/new_migration.sh {{name}}
