In this section, you'll find everything you need to test your FHEVM smart contracts in your [Hardhat](https://hardhat.org) project.

## FHEVM Runtime Modes

The FHEVM Hardhat plugin provides three **FHEVM runtime modes** tailored for different stages of contract development and testing. Each mode offers a trade-off between speed, encryption, and persistence.

1. The **Hardhat (In-Memory)** default network: 🧪 _Uses mock encryption._ Ideal for regular tests, CI test coverage, and fast feedback during early contract development. No real encryption is used.

2. The **Hardhat Node (Local Server)** network: 🧪 _Uses mock encryption._ Ideal when you need persistent state - for example, when testing frontend interactions, simulating user flows, or validating deployments in a realistic local environment. Still uses mock encryption.

3. The **Sepolia Testnet** network: 🔐 _Uses real encryption._ Use this mode once your contract logic is stable and validated locally. This is the only mode that runs on the full FHEVM stack with **real encrypted values**. It simulates real-world production conditions but is slower and requires Sepolia ETH.

### Summary

| Mode              | Encryption         | Persistent | Chain     | Speed          | Usage                                             |
| ----------------- | ------------------ | ---------- | --------- | -------------- | ------------------------------------------------- |
| Hardhat (default) | 🧪 Mock            | ❌ No      | In-Memory | ⚡⚡ Very Fast | Fast local testing and coverage                   |
| Hardhat Node      | 🧪 Mock            | ✅ Yes     | Server    | ⚡ Fast        | Frontend integration and local persistent testing |
| Sepolia Testnet   | 🔐 Real Encryption | ✅ Yes     | Server    | 🐢 Slow        | Full-stack validation with real encrypted data    |

## The FHEVM Hardhat Template

To demonstrate the three available testing modes, we'll use the [fhevm-hardhat-template](https://github.com/zama-ai/fhevm-hardhat-template), which comes with the FHEVM Hardhat Plugin pre-installed, a basic `FHECounter` smart contract, and ready-to-use tasks for interacting with a deployed instance of this contract.

## Run on Hardhat (default)

To run your tests in-memory using FHEVM mock values, simply run the following:

```sh
npx hardhat test --network hardhat
```

## Run on Hardhat Node

You can also run your tests against a local Hardhat node, allowing you to deploy contract instances and interact with them in a persistent environment.

{% stepper %}
{% step %}

#### Launch the Hardhat Node server:

- Open a new terminal window.
- From the root project directory, run the following:

```sh
npx hardhat node
```

{% endstep %}
{% step %}

#### Run your test suite (optional):

From the root project directory:

```sh
npx hardhat test --network localhost
```

{% endstep %}
{% step %}

#### Deploy the `FHECounter` smart contract on Hardhat Node

From the root project directory:

```sh
npx hardhat deploy --network localhost
```

Check the deployed contract FHEVM configuration:

```sh
npx hardhat fhevm check-fhevm-compatibility --network localhost --address <deployed contract address>
```

{% endstep %}
{% step %}

#### Interact with the deployed `FHECounter` smart contract

From the root project directory:

1. Decrypt the current counter value:

```sh
npx hardhat --network localhost task:decrypt-count
```

2. Increment the counter by 1:

```sh
npx hardhat --network localhost task:increment --value 1
```

3. Decrypt the new counter value:

```sh
npx hardhat --network localhost task:decrypt-count
```

{% endstep %}
{% endstepper %}

## Run on Sepolia Ethereum Testnet

To test your FHEVM smart contract using real encrypted values, you can run your tests on the Sepolia Testnet.

{% stepper %}
{% step %}

#### Rebuild the project for Sepolia

From the root project directory:

```sh
npx hardhat clean
npx hardhat compile --network sepolia
```

{% endstep %}
{% step %}

#### Deploy the `FHECounter` smart contract on Sepolia

```sh
npx hardhat deploy --network sepolia
```

{% endstep %}
{% step %}

#### Check the deployed `FHECounter` contract FHEVM configuration

From the root project directory:

```sh
npx hardhat fhevm check-fhevm-compatibility --network sepolia --address <deployed contract address>
```

If an internal exception is raised, it likely means the contract was not properly compiled for the Sepolia network.

{% endstep %}
{% step %}

#### Interact with the deployed `ConfidentialERC20` contract

From the root project directory:

1. Decrypt the current counter value (⏳ wait...):

```sh
npx hardhat --network sepolia task:decrypt-count
```

2. Increment the counter by 1 (⏳ wait...):

```sh
npx hardhat --network sepolia task:increment --value 1
```

3. Decrypt the new counter value (⏳ wait...):

```sh
npx hardhat --network sepolia task:decrypt-count
```

{% endstep %}
{% endstepper %}
