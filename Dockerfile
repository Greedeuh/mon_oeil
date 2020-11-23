FROM node as vue-env
WORKDIR /app
ADD . /app
RUN cd mon_oeil_front && yarn && yarn build

FROM rustlang/rust:nightly as actix-env
WORKDIR /app
ADD . /app
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10
COPY --from=vue-env /app/mon_oeil_front/dist /mon_oeil_front/dist
COPY --from=actix-env /app/target/release/mon_oeil_srv /
CMD ["./mon_oeil_srv"]