FROM docker.finogeeks.club/base/node-rust:16.13.1-alpine

COPY target/debug/translate ./translate

ENTRYPOINT "./translate"