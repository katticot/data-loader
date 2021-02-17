FROM rust as cargo-build

WORKDIR /usr/src/myapp
RUN cargo build --release --bin load

# RUN cargo install --path .

# CMD ["load"]
RUN rm -f target/release/deps/myapp*


RUN cargo install --path .
FROM alpine:latest

COPY --from=cargo-build /usr/local/cargo/bin/load /usr/local/bin/myapp

CMD ["myapp"]