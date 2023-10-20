FROM rustlang/rust:nightly-alpine AS rust_builder

WORKDIR /usr/src/app

RUN apk update
RUN apk add --no-cache musl-dev

ENV USER=appuser
ENV UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

COPY Cargo.toml ./
COPY Cargo.lock ./
COPY src ./src

ENV RUSTFLAGS='-C target-feature=-crt-static'
RUN cargo build --release

FROM alpine

WORKDIR /opt/app

RUN apk update
RUN apk add --no-cache libgcc

COPY --from=rust_builder /usr/src/app/target/release/rust_web_benchmark ./

COPY --from=rust_builder /etc/passwd /etc/passwd
COPY --from=rust_builder /etc/group /etc/group

USER appuser:appuser

EXPOSE 3000

CMD ["/opt/app/rust_web_benchmark"]
