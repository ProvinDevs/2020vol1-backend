FROM rust as build
RUN mkdir /src
WORKDIR /src
ADD Cargo.toml Cargo.lock /src/
ADD src /src/src 
RUN cargo build --release


FROM debian
COPY --from=build /src/target/release/hacku_2020_backend /app/
WORKDIR /app
ENTRYPOINT [ "/app/" ]

