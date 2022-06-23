FROM rust:latest as builder
WORKDIR /usr/src/shopping-mall-backend
COPY . .
RUN cargo install --path .
CMD ["shopping-mall-backend"]