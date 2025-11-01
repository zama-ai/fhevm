# Deployment guide for testing

This guide explains all the steps needed to deploy a test instance of the full protocol. This is not the real deployment process, and is intended only for end-to-end testing of governance.

## Fund 2 accounts on both chains

Fill both `DEPLOYER_PRIVATE_KEY` and `PROTOCOL_DEPLOYER_PRIVATE_KEY` values inside the `.env` contained in this directory (you can start by copying the `.env.example`).

Those 2 private keys *must* be different.

Make sure to fund those 2 accounts on both Ethereum Sepolia Testnet and Gateway Testnet networks.

## Deploy an Aragon DAO on Sepolia

Go to `https://app.aragon.org/`[https://app.aragon.org/] and use the front-end to deploy a DAO on sepolia. *Important:* the account connected to the Aragon front-end and deploying this DAO must be the one corresponding to `DEPLOYER_PRIVATE_KEY`.

Once deployed, copy the address of the deployed DAO contract inside the `.env`: this should be the `DAO_ADDRESS` value.

## Deploy the Zama token and OFT

## Deploy Gateway contracts

