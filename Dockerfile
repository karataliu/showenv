FROM rust:1.24-stretch AS build
WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:stretch
COPY --from=build /build/target/release/showenv /usr/local/bin
EXPOSE 8080
CMD showenv
