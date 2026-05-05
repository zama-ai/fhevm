This page walks through setting up a Foundry project for FHEVM smart contract development.

## Prerequisites

- [Foundry](https://book.getfoundry.sh/getting-started/installation) (latest)

## Option 1: Clone the FHEVM Foundry template (recommended)

The [FHEVM Foundry Template](https://github.com/zama-ai/fhevm-foundry-template) ships a working setup — `foundry.toml`, `remappings.txt`, an example `FHECounter` contract, tests, and deploy scripts.

{% stepper %}

{% step %}

#### Clone the template

```bash
git clone https://github.com/zama-ai/fhevm-foundry-template
cd fhevm-foundry-template
```

{% endstep %}

{% step %}

#### Install dependencies with Soldeer

The template uses [Soldeer](https://soldeer.xyz) for dependency management. Run:

```bash
forge soldeer install
```

This installs `forge-fhevm`, `@fhevm/solidity`, `encrypted-types`, OpenZeppelin contracts, and `forge-std` into the `dependencies/` directory.

{% endstep %}

{% step %}

#### Compile and run the tests

```bash
forge build
forge test -vvv
```

You should see the example `FHECounter` tests pass.

{% endstep %}

{% endstepper %}

## Option 2: Add forge-fhevm to an existing project

If you already have a Foundry project, add `forge-fhevm` as a Soldeer dependency.

#### 1. Configure `foundry.toml`

`forge-fhevm` requires Solidity `^0.8.27` and the Cancun EVM. Add the dependency and set the compiler to `foundry.toml`:

```toml
[profile.default]
src = "src"
out = "out"
libs = ["dependencies"]
test = "test"
script = "script"
solc = "0.8.27"
evm_version = "cancun"

[dependencies]
forge-std = "1.14.0"
"@encrypted-types" = "0.0.4"
"@fhevm-solidity" = "0.11.1"
forge-fhevm = { version = "eba2324", git = "https://github.com/zama-ai/forge-fhevm.git", rev = "eba2324" }

[soldeer]
remappings_version = false
recursive_deps = true
```

#### 2. Install dependencies

```bash
forge soldeer install
```

#### 3. Add remappings

Update (or create) `remappings.txt`:

```
@fhevm/host-contracts/=dependencies/forge-fhevm-eba2324/src/fhevm-host/
@fhevm/solidity/=dependencies/@fhevm-solidity-0.11.1/
encrypted-types/=dependencies/@encrypted-types-0.0.4/
forge-fhevm/=dependencies/forge-fhevm-eba2324/src/
forge-std/=dependencies/forge-std-1.14.0/src
```

{% hint style="warning" %}
Soldeer pins each dependency directory by version (e.g. `forge-fhevm-eba2324`). When you upgrade, regenerate or update the matching remapping paths.
{% endhint %}

## Verify the install

Create a minimal test file:

```solidity
// test/Setup.t.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {FhevmTest} from "forge-fhevm/FhevmTest.sol";

contract SetupTest is FhevmTest {
    function test_setupDeploys() public view {
        // setUp() deploys all FHEVM host contracts at deterministic addresses
        assertTrue(address(_executor) != address(0));
        assertTrue(address(_acl) != address(0));
    }
}
```

```bash
forge test --match-test test_setupDeploys -vv
```

## Where to go next

🟨 Go to [**Write FHEVM tests in Foundry**](write_test.md) to start writing tests with `forge-fhevm`.

🟨 Go to [**Deploy FHEVM contracts with Foundry**](deploy.md) for the deployment workflow.
