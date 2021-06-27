# Cosmos Cash / Credentials

[comment]: <> ([![Build]&#40;https://github.com/noandrea/cosmos-cash-credentialss/actions/workflows/rust.yml/badge.svg&#41;]&#40;https://github.com/noandrea/cosmos-cash-credentials/actions/workflows/rust.yml&#41;)

[![Coverage Status](https://coveralls.io/repos/github/noandrea/cosmos-cash-credentials/badge.svg?branch=master)](https://coveralls.io/github/noandrea/cosmos-cash-credentials?branch=master)
[![dependency status](https://deps.rs/repo/github/noandrea/cosmos-cash-credentials/status.svg)](https://deps.rs/repo/github/noandrea/cosmos-cash-credentials)
[![Crates.io](https://img.shields.io/crates/v/cosmos-cash-credentials)](https://crates.io/crates/cosmos-cash-credentials)

This is the Cosmos Cash credentials validator using HMAC and Merkle trees.

Ported from [go-merkletree](https://github.com/wealdtech/go-merkletree) with some changes.

:warning: Experimental :warning:


## Summary

The library provides a Merkle tree implementation using HMAC (with Sha256) as hashing algorithm.
The goal of the library is to provide a simple solution for selective disclosure 

## What's in the box 

The library can be built as a rust lib or as js package (via wasm)

## Wasm

run `make wasm-build` to build the wasm js package

the output of the 

use the `webui/index.html` for demo

TODO: add details


# Access gRPC from browser

gRPC web requires a proxy that handles cors:

```
git clone https://github.com/improbable-eng/grpc-web.git
cd grpc-web
go install go install ./go/grpcwebproxy
grpcwebproxy --backend_addr=localhost:9090 --run_tls_server=false --allow_all_origins
```

for more information check [improbable-eng/grpc-web](https://github.com/improbable-eng/grpc-web)
