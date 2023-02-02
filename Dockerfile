FROM rust:latest

COPY . /app
RUN cd /app && cargo build
#todo manually remove ambiguity

WORKDIR /app
