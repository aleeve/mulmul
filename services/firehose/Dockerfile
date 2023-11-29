# Build my app
FROM rust:bullseye as worker
WORKDIR /usr/src/firehose
COPY . .
RUN cargo install --path .

# Setup actual image
FROM rust:bullseye
COPY --from=worker /usr/local/cargo/bin/firehose /usr/local/bin/firehose
CMD ["firehose"]
