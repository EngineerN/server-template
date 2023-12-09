# Build Web Data
FROM node:21.4.0-bookworm as web-builder
WORKDIR /build
COPY . .
RUN npm install
RUN npm run build --workspace=crates/lib-web

# Build Rust Project
FROM rust:1.74.0-bookworm as rust-builder
WORKDIR /build
COPY . .
COPY --from=web-builder /build/crates/lib-web/dist /build/crates/lib-web/dist
ENV SKIP_NPM_BUILD true
ENV SKIP_RUSTFMT true
RUN cargo build --release

# Build Rust License List
FROM rust:1.74.0-bookworm as rust-license
WORKDIR /build
COPY . .
RUN cargo install cargo-about
RUN cargo about generate -c .cargo-about/about.toml -o RUST_LICENSE.txt .cargo-about/about.hbs

# Combine Projects
FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=web-builder /build/crates/lib-web/dist/LICENSE.txt WEB_LICENSE.txt
COPY --from=rust-builder /build/target/release/server server
COPY --from=rust-license /build/RUST_LICENSE.txt RUST_LICENSE.txt
EXPOSE 3000
ENTRYPOINT ["./server"]
