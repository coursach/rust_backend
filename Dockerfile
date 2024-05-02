

FROM rustlang/rust:nightly AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN apt-get install -y build-essential
RUN yes | apt install gcc-x86-64-linux-gnu

WORKDIR /app

COPY . /app
ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
RUN cargo build --target x86_64-unknown-linux-musl --release

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
# Replace if you are making a real service

EXPOSE 8000
WORKDIR /app

CMD cargo run