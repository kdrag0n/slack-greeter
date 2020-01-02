FROM rust:1.40 as builder
WORKDIR /usr/src/slack-greeter
COPY . .
RUN cargo build --release

FROM bitnami/minideb:buster
RUN install_packages libssl1.1 ca-certificates
COPY --from=builder /usr/src/slack-greeter/target/release/slack-greeter /usr/local/bin/slack-greeter
RUN mkdir -p /data
WORKDIR /data
CMD ["slack-greeter"]
