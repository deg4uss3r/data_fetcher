FROM rust:latest

WORKDIR "/data_fetcher"

ADD . .

RUN apt update && apt upgrade -y
RUN rustup self update && rustup update

RUN cargo build --release

ENTRYPOINT ./target/release/data_fetcher 
