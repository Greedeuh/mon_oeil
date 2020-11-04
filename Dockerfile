FROM rust:1.45.0 as build-env
WORKDIR /app
ADD . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/mon_oeil_srv /
CMD ["./mon_oeil_srv"]