# Relayer SDK

The document provides an overview of the key features of Zama's FHEVM Relayer Javascript SDK.

This SDK provides all the functionnalities to interact with the FHEVM without interacting directly with the [Gateway chain](../protocol/architecture/gateway.md).
This means that FHEVM clients need only to hold a wallet of the FHEVM host chain.
All interactions with the Gateway chain are done through HTTP calls to Zama's Relayer, which pays for it on the Gateway chain.

## Setup

The first step to using the Relayer SDK is to properly setup the configuration to use.
For more information see the [configuration page.](./initialization.md).

## Input registration

Once the Relayer SDK is setup the first operation to interact with an FHEVM smart contract is to register a new encrypted value.
For more information see the [input registration page.](./input.md).

## User decryption

Once an FHEVM smart contract did some computation, and set the proper ACL, a user can decrypt the underlying value (if authorized).
This is done by re-encrypting the encrypted value using the user's set of public/private key.

For more information see the [user/private decryption page.](./user-decryption.md).
For more information about ACL refer to the [Solidity ACL page](../solidity-guides/acl/README.md).

## Public decryption

Once an FHEVM smart contract did some computation, and set the proper ACL, a value could be decrypted by anyone.
This can be done in two ways, either calling on-chain the public decryption Oracle or through HTTP using the Relayer SDK.

For more information using HTTP see the [HTTP public decryption page.](./public-decryption.md-decryption.md).
For more information using on-chain Oracle see the [Oracle public decryption page.](../solidity-guides/decryption/oracle.md).
For more information about ACL refer to the [Solidity ACL page](../solidity-guides/acl/README.md).

