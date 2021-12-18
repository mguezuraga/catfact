# https://dev.to/rogertorres/first-steps-with-docker-rust-30oi
# https://github.com/rogertorres/dev.to/tree/main/docker/holodeck

# Rust as the base image

# Rust as the base image
FROM rust:1.57 as build

# Create a new empty shell project
RUN USER=root cargo new --bin catfact
WORKDIR /catfact

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/catfact*
RUN cargo build --release

# The final base image
FROM debian:buster-slim

# Copy from the previous build
COPY --from=build /catfact/target/release/catfact /usr/src/catfact
# COPY --from=build /holodeck/target/release/holodeck/target/x86_64-unknown-linux-musl/release/holodeck .

# Run the binary
CMD ["/usr/src/catfact"]

#CMD ["catfact"]