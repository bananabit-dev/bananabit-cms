FROM rust:1 AS chef
RUN cargo install cargo-chef
WORKDIR /app

# For Debian/Ubuntu-based images
RUN apt-get update && apt-get install -y \
    clang \
    && rm -rf /var/lib/apt/lists/*


FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

RUN rustup target add wasm32-unknown-unknown

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

# Install `dx`
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

RUN dx bundle --platform web --package web --release 

FROM chef AS runtime

COPY --from=builder /app/target/dx/web/release/web/ /usr/local/app

# Copy all assets from web/assets to /usr/local/app/assets
COPY --from=builder /app/web/assets/ /usr/local/app/assets/

# Copy additional assets from ui/assets (styling, images, etc.)
COPY --from=builder /app/ui/assets/styling/ /usr/local/app/assets/styling/
COPY --from=builder /app/ui/assets/images/ /usr/local/app/assets/images/

# Create data directory for database and uploads
RUN mkdir -p /usr/local/app/data

ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080

WORKDIR /usr/local/app
ENTRYPOINT [ "/usr/local/app/server" ]
