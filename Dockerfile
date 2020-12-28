FROM rustlang/rust:nightly-buster as builder-rust
WORKDIR /usr/src/zenith
COPY Cargo.* ./
COPY . .
RUN cargo install --path zenith

FROM node:14 as builder-node
WORKDIR /app
COPY zenith_web/package.json .
RUN npm install
COPY zenith_web .
RUN npm run build

FROM ubuntu:focal
WORKDIR /app
RUN apt-get update && apt-get install -y libssl1.1 libcurl4 ffmpeg
COPY --from=builder-rust /usr/local/cargo/bin/zenith /usr/local/bin/zenith
COPY --from=builder-node /app/build ./zenith_web/build
CMD ["zenith"]
