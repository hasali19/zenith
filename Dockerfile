FROM debian:buster-slim
WORKDIR app
RUN apt-get update && apt-get install -y libssl1.1 libcurl4 ffmpeg
COPY target/release/zenith /usr/local/bin/zenith
COPY client/web/dist client/web/dist
CMD ["/usr/local/bin/zenith"]
