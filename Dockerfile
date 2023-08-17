FROM rust:1.71

WORKDIR ../app

RUN apt-get update
RUN rustup component add rustfmt
RUN rustup component add clippy
RUN cargo install cargo-watch
RUN cargo install sqlx-cli

CMD ["tail", "-f", "/dev/null"]