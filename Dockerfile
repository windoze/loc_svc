FROM messense/rust-musl-cross:x86_64-musl AS builder
WORKDIR /usr/src/
COPY . ./
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Bundle Stage
FROM alpine
RUN apk add --update openssl bash
COPY --from=builder /usr/src/target/x86_64-unknown-linux-musl/release/loc_svc /app/loc_svc
# USER 1000
WORKDIR /app
EXPOSE 8080
CMD ["/app/loc_svc"]
