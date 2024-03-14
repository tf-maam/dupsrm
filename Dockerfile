FROM rust:slim-buster
WORKDIR /usr/src/dupsrm
COPY . .

RUN cargo install --path .

CMD ["dupsrm"]
