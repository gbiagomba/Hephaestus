# Multi-stage build for a small final image

FROM rust:1.80-slim AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs
RUN cargo build --release || true
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
RUN useradd -m -u 10001 appuser
WORKDIR /home/appuser
COPY --from=build /app/target/release/hephaestus /usr/local/bin/hephaestus
USER appuser
ENTRYPOINT ["/usr/local/bin/hephaestus"]
CMD ["--help"]
