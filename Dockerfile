#FROM rust:1.79-alpine
#LABEL authors="https://github.com/AmmRage"
#COPY ./ ./
#RUN apk add build-base
#RUN cargo build --verbose
#CMD ["./target/debug/better-one-tab-2024-server", "9401"]


FROM rust:1.79-alpine AS build
RUN apk add build-base
WORKDIR /app
COPY . .
RUN cargo build --verbose \
    && rm -rf ./target/debug/.fingerprint \
    && rm -rf ./target/debug/build \
    && rm -rf ./target/debug/examples \
    && rm -rf ./target/debug/incremental

FROM alpine:3.20 AS app
WORKDIR /app
COPY --from=build /app/target/debug .
CMD ["/app/better-one-tab-2024-server", "9401"]

