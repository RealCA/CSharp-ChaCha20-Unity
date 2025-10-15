FROM rust:latest

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl xz-utils build-essential pkg-config libssl-dev ca-certificates mingw-w64

# Install Zig for cross-compilation
RUN curl -LO https://ziglang.org/download/0.11.0/zig-linux-x86_64-0.11.0.tar.xz && \
    tar -xf zig-linux-x86_64-0.11.0.tar.xz && \
    mv zig-linux-x86_64-0.11.0 /opt/zig && \
    ln -s /opt/zig/zig /usr/local/bin/zig && \
    rm zig-linux-x86_64-0.11.0.tar.xz

# Install cargo-zigbuild
RUN cargo install cargo-zigbuild

# Add all targets Unity could need
RUN rustup target add \
    x86_64-pc-windows-msvc \
    i686-pc-windows-msvc \
    x86_64-unknown-linux-gnu \
    aarch64-unknown-linux-gnu \
    x86_64-apple-darwin \
    aarch64-apple-darwin

WORKDIR /app
COPY . .

# Build all variants and move them into /dist for Unity
CMD ["bash", "-c", "\
    mkdir -p /dist/windows/x86_64 /dist/windows/x86 /dist/linux/x86_64 /dist/linux/arm64 /dist/macos/x86_64 /dist/macos/arm64 && \
    cargo zigbuild --release --target x86_64-unknown-linux-gnu && \
    cp target/x86_64-unknown-linux-gnu/release/libSimdVector.so /dist/linux/x86_64/ && \
    cargo zigbuild --release --target aarch64-unknown-linux-gnu && \
    cp target/aarch64-unknown-linux-gnu/release/libSimdVector.so /dist/linux/arm64/ && \
    cargo zigbuild --release --target x86_64-apple-darwin && \
    cp target/x86_64-apple-darwin/release/libSimdVector.dylib /dist/macos/x86_64/ && \
    cargo zigbuild --release --target aarch64-apple-darwin && \
    cp target/aarch64-apple-darwin/release/libSimdVector.dylib /dist/macos/arm64/ && \
    echo 'All builds done. Output in /dist'"]