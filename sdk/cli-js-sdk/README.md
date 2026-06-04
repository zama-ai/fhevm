# cli-fhevm-sdk

CLI for `@fhevm/sdk` viem flows against FHETest.

## Quick Start

```bash
pnpm install
cp .env.example .env
pnpm --silent run cli --help
```

The `cli` script uses Node's native `--env-file=.env`, so create `.env` from `.env.example` before running commands that rely on environment variables.

## Environment

| Variable                | Purpose                                                                             |
| ----------------------- | ----------------------------------------------------------------------------------- |
| `SEPOLIA_RPC_URL`       | Ethereum Sepolia host chain RPC override for testnet and devnet.                    |
| `POLYGON_AMOY_RPC_URL`  | Devnet Polygon Amoy host chain RPC override.                                        |
| `ZAMA_FHEVM_API_KEY`    | Optional SDK relayer auth, required where the target environment enforces API keys. |
| `PRIVATE_KEY`           | Default wallet private key for transaction/decryption commands.                     |
| `MNEMONIC`              | Default wallet mnemonic when `PRIVATE_KEY` is not set.                              |
| `DELEGATOR_PRIVATE_KEY` | Delegator private key for delegated user decrypt flows.                             |
| `DELEGATOR_MNEMONIC`    | Delegator mnemonic when `DELEGATOR_PRIVATE_KEY` is not set.                         |

Global options must be passed before the subcommand:

- `--network testnet`: uses FHETest `0x94B9d3aF050687D1F76251aD7D09a1F216a19845` on Ethereum Sepolia.
- `--network devnet`: uses FHETest `0xf56a7990E63a63eC75aD9Aa07De8cB6bF7baa805` on Ethereum Sepolia with the dev relayer config.
- `--network devnet-amoy`: uses FHETest `0x7553CB9124f974Ee475E5cE45482F90d5B6076BC` on Polygon Amoy with the dev relayer config.
- `--relayer-url <url>`: relayer base URL override. `localhost:3000` is normalized to `http://localhost:3000`.
- `--rpc-url <url>`: host chain RPC URL override. Defaults to `SEPOLIA_RPC_URL` for Sepolia-backed networks, `POLYGON_AMOY_RPC_URL` for devnet-amoy, then the network public RPC fallback.

Progress is written to stderr. The final machine-readable payload is written to stdout as JSON, so commands remain pipeable.

Supported FHETest value types are `bool`, `uint8`, `uint16`, `uint32`, `uint64`, `uint128`, `uint256`, and `address`.

## Command Model

Decrypt workflows have two modes:

- `fresh`: encrypts/stores a new FHETest handle first, then decrypts it.
- `cached`: decrypts an existing FHETest handle from account/type, or decrypts direct `--handle` values.

Public decrypt `fresh` stores with `makePublic=true`. User decrypt and delegated user decrypt `fresh` store with `makePublic=false`.

`fhe-test inspect` is read-only and has two mutually exclusive modes:

- `--handle <handle>`: inspect a raw handle's embedded FHE type and whether FHETest can read its recorded cleartext.
- `--type <type>` with optional `--account <address>`: inspect the FHETest account/type slot. When `--account` is omitted, the wallet address from `--private-key`, `PRIVATE_KEY`, `--mnemonic`, or `MNEMONIC` is used.

`fhe-test init --bulk` initializes all FHETest value types with one `initFheTest` transaction. It is incompatible with `--type`.

`fhe-test op` runs FHETest's on-chain operation demos against the caller's stored handle for the operation type, so initialize or store that type first. Supported operations are `xor-bool`, `add-uint8`, `add-uint16`, `add-uint32`, `add-uint64`, `add-uint128`, `xor-uint256`, and `eq-address`.

## Examples

Input proof:

```bash
pnpm --silent run cli --network testnet input-proof
pnpm --silent run cli --network testnet input-proof --type uint32
pnpm --silent run cli --network testnet input-proof --type uint64 --value 42
```

Public decrypt:

```bash
pnpm --silent run cli --network testnet public-decrypt fresh --type uint8
pnpm --silent run cli --network testnet public-decrypt cached --type uint8
pnpm --silent run cli --network testnet public-decrypt cached --handle 0x...
pnpm --silent run cli --network testnet public-decrypt make-public --type uint64
```

User decrypt:

```bash
pnpm --silent run cli --network testnet user-decrypt fresh --type uint8
pnpm --silent run cli --network testnet user-decrypt cached --type uint8
pnpm --silent run cli --network testnet user-decrypt cached --handle 0x...
```

Delegated user decrypt:

```bash
pnpm --silent run cli --network testnet delegated-user-decrypt fresh --type uint8
pnpm --silent run cli --network testnet delegated-user-decrypt cached --delegator 0x... --type uint8
pnpm --silent run cli --network testnet delegated-user-decrypt cached --delegator 0x... --handle 0x...
```

FHETest utilities:

```bash
pnpm --silent run cli --network testnet fhe-test info
pnpm --silent run cli --network testnet fhe-test inspect --type uint64
pnpm --silent run cli --network testnet fhe-test inspect --account 0x... --type uint64
pnpm --silent run cli --network testnet fhe-test inspect --handle 0x...
pnpm --silent run cli --network testnet fhe-test init
pnpm --silent run cli --network testnet fhe-test init --bulk
pnpm --silent run cli --network testnet fhe-test init --type uint256 --force
pnpm --silent run cli --network testnet fhe-test op add-uint64 --value 42
pnpm --silent run cli --network testnet fhe-test op xor-bool --value true --public
```

## Development

```bash
pnpm run typecheck
```
