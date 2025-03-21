# Use Debian-based Rust image
FROM rust:latest

# Set up a non-root user for Codespaces
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    python3 \
    python3-pip \
    python3-venv \
    qemu-system \
    gdb-multiarch \
    git \
    clang \
    llvm \
    lld \
    cmake \
    ninja-build \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust components
RUN rustup update && rustup component add rustfmt clippy llvm-tools-preview

# Install useful Rust crates globally
RUN cargo install cargo-watch cargo-audit

# Create the non-root user
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && chown -R $USERNAME:$USERNAME /home/$USERNAME

USER $USERNAME

# Set up virtual environment
RUN python3 -m venv /home/$USERNAME/.venv \
    && /home/$USERNAME/.venv/bin/pip install --upgrade pip \
    && /home/$USERNAME/.venv/bin/pip install meson pyelftools numpy

# Make venv activate by default in all terminals
RUN echo 'source ~/.venv/bin/activate' >> /home/$USERNAME/.bashrc

WORKDIR /workspaces
