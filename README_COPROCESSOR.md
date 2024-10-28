# What does the setup do ?

It provides a way to run a co-processor, gateway and kms (in centralized mode)
using docker compose.

The pre-computed contract and account addresses/private keys are configured
across all the components for this test setup.

# How the repository is organized ?

- The docker compose file for KMS is defined
  [locally](./docker-compose/docker-compose-full.yml).

- Co-processor repository is cloned to work_dir and the Makefile is used for
  bringing up co-processor network.

- Solidity repository is cloned to work_dir and used from source to funding
  test accounts, deploy contracts and run tests.

- User is expected to run gateway from source.

## Prerequisites

- If there is an authentication error coming from docker.
  It maybe be that you're pulling a private image without a token.
  Go to github "Settings/Developer Settings" > "Personal Access Tokens" > "Tokens (classic)" > "Generate new token (classic)".
  The token should have the "read:packages" permission.
  Afterwards, do `docker login ghcr.io` and use your github ID and the token to login.
  Note that this token is saved by docker locally in the clear,
  so it's best to only give it the permissions you need and set the expiration time to 7 days.

### Install Binaries

Make sure you have these binaries installed in your environment:

* [docker](https://docs.docker.com/engine/install)
* [rust](https://www.rust-lang.org/tools/install)
* [protobuf](https://grpc.io/docs/protoc-installation)
* [nodejs](https://nodejs.org/en/download/package-manager)

> **NB:** We will wrap all components in docker images to remove these prerequisites in the near future.

## Steps to run the setup

0. Verify the configuration in .env file, most important variable is CENTRALIZED_KMS, set it to  false for threshold KMS

_Optionally_ you may update `KEY_GEN` value in `.env`. Default is `false`

| CENTRALIZED_KMS | Purpose |
| --- | --- |
| true    | KMS is running in centralized mode, keys are retrieved from the dev image (default) |
| false   | KMS is running in threshold mode with 4 MPC nodes, keys are freshly generated and signers are automatically updated |

1. Run the KMS in centralized mode (including the deployment of contracts on
   KMS blockchain).

    ```bash
    make run-kms
    ```

2. Run the fhevm coprocessor network (including a geth node).

    ```bash
    make run-coprocessor
    ```

3. Clone the dependant repositories, if not already present.

    ```bash
    make check-all-test-repo
    ```

4. Verify if the kms signer address is correctly configured.

   Value in `network-fhe-keys/eth_address_signer` should match
   `ADDRESS_KMS_SIGNER_0` in `work_dir/fhevm/.env.example.deployment`. If not
   update ENV file. 

5. Fund test accounts and deploy the fhevm solidity contracts.

    ```bash
    make prepare-e2e-test
    ```

    If prompted to install npm dependencies, enter `y`.

6. In a separate terminal, checkout relevant branch and run the gateway.


    ```bash
    cd $path-to-kms-core
    git checkout mano/update-config-for-rc20
    cd blockchain/gateway
    ```
  🚨 For **threshold mode** update the gateway config file (__config/gateway.toml__) with the following parameters:
  - mode = "threshold" (default is centralized)
  - key_id = "d4d17a412a6533599b010c8ffc3d6ebdc6b1cfad" (default is "408d8cbaa51dece7f782fe04ba0b1c1d017b1088")


    ```bash
    cargo run --bin gateway
    ```


    Wait for the gateway to start listening for blocks and print block numbers.

    ```bash
    ...
    2024-10-16T15:35:22.876765Z  INFO gateway::events::manager: 🧱 block number: 10
    2024-10-16T15:35:27.787809Z  INFO gateway::events::manager: 🧱 block number: 11
    2024-10-16T15:35:27.787809Z  INFO gateway::events::manager: 🧱 block number: 12
    2024-10-16T15:35:27.787809Z  INFO gateway::events::manager: 🧱 block number: 13
    ...
    ```

7. From the fhevm repo, run one of the test for trivial decryption.

    ```bash
    cd work_dir/fhevm && npx hardhat test --grep 'test async decrypt uint32$'
    ```

8. To tear down the setup, stop the docker containers:

    ```
    make stop-coprocessor
    make stop-full
    ```

    The gateway will automatically exit as the connection will be closed from blockchain side.

## Note on test results (for trivial decrypt)

1. PASSING TESTS - All tests for trivial decrypt should now pass.

    ```bash
    npx hardhat test --network localCoprocessor --grep 'test async decrypt bool$'
    npx hardhat test --network localCoprocessor --grep 'test async decrypt uint4$'
    npx hardhat test --network localCoprocessor --grep 'test async decrypt uint8$'
    npx hardhat test --network localCoprocessor --grep 'test async decrypt uint16$'
    npx hardhat test --network localCoprocessor --grep 'test async decrypt uint32$'
    npx hardhat test --network localCoprocessor --grep 'test async decrypt uint64$'
    npx hardhat test --network localCoprocessor --grep 'test async decrypt address$'
    ```
