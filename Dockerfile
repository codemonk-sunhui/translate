FROM rustlang/rust:nightly-alpine

COPY target/debug/translate ./translate

EXPOSE 8000

ENTRYPOINT "./translate"