FROM python:3.9

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN pip install transformers torch

WORKDIR /app

COPY ./src /app/src
COPY ./Cargo.toml /app/Cargo.toml
COPY ./Cargo.lock /app/Cargo.lock
COPY ./app.py /app/app.py

RUN chmod +x /app/app.py


RUN cargo build --release


ENV PATH="/app/target/release:${PATH}"

ENTRYPOINT ["/app/target/release/rust_ml_service"]

