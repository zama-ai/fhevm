# cli-relayer-sdk

CLI experiments for the new `@fhevm/sdk` viem adapter against Zama Protocol.

## Install

```bash
pnpm install
```

## Commands

The `cli` script uses Node's native `--env-file=.env`, so create `.env` from `.env.example` before running commands that rely on environment variables.

Global options must be passed before the command:

- `--network testnet`: only supported network for now.
- `--relayer-url <url>`: relayer base URL override. `localhost:3000` is normalized to `http://localhost:3000`; `/v1` and `/v2` suffixes are stripped because the SDK calls `/v2/*` routes internally.
- `--rpc-url <url>`: Sepolia RPC URL override. Defaults to `SEPOLIA_RPC_URL` or `https://ethereum-sepolia-rpc.publicnode.com`.

Input proof request, equivalent to the old frontend flow through `requestZKPVerif`:

```bash
pnpm --silent run cli --network testnet input-proof
```

Public decrypt supports `bool`, `uint8`, `uint128`, `address`, and `mixed`. Cached public decrypt has built-in alpha-format testnet handles for `bool` and `uint8`; pass repeated `--handle` values for other cached types:

```bash
pnpm --silent run cli --network testnet public-decrypt cached --type bool
pnpm --silent run cli --network testnet --relayer-url localhost:3000 public-decrypt cached --handle 0x...
```

Fresh public decrypt request. This encrypts a value, calls `makePubliclyDecryptableExternal*`, waits for the tx, then public decrypts the returned handle:

```bash
PRIVATE_KEY=0x... pnpm --silent run cli --network testnet public-decrypt fresh --type uint8
```

You can also pass `--private-key`, `--mnemonic`, and `--contract`. The default contract is the testnet `RelayerSDKTest` contract from the old frontend flow.

## Development

```bash
pnpm run typecheck
```
