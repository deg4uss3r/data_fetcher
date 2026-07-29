FROM rust:latest

WORKDIR "/data_fetcher"
ENV TZ="America/New_York"

ADD . .

RUN apt update && apt upgrade -y
RUN apt install -y tzdata
RUN rustup self update && rustup update
RUN cargo build --release

ENTRYPOINT ./target/release/data_fetcher 
