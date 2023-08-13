FROM rust:1.71

WORKDIR ../app

RUN apt-get update

CMD ["tail", "-f", "/dev/null"]