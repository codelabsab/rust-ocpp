# build
FROM rust:latest AS builder
WORKDIR /code
ADD Cargo.toml .
ADD src src/
RUN cargo build --release

# run
FROM frolvlad/alpine-glibc
ENV RUST_LOG INFO
WORKDIR /root
COPY --from=0 /code/target/release/ws .
CMD [ "./ws" ]
