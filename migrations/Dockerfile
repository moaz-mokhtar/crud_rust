FROM rust:latest


RUN set -xeu \
    apt-get update -y; \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    libpq-dev;
RUN cargo install diesel_cli --no-default-features --features postgres;
RUN mv $(which diesel) /bin

WORKDIR /db
COPY . ./migrations/

ENTRYPOINT [ "/bin/diesel" ]
CMD ["database", "setup"]