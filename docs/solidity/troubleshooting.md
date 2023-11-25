# Troubleshooting

## How to cancel a stuck transaction?

The current devnet has a gas limit of 10,000,000. If you send a transaction exceeding this limit, it won't be executed. Consequently, your wallet won't be able to emit a new transaction. To address this, emit a new transaction with the same nonce but the correct gas limit.
In Metamask, you can enforce the use of a specific nonce by enabling the feature in 'Advanced Settings'.

## How can I listen to blockchain events?

Libraries like ethers enable event listening through HTTP polling. However, it is important to note that this functionality is limited to a `JsonRpcProvider` and is not compatible with the `BrowserProvider` you'd use if you develop with user's browser wallet.

```javascript
const provider = new BrowserProvider(window.metamask);
const jsonRpcProvider = new JsonRpcProvider("https://devnet.zama.ai");
const contract = new Contract(address, abi, await provider.getSigner());
contract.on("NewGame", () => {
  console.log("A new game has been launched");
});
```

## My transaction reverts but I can't get the error message

When you include a require statement in a transaction like `require(TFHE.decrypt(ebool), "It didn't work");`, the revert message will not be returned if `ebool` is false.

## I've defined certain constants as properties, but it appears that I'm unable to utilize them."

First, to understand the issue, you need to know that a `euint32` or a `ebool` are `uint256` under the hood: they are a 256bits hash of the actual ciphertext.
So if you set your properties directly in the contract as follows:

```
euint32 constant private MY_CONSTANT = TFHE.asEuint32(42);
```

The `TFHE.asEuint32(42)` will be executed during compilation to evaluate your property `MY_CONSTANT`, because the compiler expect to have an actual value. Since you're not calling the precompiles which would return a trivial encryption of `42`, you get a `0` value.
