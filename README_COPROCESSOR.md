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
* [nodejs v20](https://nodejs.org/en/download/package-manager)

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

2. Go to `kms-core`, checkout `levent/candidate-release-rc25` and run the key-gen:
  
    ```
    git checkout levent/candidate-release-rc25
    cd blockchain/simulator
    cargo run --bin simulator -- -f config/local_threshold.toml insecure-key-gen
    ```
    You should see the public key material here: [http://localhost:9001/browser/kms](http://localhost:9001/browser/kms)
    Now, run the CRS generation:
    ```
    cargo run --bin simulator -- -f config/local_threshold.toml crs-gen --max-num-bits 2048
    ```

    Tips: 
    One can check the status by checking the connector log:

    <details>
    <summary> 💡 Check CRS generation status as it may take ~ 20 mn </summary>
  
    ```bash
    docker logs zama-kms-threshold-dev-kms-connector-1-1 > log_connector.txt 2>&1  &&  grep crsgen log_connector.txt -i
    ```
    ```bash
    2024-11-07T14:13:09.775076Z  INFO kms_blockchain_connector::application::kms_core_sync: Running KMS operation with value: CrsGen(CrsGenValues { max_num_bits: 2048, eip712_name: "eip712_name", eip712_version: "1.0.4", eip712_chain_id: HexVector([42, 0, 0... 0, 0, 0, 0, 0, 0, 0, 0]), eip712_verifying_contract: "0x00dA6BF26964af9D7EED9e03E53415d37aa960EE", eip712_salt: Some(HexVector([0, 1, 2, 3, , 31])) })
        2024-11-07T14:41:09.871344Z  INFO kms_blockchain_connector::application::kms_core_sync: Sending response to the blockchain: CrsGenResponse
        2024-11-07T14:41:09.871382Z  INFO send_result{tx_id=7087d7a61cbbd4dc0bbd1702107502bb9b88d00b}: kms_blockchain_connector::infrastructure::blockchain: Sending result to contract: ExecuteContractRequest { message: KmsMessage { txn_id: Some(TransactionId(HexVector([112, 135, 215, 166, 28, 187, 212, 220, 11, 189, 23, 2, 16, 117, 2, 187, 155, 136, 208, 11]))), value: CrsGenResponse(CrsGenResponseValues { request_id: "7087d7a61cbbd4dc0bbd1702107502bb9b88d00b", digest: "370d1b033f45014a3a546d111383d5f7b8ee5ec5", signature: HexVector([64, 0, 0, 0, ...44, 252]), max_num_bits: 2048, param: Default }) }, gas_limit: 3000000, funds: None }
    ```
    </details> 
   

3. Run the fhevm coprocessor network (including a geth node).

    ```bash
    make run-coprocessor
    ```

   📝 At this step keys are not loaded in coprocessor DB. 

4. In a separate terminal, return to the same branch `levent/candidate-release-rc25` where the keys and crs have been generated. 


    ```bash
    cd $path-to-kms-core
    cd blockchain/gateway
    ```
    🚨 For **threshold mode** update the gateway config file (__config/gateway.toml__) with the following parameters:
    - mode = "threshold" (default is centralized)

    ```bash
    cargo run --bin gateway
    ```

    Wait for the gateway to start listening for blocks and print block numbers.

    ```bash
    ...
    2024-10-16T15:35:22.876765Z  INFO gateway::events::manager: 🧱 block number: 10
    2024-10-16T15:35:27.787809Z  INFO gateway::events::manager: 🧱 block number: 11
    ...
    ```

4. Retrieve the fhe keys and load them in coprocessor DB

    ```bash
    make init-db
    ```
    This command will make a call to `/keyurl` endpoint of gateway and retrieve the corresponding keys from the minio server.
    Then, keys will be copied into right folder in coprocessor and inserted in the DB

4. Clone the dependant repositories, if not already present.

    ```bash
    make check-all-test-repo
    ```

4. Verify if the kms signer address is correctly configured.

   Value in `network-fhe-keys/signerN` should match
   `ADDRESS_KMS_SIGNER_N` in `work_dir/fhevm/.env.example.deployment`. If not
   update ENV file. 

5. Fund test accounts and deploy the fhevm solidity contracts.

    ```bash
    make prepare-e2e-test
    ```

    If prompted to install npm dependencies, enter `y`.



7. From the fhevm repo, run one of the test for trivial decryption.

    ```bash
    cd work_dir/fhevm && npx hardhat test --grep 'test async decrypt uint32$'
    ```

8. To tear down the setup, stop the docker containers:

    ```
    make stop-coprocessor
    make stop-kms
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
