FROM rust:1.53
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .
ENTRYPOINT ["fasta_multi_grep"]