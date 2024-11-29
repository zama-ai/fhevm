# e2e tests

## Install

```bash
npm install
```

## Configuration

1. Copy `.env.example` to `.env` and edit addresses with the correct one.
2. Edit your `.env` file with correct values.
3. Edit `contracts/E2EFHEVMConfig.sol` and set correct addresses of your fhEVM.
4. Edit `hardhat.config.ts` to set the `defaultNetwork`. By default, it is set to Sepolia, but you can a different one
   or add your own L1 address. 4. Fund your wallet
5. Fund the primary wallet derived from your mnemomic. If you don't know what is the public address, run
   `npm run task:accounts`

## Run

```bash
npm run test
```

or if you want to run only one test

```bash
npm run test test/encryptedERC20/EncryptedERC20.ts
```
