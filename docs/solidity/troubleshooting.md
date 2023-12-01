# Troubleshooting

## Transaction

### How to cancel a stuck transaction?

The current devnet has a gas limit of 10,000,000. If you send a transaction exceeding this limit, it won't be executed. Consequently, your wallet won't be able to emit a new transaction. To address this, emit a new transaction with the same nonce but the correct gas limit.
In Metamask, you can enforce the use of a specific nonce by enabling the feature in 'Advanced Settings'.

### My transaction reverts but I can't get the error message

When you include a require statement in a transaction like `require(TFHE.decrypt(ebool), "It didn't work");`, the revert message will not be returned if `ebool` is false.

### My transaction seems to be reverted randomly (or silently)

When you invoke the gas estimation method, fhEVM may not provide an accurate estimation if your code includes `TFHE.decrypt` because it is unable to determine the actual value. In such cases, the gas estimation assumes that `TFHE.decrypt()` returns `1`. Depending on your code, this assumption may lead to a deviation from the actual gas usage. To mitigate this, consider adding a 20% buffer to the gas estimation or more. However, be cautious not to exceed 10,000,000 as the upper limit. We've written a method, available in the hardhat template, to tackle this issue. Feel free to use it.

```typescript
export const createTransaction = async <A extends [...{ [I in keyof A]-?: A[I] | Typed }]>(
  method: TypedContractMethod<A>,
  ...params: A
) => {
  const gasLimit = await method.estimateGas(...params);
  const updatedParams: ContractMethodArgs<A> = [
    ...params,
    { gasLimit: Math.min(Math.round(+gasLimit.toString() * 1.2), 10000000) },
  ];
  return method(...updatedParams);
};
```

## Contract

### I've defined certain constants as properties, but it appears that I'm unable to utilize them.

First, to understand the issue, you need to know that a `euint32` or a `ebool` are `uint256` under the hood: they are a 256bits hash of the actual ciphertext.
So if you set your properties directly in the contract as follows:

```
euint32 constant private MY_CONSTANT = TFHE.asEuint32(42);
```

The `TFHE.asEuint32(42)` will be executed during compilation to evaluate your property `MY_CONSTANT`, because the compiler expect to have an actual value. Since you're not calling the precompiles which would return a trivial encryption of `42`, you get a `0` value.

### I return bytes of the ciphertext, but the ciphertext is very small.

In the contract, obtaining the bytes of the ciphertext is not possible. As outlined in [our whitepaper](https://github.com/zama-ai/fhevm/blob/main/fhevm-whitepaper.pdf), you manipulate the hash of the ciphertext in the EVM; the actual ciphertext is only utilized when interacting with the precompiles (`TFHE.decrypt`, `TFHE.add`, ...).

## Blockchain

### I get the error "fhePubKey FHE public key hash doesn't match one stored in state"

This error occurs when you haven't yet published any contracts on the blockchain. To resolve this, simply deploy a contract on the blockchain.

### How can I listen to blockchain events?

You can listen for events using WebSocket on port `8546` of your local Docker image. Alternatively, if you prefer to do it on Zama's devnet, you can use the WebSocket server available at `wss://devnet.ws.zama.ai/`.

```javascript
const WEBSOCKET_URL = "ws://localhost:8546/";
// const WEBSOCKET_URL = 'wss://devnet.ws.zama.ai/';
const wsProvider = new WebsocketProvider(WEBSOCKET_URL);
const contract = new Contract(address, abi, await wsProvider.getSigner());
contract.on(contract.filters.GameLaunched, () => {
  console.log("A new game has been launched");
});
```
