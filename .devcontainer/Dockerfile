FROM mcr.microsoft.com/devcontainers/base:ubuntu24.04

# Configuration
## Environment
ARG USERNAME=vscode
## Rust
ARG RUSTUP_VERSION=1.29.0
ARG RUSTUP_INIT_SHA256=4acc9acc76d5079515b46346a485974457b5a79893cfb01112423c89aeb5aa10
ARG RUST_ARCH=x86_64-unknown-linux-gnu
ARG RUST_VERSION=1.97.1
## Node.js
ARG PNPM_VERSION=11.21.0
ARG NODE_VERSION=24.19.0
## gh
ARG GH_GPG_SHA512=ce6b9466dbd2a90b3227e177aa9b8187bd2405b1c29f91d78de83b9699dbbe2af35efd733bf53da622e7a38c59a7bc55539d63a3deae3c9ff9c2bff8af626434

# Root-level setup
ENV PATH="/usr/local/cargo/bin:$PATH"

## Install Rust
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ARG RUSTUP_INIT_URL="https://static.rust-lang.org/rustup/archive/${RUSTUP_VERSION}/${RUST_ARCH}/rustup-init"

RUN TMP_DIR="$(mktemp -d)" \
    && TMP_RUSTUP_INIT="${TMP_DIR}/rustup-init" \
    && curl -fsSL "${RUSTUP_INIT_URL}" -o "${TMP_RUSTUP_INIT}" \
    && echo "${RUSTUP_INIT_SHA256} *${TMP_RUSTUP_INIT}" | sha256sum -c - \
    && chmod +x "${TMP_RUSTUP_INIT}" \
    && "${TMP_RUSTUP_INIT}" -y --no-modify-path --profile minimal --default-toolchain "${RUST_VERSION}" --default-host "${RUST_ARCH}" \
    && rm -rf "${TMP_DIR}" \
    && chmod -R a+w "${RUSTUP_HOME}" "${CARGO_HOME}"

## Install gh
RUN (type -p wget >/dev/null || (apt-get update && apt-get install wget -y)) \
	&& mkdir -p -m 755 /etc/apt/keyrings \
	&& out=$(mktemp) && wget -nv -O$out https://cli.github.com/packages/githubcli-archive-keyring.gpg \
    && echo "${GH_GPG_SHA512} *${out}" | sha512sum -c - \
	&& cat $out | tee /etc/apt/keyrings/githubcli-archive-keyring.gpg > /dev/null \
	&& chmod go+r /etc/apt/keyrings/githubcli-archive-keyring.gpg \
	&& mkdir -p -m 755 /etc/apt/sources.list.d \
	&& echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | tee /etc/apt/sources.list.d/github-cli.list > /dev/null \
	&& apt update \
	&& apt install gh -y

## Install tools
RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      file \
      make \
    && rm -rf /var/lib/apt/lists/*

# User-level setup
USER "${USERNAME}"
ENV PNPM_HOME="/home/${USERNAME}/.local/share/pnpm"
ARG LOCAL_BIN_DIR="/home/${USERNAME}/.local/bin"
ENV PATH="${PNPM_HOME}/bin:${PATH}"
ENV PATH="${LOCAL_BIN_DIR}:$PATH"

## Ensure local bin directory exists
RUN mkdir -p "${LOCAL_BIN_DIR}"

## Install pnpm
ARG PNPM_INSTALL_URL="https://get.pnpm.io/install.sh"
RUN curl -fsSL "${PNPM_INSTALL_URL}" \
      | ENV="$HOME/.bashrc" SHELL="$(which bash)" PNPM_VERSION="${PNPM_VERSION}" bash - \
    && pnpm config set store-dir ~/.pnpm-store --global

## Install Node.js
RUN pnpm runtime set node "${NODE_VERSION}" --global

## Install Claude Code
ARG CLAUDE_INSTALL_URL="https://claude.ai/install.sh"
RUN curl -fsSL "${CLAUDE_INSTALL_URL}" | bash
