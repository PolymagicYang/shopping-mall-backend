FROM rust:latest as builder
WORKDIR /usr/src/shopping-mall-backend
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/shopping-mall-backend /usr/local/bin/shopping-mall-backend
CMD ["shopping-mall-backend"]