FROM rust:1.75

WORKDIR /app

COPY  ./src ./src
COPY ./db ./db
COPY ./.sqlx ./.sqlx
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml 

RUN cargo build --release

CMD ["./target/release/my-first-api-project"]


