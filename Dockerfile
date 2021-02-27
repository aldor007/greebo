FROM rust:1.50.0-alpine as cargo-build

RUN apk update && apk add  \
    ca-certificates \
    git \
    curl \
    gcc \
    make \
    openssl-dev \
    clang \
    brotli-dev \
    libc6-compat \
    libc-dev \
    protoc

WORKDIR /usr/src/myapp

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

COPY src src
COPY build.rs build.rs
COPY proto proto
RUN rustup component add rustfmt && rustup target add $(uname -m)-unknown-linux-musl
RUN cargo build --release

RUN rm -f target/release/deps/greebo**

COPY . .

RUN cargo build --release  --target $(uname -m)-unknown-linux-musl

RUN cargo install --path .
# RUN /usr/local/cargo/bin/greebo
# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

COPY --from=cargo-build /usr/local/cargo/bin/greebo /usr/local/bin/greebo
RUN /usr/local/bin/greebo -h
ENTRYPOINT ["/usr/local/bin/greebo"]

EXPOSE 8080 8081
