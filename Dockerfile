FROM docker.finogeeks.club/base/node-rust:16.13.1-alpine

COPY target/debug/translate ./translate

EXPOSE 8000

ENTRYPOINT "./translate"