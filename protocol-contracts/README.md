## Introduction

**Protocol Contracts** are the on-chain applications that govern and fund the FHEVM protocol, as
opposed to the contracts that execute it. They own the protocol's token, its staking and rewards,
its DAO-driven governance across chains, and the Safe wiring that executes governance decisions on
each host chain.

These packages are moving here from `zama-ai/protocol-apps`, tracked in
[`planning-blockchain#1314`](https://github.com/zama-ai/planning-blockchain/issues/1314). This
directory is created ahead of them so that ownership and review routing are in place before the
first package lands.

## Packages

| Package | Description | Status |
| --- | --- | --- |
| `staking` | Operator and protocol staking vaults, and the rewarder that distributes to them | moved |
| `governance` | The cross-chain governance path — an Aragon DAO on Ethereum driving LayerZero OApp senders and receivers | moved |
| `safe` | Per-chain Safe and the admin module that lets a governance message execute locally | moved |
| `token` | The `$ZAMA` token and its LayerZero OFT deployments | moved |
| `feesBurner` | Receives protocol fees and burns them | moved |
| `pauserSetWrapper` | Wraps the pauser set so governance can administer it | moved |
| `solanaOFT` | The `$ZAMA` OFT on Solana and the EVM side of its bridge — Anchor programs beside Hardhat and Foundry contracts | moved |

`solanaOFT` lives here rather than in a repository of its own — the decision is recorded on
[`planning-blockchain#1352`](https://github.com/zama-ai/planning-blockchain/issues/1352). Its CI runs
the EVM half only — lint and Foundry tests; the Anchor programs need a Solana toolchain no runner
here installs yet, which is the coverage they had in `protocol-apps` too.

`confidential-wrapper` and `confidential-token-wrappers-registry` are **not** moving; they stay with
Protocol Apps.

## Support

<a target="_blank" href="https://community.zama.ai">
  <img src="https://img.shields.io/badge/Community%20forum-ffb243?style=flat-square" alt="Community forum">
</a>
