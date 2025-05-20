# fhevm dev setup for E2E tests

## All-in-One CLI

The test suite offers a unified CLI for all operations:

```sh
cd fhevm
# Deploy the entire stack
./fhevm-cli deploy

# Run specific tests
./fhevm-cli test input-proof
# Trivial
./fhevm-cli test user-decryption
# Trivial
./fhevm-cli test public-decryption
./fhevm-cli test erc20

# Upgrade a specific service
./fhevm-cli upgrade coprocessor

# View logs
./fhevm-cli logs relayer

# Clean up
./fhevm-cli clean
