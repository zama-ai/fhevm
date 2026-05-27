# cli-relayer-sdk

CLI experiments for the new `@fhevm/sdk` viem adapter against the FHETest v2 contract.

## Install

```bash
pnpm install
```

## Commands

The `cli` script uses Node's native `--env-file=.env`, so create `.env` from `.env.example` before running commands that rely on environment variables.

Global options must be passed before the command:

- `--network testnet`: uses FHETest `0x94B9d3aF050687D1F76251aD7D09a1F216a19845` on Ethereum Sepolia.
- `--network devnet`: uses FHETest `0xD26bB032e2F06A5382902559c4EbBB82C35C6dDF` on Ethereum Sepolia with the dev relayer config.
- `--relayer-url <url>`: relayer base URL override. `localhost:3000` is normalized to `http://localhost:3000`; `/v1` and `/v2` suffixes are stripped because the SDK calls `/v2/*` routes internally.
- `--rpc-url <url>`: host chain RPC URL override. Defaults to `SEPOLIA_RPC_URL` for testnet, `DEVNET_RPC_URL` for devnet, then the public Sepolia RPC fallback.

Progress is written to stderr. The final JSON payload is written to stdout, so commands remain pipeable.

Supported FHETest value types are `bool`, `uint8`, `uint16`, `uint32`, `uint64`, `uint128`, `uint256`, and `address`.

Input proof request:

```bash
pnpm --silent run cli --network testnet input-proof
pnpm --silent run cli --network testnet input-proof --type uint64 --value 42
```

Cached public decrypt from an FHETest account/type handle:

```bash
PRIVATE_KEY=0x... pnpm --silent run cli --network testnet public-decrypt cached --type bool
pnpm --silent run cli --network testnet public-decrypt cached --type uint128 --account 0x...
```

Cached public decrypt from direct handles:

```bash
pnpm --silent run cli --network testnet public-decrypt cached --handle 0x...
```

Fresh public decrypt. This encrypts a value, calls the matching `FHETest.setE*` function with `makePublic=true`, waits for the transaction, reads the stored handle, then public decrypts it:

```bash
PRIVATE_KEY=0x... pnpm --silent run cli --network testnet public-decrypt fresh --type uint8
PRIVATE_KEY=0x... pnpm --silent run cli --network testnet public-decrypt fresh --type address --value 0x37AC010c1c566696326813b840319B58Bb5840E4
```

Make an existing caller-owned FHETest handle public, then decrypt it:

```bash
PRIVATE_KEY=0x... pnpm --silent run cli --network testnet public-decrypt make-public --type uint64
```

Initialize FHETest handles for the wallet, using `setClearE*` and `makePublic=true`:

```bash
PRIVATE_KEY=0x... pnpm --silent run cli --network testnet fhe-test init
PRIVATE_KEY=0x... pnpm --silent run cli --network testnet fhe-test init --type uint256 --force
```

You can pass `--private-key`, `--mnemonic`, and `--contract` to transaction commands. Environment fallbacks are `PRIVATE_KEY` and `MNEMONIC`.

## Development

```bash
pnpm run typecheck
```
