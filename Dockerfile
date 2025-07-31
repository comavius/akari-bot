# Stage 1: Build and Test
FROM rust:1.78 as builder

# Install mysql client development headers
RUN apt-get update && apt-get install -y libmysqlclient-dev

# Create a new empty shell project
RUN USER=root cargo new --bin akari-bot
WORKDIR /akari-bot

# Copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This build step will cache your dependencies
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/akari-bot/target \
    cargo build --release
RUN rm src/*.rs

# Copy your source code
COPY ./src ./src

# Build for release
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/akari-bot/target \
    rm ./target/release/deps/akari_bot* && \
    cargo build --release

# Run tests
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/akari-bot/target \
    cargo test --release

# Stage 2: Final image
FROM debian:slim-bullseye

# Copy the built binary from the builder stage
COPY --from=builder /akari-bot/target/release/akari-bot .

# Copy the example environment file
COPY ./.env.example ./.env.example

# Set the startup command to run your binary
CMD ["./akari-bot"]