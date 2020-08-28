FROM rust as build
RUN mkdir /src
WORKDIR /src
ADD Cargo.toml Cargo.lock /src/
ADD src /src/src 
RUN cargo build --release


FROM debian

RUN apt-get update -y; \
    apt-get install -y --no-install-recommends libssl-dev; \
    apt-get clean -y; \
    rm -rf /var/lib/apt/lists/*;

COPY --from=build /src/target/release/hacku_2020_backend /app/
WORKDIR /app
EXPOSE 8080
CMD [ "/app/hacku_2020_backend" ]
