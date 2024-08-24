FROM rust:alpine3.20 AS builder
WORKDIR /server-app
COPY . .
RUN apk add --no-cache protoc musl-dev openssl-dev upx  
RUN cargo install --bin serverdb --path .
RUN upx --ultra-brute -qq /usr/local/cargo/bin/serverdb && upx -t /usr/local/cargo/bin/serverdb

FROM scratch
COPY --from=builder /usr/local/cargo/bin/serverdb /usr/bin/serverdb
USER 1000
CMD ["serverdb"]
