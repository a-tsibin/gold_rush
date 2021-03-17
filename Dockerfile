FROM rust:1.53

WORKDIR /CLionProjects/gold_rush
COPY . .

RUN cargo install --path .

CMD ["gold_rush"]