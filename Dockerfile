FROM rust:1.53
LABEL org.opencontainers.image.source=https://github.com/GreeningLab/FastaMultiGrep
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .