# syntax=docker/dockerfile:1.4

FROM rust:1.86.0-slim as builder
LABEL maintainer="M10 Networks, Inc."

RUN rustup component add rustfmt

RUN rm -f /etc/apt/apt.conf.d/docker-clean && \
    echo 'Binary::apt::APT::Keep-Downloaded-Packages "true";' > /etc/apt/apt.conf.d/keep-cache

RUN --mount=type=cache,id=dvca,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,id=dvla,target=/var/lib/apt,sharing=locked \
    apt update && apt install -y --no-install-recommends \
     build-essential lld libssl-dev pkg-config libclang-dev curl unzip && \
    rm -rf /var/lib/apt/lists/*

# Install latest protoc
RUN curl -L https://github.com/protocolbuffers/protobuf/releases/download/v27.3/protoc-27.3-linux-x86_64.zip \
    -o /tmp/protoc.zip && \
    unzip /tmp/protoc.zip -d /usr/local && \
    rm /tmp/protoc.zip && \
    protoc --version && \
    which protoc

ARG ENV=release
WORKDIR /src/services/m10-bank-emulator

# Copy source code
COPY ./services/m10-bank-emulator/ ./
COPY ./shared/rust/ /src/shared/rust
COPY ./shared/protobuf/ /src/shared/protobuf

# Build the project
RUN --mount=type=bind,source=./services/m10-bank-emulator,rw \
    --mount=type=bind,source=./shared/rust,target=/src/shared/rust \
    --mount=type=bind,source=./shared/protobuf,target=/src/shared/protobuf \
    --mount=type=cache,id=ulcg,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,id=ulcr,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,id=dfx-bt,target=/build/target,sharing=locked \
    if [ "$ENV" = "release" ]; then rf="--release"; else rf=""; fi && \
    RUSTFLAGS="-C link-arg=-fuse-ld=lld" \
    CARGO_TARGET_DIR="/build/target" \
    cargo build $rf --bins && \
    cp /build/target/"$ENV"/m10-bank-emulator /bin/

FROM amazonlinux:2023 as m10-bank-emulator
LABEL maintainer="M10 Networks, Inc."

# Install runtime dependencies
RUN dnf -y update && \
    dnf -y install ca-certificates openssl shadow-utils.x86_64 && \
    dnf clean all

# Create a non-root user
RUN groupadd -r appgroup && useradd -r -g appgroup appuser

# Copy the built binary
COPY --from=builder /bin/m10-bank-emulator /bin/m10-bank-emulator

# Change ownership of the application files
RUN chown appuser:appgroup /bin/m10-bank-emulator

# Switch to the non-root user
USER appuser

ENTRYPOINT ["/bin/m10-bank-emulator"]
