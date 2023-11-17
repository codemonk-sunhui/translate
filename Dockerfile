FROM docker.finogeeks.club/base/rust:1.20.0

COPY target/debug/translate ./translate

EXPOSE 8000

ENTRYPOINT "./translate"