FROM rust:1.62 AS rust
RUN rustup target add aarch64-unknown-linux-musl
WORKDIR /tmp/build
COPY . ./
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=target \
    cargo build --release --bin webapi --target aarch64-unknown-linux-musl \
    && cp target/aarch64-unknown-linux-musl/release/webapi /tmp/demo-webapi

FROM alpine
WORKDIR /
COPY --from=rust /tmp/demo-webapi .
ENTRYPOINT [ "/demo-webapi" ]
