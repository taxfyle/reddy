FROM --platform=$BUILDPLATFORM rust:1.78 as builder

# The build platform we are compiling on.
# Populated by BuildX
ARG BUILDPLATFORM

# The target platform we are compiling for.
# Populated by BuildX
ARG TARGETPLATFORM

# Create a new empty project.
RUN cargo new --bin reddy
WORKDIR /reddy

# Copy manifests.
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN <<EOF
  set -e;

  # Build to install deps.
  cargo build --release

  # Remove built empty files.
  rm ./target/release/deps/reddy*

  # Remove the shell project's code files.
  rm src/*.rs
EOF

# Copy source
COPY ./src ./src

# Build for release
RUN cargo build --release

FROM --platform=$TARGETPLATFORM ubuntu:latest
COPY --from=builder /reddy/target/release/reddy  /usr/local/bin/reddy
