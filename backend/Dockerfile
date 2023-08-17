##
# Adapted from this GitHub answer: https://github.com/emk/rust-musl-builder/issues/151#issuecomment-1515373012
##

# Chef
FROM messense/rust-musl-cross:x86_64-musl as chef

RUN cargo install cargo-chef
WORKDIR /panel-backend

# Planner
FROM chef as planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Builder
FROM chef as builder 

COPY --from=planner /panel-backend/recipe.json recipe.json

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

# Run :)
FROM scratch
COPY --from=builder /panel-backend/target/x86_64-unknown-linux-musl/release/panel-backend /usr/local/bin/panel-backend
EXPOSE 8080
ENTRYPOINT [ "panel-backend" ]

