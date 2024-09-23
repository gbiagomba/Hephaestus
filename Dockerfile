# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/hephaestus

# Copy the Cargo.toml and Cargo.lock files first
COPY Cargo.toml Cargo.lock ./

# This step will build only the dependencies (to leverage Docker layer caching)
RUN cargo build --release || true

# Now copy the source code
COPY . .

# Build the project in release mode
RUN cargo build --release

# The final command to run the program
CMD ["./target/release/hephaestus", "--help"]
