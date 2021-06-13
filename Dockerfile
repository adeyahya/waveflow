FROM arm64v8/ubuntu:18.04

WORKDIR /app

RUN apt-get update
RUN apt-get install -y build-essential libsqlite3-dev curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y