FROM rust:1.79.0-slim-bullseye as build
WORKDIR /build
COPY Cargo.lock Cargo.toml ./
COPY src src
RUN cargo build --locked --release


FROM debian:bullseye-slim AS final
WORKDIR /app
COPY --from=build /build/target/release/minebot /app
CMD "./minebot"