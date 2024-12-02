###################### Dev ############################
FROM rust:latest AS dev

RUN apt update -y && \
  apt install -y postgresql-client lcov redis-tools && \
  rustup component add rustfmt clippy llvm-tools-preview && \
  cargo install cargo-watch sqlx-cli --locked

# Set the shell to /bin/bash to avoid repetition
ENV SHELL=bash
SHELL ["/bin/bash", "-c"]

# Install Node, pnpm & nx
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash && \
    source $HOME/.nvm/nvm.sh && \
    nvm install 20 && \
    nvm alias default 20 && \
    nvm use default && \
    corepack enable pnpm && \
    corepack use pnpm@latest && \
    pnpm setup && \
    source $HOME/.bashrc && \
    pnpm install -g nx

# Install grcov
RUN curl -sL https://github.com/mozilla/grcov/releases/download/v0.8.19/grcov-aarch64-unknown-linux-gnu.tar.bz2 | tar jxf - -C /usr/local/bin/

WORKDIR /app

###################### Build ############################
FROM rust:latest AS build

ARG APP_NAME

WORKDIR /app

COPY . .

# Build the binary in release mode
RUN cargo build --release --bin ${APP_NAME}

###################### Production ############################
FROM debian:12.6 AS prod

ARG APP_NAME
ENV APP_NAME=${APP_NAME}

RUN apt-get update && \
    apt-get install -y \
        libssl-dev \
        build-essential \
        curl \
        openssl \
  && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/${APP_NAME} /usr/local/bin/app

CMD ["/usr/local/bin/app"]
