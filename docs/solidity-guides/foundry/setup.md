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

If you already have a Foundry project, add `forge-fhevm` as a [Soldeer](https://soldeer.xyz) dependency. The shape of the configuration is shown below; for the exact pinned versions, copy `foundry.toml` and `remappings.txt` from the [Foundry template](https://github.com/zama-ai/fhevm-foundry-template) — that's where the canonical, tested versions live.

#### 1. Configure `foundry.toml`

`forge-fhevm` targets the Cancun EVM and a recent Solidity compiler. Your `foundry.toml` should look roughly like:

```toml
[profile.default]
src = "src"
out = "out"
libs = ["dependencies"]
test = "test"
script = "script"
evm_version = "cancun"
# solc = "0.8.x"   # see the template for the version currently tested

[dependencies]
# See the template's foundry.toml for the current versions
forge-std = "..."
"@encrypted-types" = "..."
"@fhevm-solidity" = "..."
forge-fhevm = { git = "https://github.com/zama-ai/forge-fhevm.git", rev = "..." }

[soldeer]
remappings_version = false
recursive_deps = true
```

#### 2. Install dependencies

```bash
forge soldeer install
```

#### 3. Add remappings

Soldeer materializes each dependency under `dependencies/<name>-<version>/`, so your `remappings.txt` needs an entry per import prefix. The shape is:

```
@fhevm/host-contracts/=dependencies/forge-fhevm-<rev>/src/fhevm-host/
@fhevm/solidity/=dependencies/@fhevm-solidity-<version>/
encrypted-types/=dependencies/@encrypted-types-<version>/
forge-fhevm/=dependencies/forge-fhevm-<rev>/src/
forge-std/=dependencies/forge-std-<version>/src
```

Replace the `<version>` / `<rev>` placeholders with whatever Soldeer wrote into `dependencies/`, or copy the whole file from the [template `remappings.txt`](https://github.com/zama-ai/fhevm-foundry-template/blob/main/remappings.txt) and adjust as you upgrade.

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
