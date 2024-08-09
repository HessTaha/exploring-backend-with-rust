FROM rust:1.75 as build

RUN USER=root cargo new --bin simple-backend
WORKDIR /simple-backend

RUN cargo build --release
RUN rm src/*.rs
RUN rm ./target/release/deps/simple_backend*

COPY  ./src ./src
COPY ./db ./db
COPY ./.sqlx ./.sqlx
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml 

RUN cargo build --release

FROM rust:1.75

COPY --from=build /simple-backend/target/release/my-first-api-project .

CMD ["./my-first-api-project"]



