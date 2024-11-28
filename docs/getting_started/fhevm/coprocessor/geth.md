# Integration

This document is a guide listing detailed steps to integrate `fhevm-backend` into [go-ethereum](https://github.com/ethereum/go-ethereum) or any other implementations that follow the same architecture. We use `geth` and `go-ethereum` interchangeably from now on.

{% hint style="info" %}
This document is based on go-ethereum v1.14.3
{% endhint %}

The Go library that we integrate into geth is [fhevm-go-coproc](../../../../fhevm-engine/fhevm-go-coproc/README.md).

<!-- markdown-link-check-disable-next-line -->
We also have changes to geth itself here [go-ethereum](https://github.com/zama-ai/go-ethereum-coprocessor).

<!-- markdown-link-check-disable-next-line -->
To start a local geth coprocessor full node, please have a look at: https://github.com/zama-ai/go-ethereum-coprocessor/blob/master/Dockerfile.devnode

To execute the actual FHE computation, a [Coprocessor Backend](coprocessor_backend.md) is needed.

This document is still work in progress. Above repositories serve as reference for now.
