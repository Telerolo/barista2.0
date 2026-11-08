# Этап сборки
FROM rust:1.75-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Этап выполнения
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/barista_backend /app/barista_backend

EXPOSE 8000
CMD ["./barista_backend"]
