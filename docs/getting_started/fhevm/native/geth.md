# Integration

This document is a guide listing detailed steps to integrate `fhevm-backend` into [go-ethereum](https://github.com/ethereum/go-ethereum) or any other implementations that follow the same architecture.

{% hint style="info" %}
This document is based on go-ethereum v1.13.5
{% endhint %}

An fhEVM-native node consists of the following components:
 * full node/validator node
 * Executor service

At the time of writing, the geth full node/validator node is not yet implemented. The [Executor](executor.md) is almost fully functional.

