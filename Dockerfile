FROM alpine:3.14
WORKDIR app
RUN apk --no-cache add ffmpeg
COPY target/release/zenith /usr/local/bin/zenith
COPY client/web/dist client/web/dist
RUN chmod +x /usr/local/bin/zenith
CMD ["/usr/local/bin/zenith"]
