FROM rust:slim as chef
RUN cargo install cargo-chef

FROM chef AS planner

WORKDIR /planner
COPY . /planner

RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
WORKDIR /
COPY --from=planner planner/recipe.json recipe.json
RUN apt-get update -y
RUN apt-get install libopus-dev build-essential -y

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo install --path . --profile release

FROM ubuntu:latest AS runner

WORKDIR /usr/local/bin/
COPY --from=builder /usr/local/cargo/bin/djSonic /usr/local/bin/

RUN apt-get install ca-certificates ffmpeg -y
ENTRYPOINT ["djSonic"]