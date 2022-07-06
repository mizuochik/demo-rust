FROM rust:1.62 AS rust
WORKDIR /build
COPY . ./
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=target/release \
    cargo build --release --bin webapi && cp target/release/webapi demo-webapi

FROM alpine
WORKDIR /
COPY --from=rust /build/demo-webapi .
ENTRYPOINT [ "/demo-webapi" ]
