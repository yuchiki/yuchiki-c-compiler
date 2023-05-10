# syntax=docker/dockerfile:1.4
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY --link . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS dependencies

COPY --from=planner --link /app/recipe.json recipe.json

RUN echo recipe.json
RUN cat recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY --link . .
RUN cargo build --release

CMD ["cargo", "test", "--release"]
