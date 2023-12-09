clippy:
    cargo clippy --workspace --all-features -- -Dwarnings

test:
    cargo nextest run --features mocks

coverage:
    cargo llvm-cov nextest --features mocks --json --hide-instantiations --ignore-filename-regex external | llvm-cov-pretty
