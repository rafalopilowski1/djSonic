FROM rust:alpine as chef
RUN apk add musl-dev -f --no-cache
RUN cargo install cargo-chef

FROM chef AS planner

WORKDIR /planner
COPY . /planner

RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
WORKDIR /
ARG TARGET=x86_64-unknown-linux-musl
COPY --from=planner planner/recipe.json recipe.json
RUN apk add opus-dev -f --no-cache

RUN cargo chef cook --release --recipe-path recipe.json --target=${TARGET}

COPY . .
RUN cargo install --path . --profile release --target=${TARGET}

FROM alpine:latest AS runner

WORKDIR /usr/local/bin/
COPY --from=builder /usr/local/cargo/bin/djSonic /usr/local/bin/

RUN apk add ca-certificates ffmpeg -f --no-cache
ENTRYPOINT ["djSonic"]