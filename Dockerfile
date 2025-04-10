# Base builder
FROM rust:1.75 as builder
WORKDIR /app
COPY . .

# Server builder
FROM builder as server-builder
RUN cargo build --release -p fullrstack-server

# Web builder
FROM builder as web-builder
RUN cargo build --release -p fullrstack-web
RUN cargo install trunk
RUN cd crates/web && trunk build --release

# Server runtime
FROM debian:bookworm-slim as server
RUN apt-get update && apt-get install -y libssl-dev ca-certificates
COPY --from=server-builder /app/target/release/fullrstack-server /usr/local/bin/
EXPOSE 8080 9090
CMD ["fullrstack-server"]

# Web runtime
FROM nginx:alpine as web
COPY --from=web-builder /app/crates/web/dist /usr/share/nginx/html
COPY deployment/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 3000 