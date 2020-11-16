FROM rust:nightly as build-env
WORKDIR /app
ADD . /app
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10
COPY --from=build-env /app/target/release/mon_oeil_srv /
CMD ["./mon_oeil_srv"]