FROM rust:alpine as builder
RUN apk add build-base
WORKDIR /usr/src/chisa
COPY . .
RUN cargo install --path .

FROM alpine:latest
COPY --from=builder /usr/local/cargo/bin/chisa /usr/local/bin/chisa
CMD ["chisa"]
