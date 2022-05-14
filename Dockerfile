FROM rust:slim-buster AS base
LABEL maintainer="Semen Syrovatskiy <mail@syrovatskiy.tk>"
ENV APP_HOME="/poem-example-app" \
    APP_PATH="/poem-example-app/target/release/poem-example-app" \
    APP_DB_FILE="/poem-example-app.db"

FROM base AS deps
RUN apt-get update && apt-get install -y libssl-dev pkg-config sqlite3
RUN cargo install sqlx-cli --no-default-features --features native-tls,sqlite

FROM deps AS build
WORKDIR ${APP_HOME}
COPY migrations migrations
COPY src src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN cargo build --release

FROM build AS runtime
RUN touch ${APP_DB_FILE}
CMD ${APP_PATH}
