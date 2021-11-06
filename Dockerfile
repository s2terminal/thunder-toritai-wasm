FROM rust:1.56-slim
WORKDIR /app

RUN apt-get update && apt-get install -y \
  git \
  curl \
  build-essential \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

RUN cargo install --locked miniserve
RUN cargo install wasm-pack

# build
COPY ./ ./
RUN wasm-pack build --target web --out-name wasm --out-dir ./static

CMD miniserve ./static --index index.html
