#!/bin/sh
if [ -z "$AWS_LAMBDA_RUNTIME_API" ]; then
  exec /usr/local/bin/aws-lambda-rie /app/target/release/rust_ml_service
else
  exec /app/target/release/rust_ml_service
fi