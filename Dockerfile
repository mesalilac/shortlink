FROM rust:1.88.0 AS rust_builder

WORKDIR /src

# cache deps
COPY backend/Cargo.toml backend/Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# real build
COPY backend .
RUN cargo build --release

FROM node:20.19.3-slim AS node_builder

WORKDIR /src

# cache deps
COPY frontend/package*.json ./
RUN npm install

# real build
COPY ./frontend .
RUN npm run build

# =============
# runtime image
# =============

FROM debian:trixie-backports

RUN apt-get update && apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=rust_builder /src/target/release/shortlink /app/
COPY --from=node_builder /src/dist /app/frontend

EXPOSE 8080
VOLUME /app/data

CMD ["./shortlink", "--data-dir", "/app/data", "--frontend-dir", "/app/frontend"]

