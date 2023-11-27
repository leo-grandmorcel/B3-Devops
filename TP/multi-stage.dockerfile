# docker build -t multi-stage -f multi-stage.dockerfile .
# docker run -d -p 8081:8080 --name multi-stage multi-stage 
FROM rust:slim-buster AS Builder
WORKDIR /build

RUN adduser --no-create-home --disabled-login --group --system builder
RUN chown builder -R /build
USER builder

RUN cargo new --bin api
WORKDIR /build/api
COPY Cargo.* ./
ENV RUSTFLAGS='-C target-feature=+crt-static'
RUN cargo build --release
RUN rm src/*.rs
COPY ./src ./src
RUN rm ./target/release/deps/api*
RUN cargo build --release

FROM scratch
COPY --from=Builder /build/api/target/release/api /app
CMD ["/app"]