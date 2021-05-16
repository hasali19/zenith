FROM rust:1.49-buster as planner
WORKDIR app
RUN cargo install cargo-chef
COPY Cargo.* ./
COPY zenith zenith
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.49-buster as cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.49-buster as builder-rust
WORKDIR app
COPY Cargo.* ./
COPY zenith zenith
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --bin zenith --release

FROM node:14 as builder-node
WORKDIR /app
COPY client/web/package.json .
RUN yarn
COPY client/web .
RUN yarn build

FROM debian:buster-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libssl1.1 libcurl4 ffmpeg
COPY --from=builder-rust /app/target/release/zenith /usr/local/bin
COPY --from=builder-node /app/dist ./client/web/dist
CMD ["/usr/local/bin/zenith"]
