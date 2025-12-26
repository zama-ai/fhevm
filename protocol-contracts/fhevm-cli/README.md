# Fhevm CLI tool

This CLI tool contains tools for common useful tasks needed to interact with FHEVM.

# Initial setup

Install dependencies via: 
```
npm i
```

And enter all needed values in a `.env` file (see available variables in [`.env.example`](./.env.example)).

If you want to interact with Ethereum mainnet, do not forget to get an API key and set it via: 

```
npx hardhat vars set ZAMA_FHEVM_API_KEY <your-api-key>
```

# Request a user decryption

To request a user decryption, use a command similar to this example: 

```
npx hardhat task:userDecrypt --handle 0x980769a416dbe44044fac20626c9521085a3ba57acff00000000000000010500 --contract-address 0xb1A7026C28cB91604FB7B1669f060aB74A30c255 --encrypted-type euint64 --network mainnet
```

Replace the `handle` flag value from previous command by a handle you are allowed for, and the `contract-address` value with a contract address which is also allowed for this handle. Make sure to use the correct value for `encrypted-type` (e.g `euint64`, `ebool` , `eaddress`,  `euint8`, etc), and to chose correct `network`, ie `mainnet` or `testnet`. It is also required, before running this command, to have filled correct environement vriable values inside your `.env` file: i.e `PRIVATE_KEY` for the user's private key requesting the user decryption, as well as `MAINNET_RPC_URL` (for mainnet) or `TESTNET_RPC_URL` (for testnet).

# Request an encryption

# Request a public decryption