FROM docker.finogeeks.club/base/node-rust-build:16

COPY target/debug/translate /opt/translate

EXPOSE 8000

ENTRYPOINT /opt/translate