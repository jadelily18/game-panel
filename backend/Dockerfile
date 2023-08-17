FROM messense/rust-musl-cross:x86_64-musl as chef

RUN cargo install cargo-chef
WORKDIR /panel-backend


FROM chef as planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef as builder 

COPY --from=planner /panel-backend/recipe.json recipe.json

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl



FROM scratch
COPY --from=builder /panel-backend/target/x86_64-unknown-linux-musl/release/panel-backend /usr/local/bin/panel-backend
EXPOSE 8080
ENTRYPOINT [ "panel-backend" ]


# FROM alpine:latest

# ARG APP=/usr/src/APP

# EXPOSE 8080

# ENV TZ=Etc/UTC \
#     APP_USER=appuser

# RUN addgroup -S $APP_USER \
#     && adduser -S -g $APP_USER $APP_USER

# RUN apk update \
#     && apk add --no-cache ca-certificates tzdata \
#     && rm -rf /var/cache/apk/*

# COPY --from=builder /panel-backend/target/x86_64-unknown-linux-musl/release/panel-backend ${APP}/panel-backend

# RUN chown -R $APP_USER:$APP_USER ${APP}

# USER $APP_USER
# WORKDIR ${APP}

# CMD ls -l
