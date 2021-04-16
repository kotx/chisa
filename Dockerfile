FROM rust:alpine
RUN apk add build-base
WORKDIR /usr/src/chisa
COPY . .
RUN cargo install --path .

CMD ["chisa"]
