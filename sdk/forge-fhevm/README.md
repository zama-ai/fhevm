# forge-fhevm

FHEVM testing library for Foundry projects, backed by `@fhevm/host-contracts-cleartext`.

Inherit `FhevmTest`, call `super.setUp()`, and the cleartext host stack is live at the addresses your
contract's `ZamaEthereumConfig` already points at:

```solidity
import {FhevmTest} from "forge-fhevm/FhevmTest.sol";

contract CounterTest is FhevmTest {
    function test_increment() public {
        (externalEuint32 v, bytes memory proof) = encryptUint32(42, alice, address(counter));
        vm.prank(alice);
        counter.increment(v, proof);
    }
}
```

## What this library is — and is not

This is a thin consumer of `@fhevm/host-contracts-cleartext` (see the package's `DESIGN.md`). The
split of responsibilities:

- **The package owns deployment.** `FhevmTest` inherits the package's `deploy/FhevmStack.sol`, which
  stands the whole stack up (proxies, initializer order, ownership). Nothing in this library deploys
  or places host-contract code.
- **This library supplies the three things the package cannot know:**
  - *Where* the stack lives — `src/config/addresses.sol`, substituted into the host contracts via the
    `fhevm-config-0.14.0/` remapping (three of those addresses are pinned by `ZamaConfig`).
  - *What* to initialize it with — the mock KMS / input-signer keys in `FhevmTest`.
  - The test-facing API — `encryptBool`/`encryptUintN`/`encryptAddress`, `userDecrypt`,
    `publicDecrypt`, `kmsDecryptionProof`, `dealConfidential`, plus the proof-building internals
    (`src/internal/InputProofLib.sol`, `src/internal/KmsProofLib.sol`) that play the off-chain
    gateway/KMS role.

The stack it deploys is the real thing — real ACL enforcement, real signature verification, real HCU
accounting. Only the cryptography is faked: values travel in the clear inside the input proof.

## Layout

```
src/FhevmTest.sol              the base test contract (inherit this)
src/config/addresses.sol       where the stack lives (feeds the fhevm-config remapping)
src/internal/InputProofLib.sol input-proof layout + EIP-712 input signing
src/internal/KmsProofLib.sol   KMS decryption-proof layout + EIP-712 decryption signing
```

Host contracts are compiled from the sibling package's sources via `remappings.txt` — nothing is
vendored.
