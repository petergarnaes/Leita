FROM rust:1.26.0-slim

WORKDIR /usr/src/leita
COPY . .

RUN cargo install
RUN cargo build --release

CMD ["leita"]
