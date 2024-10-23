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

## Steps to run the setup

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

3. Verify if the kms signer address is correctly configured.

   Value in `network-fhe-keys/eth_address_signer` should match
   `ADDRESS_KMS_SIGNER_0` in `work_dir/fhevm/.env.example.deployment`. If not
   update ENV file. 

4. Fund test accounts and deploy the fhevm solidity contracts.

    ```bash
    make prepare-e2e-test
    ```

    If prompted to install npm dependencies, enter `y`.

4. In a separate terminal, checkout relevant branch and run the gateway.

    ```bash
    cd $path-to-kms-core
    git checkout mano/update-config-for-rc20
    cd blockchain/gateway
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

5. From the fhevm repo, run one of the test for trivial decryption.

    ```bash
    cd work_dir/fhevm && npx hardhat test --grep 'test async decrypt uint32$'
    ```

6. To tear down the setup, stop the docker containers:

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
