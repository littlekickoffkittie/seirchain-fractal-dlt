# Use the official Rust image as a parent image
FROM rust:1.79

# Set the working directory in the container
WORKDIR /usr/src/seirchain

# Copy the local package files to the container's workspace
COPY . .

# Build the project
RUN cargo build --release

# Command to run the executable
CMD ["./target/release/server"]
