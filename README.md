# How to run the new fhEVM

THis documentation is only temporary, it allows to run the last fhEVM with the fhe keys generated using kms-core dedicated binary. 


# Fast run and test


```bash
make e2e-test
```

Note: if you get `override the existing name orchestrator [y/N]: `, just ^C and  run `make clean-node-storage` to remove the state.


# Init fhEVM

```bash
make init-ethermint-node 
```

This will initialize and generate the fhe keys.
IMPORTANT: ensure to have 15 GB of empty ram to generate the keys.

# Run fhEVM

```bash
make run-ethermint
# Check the logs
docker logs ethermintnode0 -f     
```

# Stop fhEVM

```bash
make stop-ethermint
```

# Fresh start

```bash
make clean
```

Note: Fhe keys are in res/keys folder, delete them to regenerate new keys at ```make init-ethermint-node```


# Test using fhevm

```bash
# if not executed before
make init-ethermint-node 
# if not executed before
make run-ethermint
# In new terminal
make run-e2e-test
```

You should see the pre-processing steps, i.e. deployment of ACL, Gateway, KMSVerifier ...

```bash
Generating typings for: 22 artifacts in dir: types for target: ethers-v6
Successfully generated 50 typings!
Compiled 22 Solidity files successfully (evm target: cancun).
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
gatewayContractAddress written to gateway/.env.gateway successfully!
gateway/lib/PredeployAddress.sol file has been generated successfully.
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
ACL address 0x2Fb4341027eb1d2aD8B5D9708187df8633cAFA92 written successfully!
./lib/ACLAddress.sol file generated successfully!
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
TFHE Executor address 0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c written successfully!
./lib/FHEVMCoprocessorAddress.sol file generated successfully!
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
KMS Verifier address 0x12B064FB845C1cc05e9493856a1D637a73e944bE written successfully!
./lib/KMSVerifierAddress.sol file generated successfully!
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
ACL was deployed at address: 0x2Fb4341027eb1d2aD8B5D9708187df8633cAFA92
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
TFHEExecutor was deployed at address: 0x05fD9B5EFE0a996095f42Ed7e77c390810CF660c
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
KMSVerifier was deployed at address: 0x12B064FB845C1cc05e9493856a1D637a73e944bE
bigint: Failed to load bindings, pure JS will be used (try npm run rebuild?)
privateKey 717fd99986df414889fd8b51069d4f90a50af72e542c58ee065f5883779099c6
ownerAddress 0x305F1F471e9baCFF2b3549F9601f9A4BEafc94e1
GatewayContract was deployed at address:  0xc8c9303Cd7F337fab769686B593B87DC3403E0ce
Account 0x97F272ccfef4026A1F3f0e0E879d514627B84E69 was succesfully added as an gateway relayer


```

Then some tests, only ERC20 and rand for now: 

```bash
EncryptedERC20
    ✔ should mint the contract (5083ms)
    ✔ should transfer tokens between two users (12342ms)
    ✔ should not transfer tokens between two users (12324ms)
    ✔ should be able to transferFrom only if allowance is sufficient (23991ms)


Rand
    ✔ 8 bits generate and decrypt (25643ms)
    ✔ 8 bits generate with upper bound and decrypt (25160ms)
    ✔ 16 bits generate and decrypt (25706
```

