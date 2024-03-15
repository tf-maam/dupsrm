FROM rust:slim-buster
WORKDIR /usr/src/dupsrm
COPY . .

RUN cargo build
RUN cargo build --release
RUN cargo test
RUN cargo install --path .

CMD ["dupsrm"]
