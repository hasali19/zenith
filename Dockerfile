FROM rust:1.49-buster as builder-rust
WORKDIR app
COPY Cargo.* .
COPY zenith/Cargo.toml zenith/Cargo.toml
RUN mkdir zenith/src && \
    echo "fn main() {}" > zenith/src/main.rs && \
    cargo build --release && \
    rm -rf zenith/src
COPY zenith/src zenith/src
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
