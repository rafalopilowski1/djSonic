FROM rust


COPY . /usr/src/djSonic

WORKDIR /usr/src/djSonic

RUN apt-get update && apt-get install ffmpeg -y

RUN cargo install --path . --profile release

RUN rm -rf /src/target

CMD ["djSonic"]