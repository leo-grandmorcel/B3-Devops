# docker build -t single-stage -f single-stage.dockerfile .
# docker run -d -p 8080:8080 --name single-stage single-stage
FROM rust:slim-buster
WORKDIR /app

RUN adduser --no-create-home --disabled-login --group --system www
RUN chown www -R /app
USER www

RUN cargo new --bin api
WORKDIR /app/api
COPY Cargo.* ./
RUN cargo build --release
RUN rm src/*.rs
COPY ./src ./src
RUN rm ./target/release/deps/api*
RUN cargo build --release

CMD ["./target/release/api"]