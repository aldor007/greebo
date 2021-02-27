# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

WORKDIR /usr/src/myapp

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

COPY src src
COPY build.rs build.rs
COPY proto proto

RUN rustup component add rustfmt
RUN cargo build --release

RUN rm -f target/release/deps/greebo**

COPY . .

RUN cargo build --release

RUN cargo install --path .

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

COPY --from=cargo-build /usr/local/cargo/bin/greebo /usr/local/bin/greebo

CMD ["greebo"]
