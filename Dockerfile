FROM rust:latest as build

RUN rustup traget add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /Users/alex/Developer/Rust/credex
COPY . .

RUN cd frontend && trunk build --release
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /Users/alex/Developer/Rust/credex/target/release/credex /usr/local/bin/backend
COPY --from=build /Users/alex/Developer/Rust/credex/frontend/dist /usr/local/bin/dist

WORKDIR /usr/local/bin
CMD ["backend"]