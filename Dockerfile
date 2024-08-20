FROM rust:alpine3.20 AS builder
WORKDIR /server-app
COPY . .
RUN apk add --no-cache protoc musl-dev openssl-dev
RUN cargo install --bin serverdb --path .

FROM alpine:3.20
COPY --from=builder /usr/local/cargo/bin/serverdb /usr/bin/serverdb
USER 1000
CMD ["serverdb"]
