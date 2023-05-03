FROM rust:latest

WORKDIR /app

COPY . .

RUN set -xeu \
    apt-get update -y; \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    libpq-dev;

RUN cargo build --release

CMD ["cargo", "run", "--release"]
