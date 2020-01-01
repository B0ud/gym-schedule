#FROM rust:1.40.0 as builder
FROM clux/muslrust as builder
WORKDIR /usr/src/app

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN mkdir src/
RUN echo "extern crate openssl;\n#[macro_use]\nextern crate diesel;\n#[macro_use]\nextern crate diesel_migrations;\n#[macro_use]\nextern crate serde_derive;\n#[macro_use]\nextern crate log;\nextern crate r2d2;\nfn main() {println!(\"if you see this, the build is broken\")}" > src/main.rs

#RUN apt-get update
#RUN apt-get install musl-tools -y
#RUN rustup target add x86_64-unknown-linux-musl
#RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --target x86_64-unknown-linux-musl  --release
RUN cargo build --release

RUN rm -rf src/
RUN rm -f target/x86_64-unknown-linux-musl/release/gym-schedule*
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/gym-schedule*
RUN rm -f target/x86_64-unknown-linux-musl/release/gym-schedule.d


COPY src/* src/

RUN cargo build --release

#RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --target x86_64-unknown-linux-musl  --release

FROM alpine:latest

RUN apk update  && apk upgrade && apk add postgresql-dev
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/gym-schedule .


EXPOSE 3000

CMD ["./gym-schedule"]