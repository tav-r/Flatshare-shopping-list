FROM rust:latest as build-env
WORKDIR /app
COPY /bot /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/flatshare_bot /
CMD ["./flatshare_bot"]
