# Use a Rust official image as the base image
FROM rust:latest

# Install PostgreSQL
RUN apt-get update && apt-get install -y postgresql postgresql-contrib

# Set the working directory
WORKDIR /usr/src/myapp

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Build the dependencies
RUN cargo build --release

# Copy the source code
COPY . .

# Copy the .env file
COPY .env .env

# Build the application
RUN cargo install --path .

# Expose the port the app runs on
EXPOSE 8080

# Start the PostgreSQL service and run the application
CMD service postgresql start && ./target/release/my_project