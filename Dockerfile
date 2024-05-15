FROM rust:1.78 as builder

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

FROM ubuntu:latest
COPY --from=builder /reddy/target/release/reddy  /usr/local/bin/reddy

CMD ["reddy"]