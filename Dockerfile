FROM alpine:latest as builder

RUN apk add --no-cache rust cargo

WORKDIR /usr/src/thedate

COPY . .

RUN cargo build --release

FROM alpine:latest

RUN apk add --no-cache libgcc

COPY --from=builder /usr/src/thedate/target/release/thedate /usr/local/bin/thedate

EXPOSE 8080

CMD ["thedate"]
