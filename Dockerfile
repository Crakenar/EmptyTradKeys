# Use an official Rust runtime as a parent image
FROM rust:1.75

# Set the working directory to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . .

# Build the application
RUN cargo build --release

# Expose the port that the application runs on
EXPOSE 8080


# Run the application
CMD ["./target/release/empty_array_keys"]