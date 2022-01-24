FROM rust:alpine as chef
RUN apk add musl-dev opus-dev openssl-dev -f
ARG TARGET="x86_64-unknown-linux-musl"
RUN cargo install cargo-chef --target=${TARGET}

FROM chef AS planner
WORKDIR /planner
COPY . /planner
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder

ARG TARGET="x86_64-unknown-linux-musl"

WORKDIR /builder/

COPY --from=planner planner/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . /builder/
RUN cargo build --release --target=${TARGET}

FROM alpine:latest as ffmpeg-runner

WORKDIR /usr/local/bin/

COPY --from=builder builder/target/release/djSonic /usr/local/bin/
RUN apk add ffmpeg -f

ENTRYPOINT ["djSonic"]