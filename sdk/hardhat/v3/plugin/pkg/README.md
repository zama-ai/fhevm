# @fhevm/hardhat-plugin-v3

FHEVM tooling for [Hardhat 3](https://hardhat.org). On every development connection the plugin deploys
and verifies the cleartext FHEVM stack, then hands you `connection.fhevm`: encryption, public and user
decryption, error diagnostics, HCU accounting and a test-only debugger — the same surface the Hardhat 2
plugin offered, rebuilt on Hardhat 3 connections and [viem](https://viem.sh).

Hardhat 2 projects keep using `@fhevm/hardhat-plugin`; this package is the Hardhat 3 plugin, versioned
on the FHEVM protocol line it targets (`0.13.x`).

## Install

```sh
npm install --save-dev @fhevm/hardhat-plugin-v3 hardhat viem @fhevm/sdk @fhevm/solidity
```

`hardhat`, `viem` and `@fhevm/sdk` are peer dependencies. Node 22 or later.

## Configure

```ts
// hardhat.config.ts
import fhevmPlugin from '@fhevm/hardhat-plugin-v3';
import { defineConfig } from 'hardhat/config';

export default defineConfig({
  plugins: [fhevmPlugin],
  networks: {
    default: { type: 'edr-simulated', chainId: 31337 },
    localhost: { type: 'http', chainId: 31337, url: 'http://127.0.0.1:8545' },
  },
});
```

Your contracts inherit `ZamaEthereumConfig` from `@fhevm/solidity/config/ZamaConfig.sol` as usual; the
local stack lands on the addresses that file compiles in.

## Use

Hardhat 3 scopes networks to connections, so there is no `hre.fhevm`: the object lives on the connection.

```ts
import { FhevmType } from '@fhevm/hardhat-plugin-v3';
import { network } from 'hardhat';

const { ethers, fhevm } = await network.connect();

// Encrypt an input for (contract, user); several values share one proof.
const input = await fhevm.createEncryptedInput(counterAddress, alice.address).add32(5).encrypt();
await counter.increment(input.handles[0], input.inputProof);

// Public decryption (the handle must be `FHE.makePubliclyDecryptable`).
const count = await fhevm.publicDecryptEuint(FhevmType.euint32, await counter.getCount());

// User decryption: the user is a viem local account or a wallet client that carries its account.
const clear = await fhevm.userDecryptEuint(FhevmType.euint32, handle, counterAddress, aliceAccount);
// Delegated: decrypt on behalf of a contract that granted `FHE.delegateUserDecryption`.
await fhevm.userDecryptEuint(FhevmType.euint32, handle, counterAddress, bobAccount, { delegatorAddress: wallet });
```

Other members of `connection.fhevm`:

| Member                                                        | What it does                                                         |
| ------------------------------------------------------------- | -------------------------------------------------------------------- |
| `isCleartext`, `isDevelopment`, `network`                     | What kind of network the connection reached                          |
| `client`                                                      | The underlying `@fhevm/sdk` client                                   |
| `encryptUint / encryptBool / encryptAddress`                  | One value, one proof                                                 |
| `publicDecrypt(handles)`                                      | Handle-keyed clear values plus the proof `FHE.checkSignatures` takes |
| `revertedWithCustomErrorArgs(contract, error)`                | Chai: `expect(tx).to.be.revertedWithCustomError(...args)`            |
| `tryParseFhevmError(e)`                                       | A structured InputVerifier `InvalidSigner` diagnosis, else undefined |
| `parseCoprocessorEvents(logs)`, `computeTransactionHCU(rcpt)` | The executor's operator events and the HCU a transaction consumed    |
| `getCoprocessorConfig(c)`, `assertCoprocessorInitialized(c)`  | The addresses a contract compiled in, checked against the stack      |
| `typeof(handle)`                                              | The FHE type name a handle encodes                                   |
| `debugger.decryptEuint / Ebool / Eaddress`                    | Reads a cleartext with NO permission check — tests only              |

Reverts inside the FHEVM contracts surface with their meaning (for instance an `InputVerifier`
`InvalidSigner` explains that the input was encrypted for another contract or user), and
`eth_estimateGas` answers are inflated by 20% for FHE calls.

Module exports: `FhevmType` (a runtime enum), `getHCU(event, type)`, `timestampNow()`, and every type of
the surface.

## Tasks

```sh
npx hardhat fhevm public-decrypt <type> <handle>
npx hardhat fhevm user-decrypt <type> <handle> <contract> [--user <account index>]
npx hardhat fhevm check-fhevm-compatibility <address>
```

`--network` selects the connection as for any Hardhat task.

## `hardhat node`

The node deploys the stack before it listens and says so:

```
fhevm: cleartext FHEVM stack on 'node' (chainId 31337) — deployed by this node from @fhevm/host-contracts-cleartext, verified (-vvv lists the addresses)
```

`npx hardhat -vvv node` lists the ten contract addresses; Hardhat's own call traces start at the same
level. A second project connecting over `localhost` finds the stack and deploys nothing.

## Networks

- `edr-simulated` (in-process), `hardhat node` and anvil at chain id 31337: the cleartext stack is
  deployed or reused, and everything above works.
- Public FHEVM networks: detected, not yet served by the client — they follow in a later release.
