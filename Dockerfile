FROM ubuntu:20.04
WORKDIR app
RUN apt-get update && apt-get install -y libssl1.1 libcurl4 ffmpeg
COPY target/release/zenith /usr/local/bin/zenith
COPY client/web/dist client/web/dist
RUN chmod +x /usr/local/bin/zenith
CMD ["/usr/local/bin/zenith"]
