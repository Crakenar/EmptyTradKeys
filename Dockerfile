# Use an official Rust runtime as a parent image
FROM rust:1.75
RUN cargo install cargo-watch
# Set the working directory to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . .

# Build the application
RUN cargo build --release

# Run the application
#RUN chmod +x ./target/release/empty_array_keys
CMD ["./target/release/empty_array_keys"]