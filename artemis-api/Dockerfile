FROM rust:latest as builder
WORKDIR /usr/src/artemis
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim as runtime
COPY --from=builder /usr/src/artemis/target/release/artemis-api .
ENTRYPOINT ["./artemis-api"]