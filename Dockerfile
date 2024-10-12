# Use the official Rust image as a base
FROM rust:1.81

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a new directory for the source files
RUN mkdir src

# Copy the source files
COPY src/ src/

# Build the project
RUN cargo build --release

# Copy the entry point
COPY . .

# Command to run the application
CMD ["./target/release/your_app_name"] # substitua 'your_app_name' pelo nome do seu executável
