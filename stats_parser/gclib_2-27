FROM ubuntu:18.04

ARG DEBIAN_FRONTEND=noninteractive

# Install basic dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    gcc \
    g++ \
    libc6-dev \
    libssl-dev \
    pkg-config \
    git \
    cmake \
    file \
    wget \
    python3 \
    sudo \
    xz-utils \
    && rm -rf /var/lib/apt/lists/*

# Install Rust target for aarch64
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add aarch64-unknown-linux-gnu

# Install cross-compilation toolchain from ARM
RUN mkdir -p /opt/toolchain && \
    cd /opt/toolchain && \
    wget https://developer.arm.com/-/media/Files/downloads/gnu-a/8.3-2019.03/binrel/gcc-arm-8.3-2019.03-x86_64-aarch64-linux-gnu.tar.xz && \
    tar xf gcc-arm-8.3-2019.03-x86_64-aarch64-linux-gnu.tar.xz && \
    rm gcc-arm-8.3-2019.03-x86_64-aarch64-linux-gnu.tar.xz

# Add toolchain to PATH
ENV PATH="/opt/toolchain/gcc-arm-8.3-2019.03-x86_64-aarch64-linux-gnu/bin:${PATH}"

# Set environment variables for cross-compilation
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
    CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
    CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++

# Create a non-root user for better security
RUN useradd -m -u 1000 rust
USER rust
WORKDIR /home/rust/src

CMD ["/bin/bash"]
