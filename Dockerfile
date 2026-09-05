# infra-job-killer — Multi-stage Docker Build
FROM rust:1.82-bookworm AS builder

RUN rustup target add wasm32-unknown-unknown && \
    cargo install cargo-leptos

WORKDIR /app
COPY Cargo.toml rust-toolchain.toml ./
COPY shared/ shared/
COPY frontend/ frontend/

RUN cargo leptos build --release

# Runtime with Terraform + AWS CLI
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl unzip && rm -rf /var/lib/apt/lists/*

ARG TERRAFORM_VERSION=1.9.2
RUN curl -fsSL https://releases.hashicorp.com/terraform/${TERRAFORM_VERSION}/terraform_${TERRAFORM_VERSION}_linux_amd64.zip -o terraform.zip && \
    unzip terraform.zip -d /usr/local/bin/ && rm terraform.zip

RUN curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip" && \
    unzip awscliv2.zip && ./aws/install && rm -rf awscliv2.zip aws

RUN useradd -m -s /bin/bash infra && \
    mkdir -p /app/data /app/deployments && chown -R infra:infra /app

USER infra
WORKDIR /app

COPY --from=builder --chown=infra:infra /app/target/release/platform-made-easy ./
COPY --from=builder --chown=infra:infra /app/target/site ./target/site
COPY --chown=infra:infra terraform/ ./terraform/

VOLUME ["/app/data", "/app/deployments"]
EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=3s CMD curl -f http://localhost:3000/ || exit 1

ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="target/site"

ENTRYPOINT ["./platform-made-easy"]
