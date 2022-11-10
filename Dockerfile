FROM rust:slim

RUN apt-get update && apt-get install -y libssl-dev pkg-config

WORKDIR /usr/src/entropy_worker
COPY . .
RUN cargo install --path .

RUN mkdir /data
WORKDIR /data

CMD ["entropy", "worker", "start"]