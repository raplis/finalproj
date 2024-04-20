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

RUN apt-get update && apt-get install -y curl
RUN curl -LO "https://github.com/prometheus/prometheus/releases/download/v2.26.0/prometheus-2.26.0.linux-amd64.tar.gz" \
    && tar -xvf prometheus-*.tar.gz \
    && mv prometheus-2.26.0.linux-amd64/prometheus /usr/local/bin/


COPY ./prometheus.yml /etc/prometheus/prometheus.yml



ENV PATH="/app/target/release:${PATH}"

CMD ["/bin/sh", "-c", "/usr/local/bin/prometheus --config.file=/etc/prometheus/prometheus.yml & /app/target/release/rust_ml_service"]


# ENTRYPOINT ["/app/target/release/rust_ml_service"]

