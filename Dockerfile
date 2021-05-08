FROM rust:1.49-buster as builder-rust
WORKDIR /usr/src/zenith
COPY Cargo.* ./
COPY zenith zenith
RUN cargo install --path zenith

FROM node:14 as builder-node
WORKDIR /app
COPY client/web/package.json .
RUN yarn
COPY client/web .
RUN yarn build

FROM debian:buster-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libssl1.1 libcurl4 ffmpeg
COPY --from=builder-rust /usr/local/cargo/bin/zenith /usr/local/bin/zenith
COPY --from=builder-node /app/dist ./client/web/dist
CMD ["zenith"]
