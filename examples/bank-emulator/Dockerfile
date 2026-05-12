# syntax=docker/dockerfile:1.4

FROM public.ecr.aws/amazonlinux/amazonlinux:2023 AS builder
LABEL maintainer="M10 Networks, Inc."

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:/usr/local/rustup/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

RUN dnf -y update && \
    dnf -y install \
    gcc gcc-c++ make \
    clang clang-devel llvm llvm-devel lld \
    openssl-devel pkgconfig \
    cmake python3 curl-minimal unzip which \
    ca-certificates && \
    dnf clean all

# Install Rust toolchain
RUN curl -fsSL https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.93.0 && \
    rustup component add rustfmt

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
RUN --mount=type=bind,source=./services/m10-bank-emulator,target=/src/services/m10-bank-emulator,rw \
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

FROM public.ecr.aws/amazonlinux/amazonlinux:2023 AS m10-bank-emulator
LABEL maintainer="M10 Networks, Inc."

# Install runtime dependencies
RUN dnf -y update && \
    dnf -y install ca-certificates openssl shadow-utils && \
    dnf upgrade-minimal -y && \
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
