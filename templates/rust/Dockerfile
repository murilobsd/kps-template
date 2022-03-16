FROM ekidd/rust-musl-builder:stable as chef
USER root
RUN cargo install cargo-chef
WORKDIR /{{ service_name }}

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /{{ service_name }}/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin {{ service_name }}

FROM alpine:latest as runtime
RUN addgroup -S appuser && adduser -S appuser -G appuser
EXPOSE 8080
ENV TZ=Etc/UTC
RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*
COPY --from=builder /{{ service_name }}/target/x86_64-unknown-linux-musl/release/{{ service_name }} /usr/local/bin/
USER appuser
CMD ["/usr/local/bin/{{ service_name }}"]
