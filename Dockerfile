FROM rust:slim as chef

RUN apt-get update -y && apt-get install libopus-dev libssl-dev pkg-config -y && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-chef

FROM chef AS planner

WORKDIR /planner
COPY . /planner
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder

WORKDIR /builder/

COPY --from=planner planner/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . /builder/
RUN cargo build --release

FROM ubuntu:latest AS runner

WORKDIR /usr/local/bin/

COPY --from=builder builder/target/release/djSonic /usr/local/bin/
COPY --from=mwader/static-ffmpeg:5.0 /ffmpeg /usr/local/bin/
COPY --from=mwader/static-ffmpeg:5.0 /ffprobe /usr/local/bin/

RUN apt-get update -y && apt-get install ca-certificates libopus0 --no-install-recommends -y && rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["djSonic"]