# Stage 1: Build Frontend
FROM node:20-alpine AS frontend-builder
WORKDIR /app/client
COPY client/package*.json ./
RUN npm install
COPY client/ .
# Pass the VITE_APP_SOCKET_URL as a build-arg and then set it as an environment variable for the build process
ARG VITE_APP_SOCKET_URL=http://localhost:3000
ENV VITE_APP_SOCKET_URL=$VITE_APP_SOCKET_URL
RUN npm run build

# Stage 2: Build Backend
FROM rust:latest AS backend-builder
WORKDIR /app/server
# Copy manifest files to cache dependencies
COPY server/Cargo.toml server/Cargo.lock ./
# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
# Remove the dummy build artifacts
RUN rm -rf target/release/deps/server*
# Copy the actual source code
COPY server/src ./src
# Build the actual application
RUN cargo build --release

# Stage 3: Final Image
FROM debian:bookworm-slim
WORKDIR /app

# Install OpenSSL (often required for Rust HTTP clients/servers)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy backend binary
COPY --from=backend-builder /app/server/target/release/server .

# Copy frontend build to 'dist' folder (which the server serves)
COPY --from=frontend-builder /app/client/dist ./dist

# Expose the port
EXPOSE 3000

# Run the server
CMD ["./server"]
