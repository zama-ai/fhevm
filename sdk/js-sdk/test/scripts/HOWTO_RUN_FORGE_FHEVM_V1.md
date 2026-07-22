## Run Anvil on port 8544

```sh
anvil --port 8544
```

## Deploy cleartext stack v1

1. `git clone https://github.com/zama-ai/forge-fhevm.git`
2. `cd forge-fhevm`
3. `./deploy-local.sh --anvil-port 8544 --skip-build`

## Deploy FHETest.sol

```sh
./fhetest-deploy.sh --chain localcleartext
```

## Run tests

```sh
CHAIN=localcleartext_legacy npx vitest run --config test/fheTest/vitest.config.ts ethers-cleartext
```

```sh
CHAIN=localcleartext_legacy npx vitest run --config test/fheTest/vitest.config.ts viem-cleartext
```
