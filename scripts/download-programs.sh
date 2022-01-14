#!/usr/bin/env sh

cd $(dirname $0)/..

mkdir -p artifacts/deploy/

curl -L https://github.com/DeployDAO/verified-program-artifacts/raw/verify-CrateProtocol__crate-v0.4.0/verifiable/crate_token.so \
    >artifacts/deploy/crate_token.so
