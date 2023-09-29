# Examples

## Nodejs

### Transfer ERC-20 tokens

```javascript
const { createInstance } = require("fhevmjs");
const { Wallet, JsonRpcProvider, Contract } = require("ethers");

const contractInfo = require("./EncryptedERC20.json");

const CONTRACT_ADDRESS = "0x309cf2aae85ad8a1db70ca88cfd4225bf17a7482";

const provider = new JsonRpcProvider(`https://devnet.zama.ai/`);

const signer = new Wallet("0x92293977156de6e03b20b26708cb4496b523116190b5c32d77cee8286d0c41f6", provider);

let _instance;

const getInstance = async () => {
  if (_instance) return _instance;

  // 1. Get chain id
  const network = await provider.getNetwork();
  const chainId = +network.chainId.toString(); // Need to be a number

  // Get blockchain public key
  const publicKey = await provider.call({
    to: "0x0000000000000000000000000000000000000044",
  });

  // Create instance
  _instance = createInstance({ chainId, publicKey });
  return _instance;
};

const transfer = async (to, amount) => {
  // Initialize contract with ethers
  const contract = new Contract(CONTRACT_ADDRESS, contractInfo.abi, signer);
  // Get instance to encrypt amount parameter
  const instance = await getInstance();
  const encryptedAmount = instance.encrypt32(amount);

  const transaction = await contract["transfer(address,bytes)"](to, encryptedAmount);
  return transaction;
};

transfer("0xa83a498Eee26f9594E3A784f204e507a5Fae3210", 10);
```

## Get balance

```javascript
const { createInstance } = require("fhevmjs");
const { Wallet, JsonRpcProvider, Contract } = require("ethers");

const contractInfo = require("./EncryptedERC20.json");

const CONTRACT_ADDRESS = "0x309cf2aae85ad8a1db70ca88cfd4225bf17a7482";

const provider = new JsonRpcProvider(`https://devnet.zama.ai/`);

const signer = new Wallet("0x92293977156de6e03b20b26708cb4496b523116190b5c32d77cee8286d0c41f6", provider);

let _instance;

const getInstance = async () => {
  if (_instance) return _instance;

  // 1. Get chain id
  const network = await provider.getNetwork();

  const chainId = +network.chainId.toString();

  // Get blockchain public key
  const publicKey = await provider.call({
    to: "0x0000000000000000000000000000000000000044",
  });

  // Create instance
  _instance = createInstance({ chainId, publicKey });
  return _instance;
};

const getBalance = async () => {
  // Initialize contract with ethers
  const contract = new Contract(CONTRACT_ADDRESS, contractInfo.abi, signer);

  // Get instance to encrypt amount parameter
  const instance = await getInstance();

  // Generate token to decrypt
  const generatedToken = instance.generateToken({
    verifyingContract: CONTRACT_ADDRESS,
  });

  // Sign the public key
  const signature = await signer.signTypedData(
    generatedToken.token.domain,
    { Reencrypt: generatedToken.token.types.Reencrypt }, // Need to remove EIP712Domain from types
    generatedToken.token.message,
  );
  instance.setTokenSignature(CONTRACT_ADDRESS, signature);

  // Call the method
  const encryptedBalance = await contract.balanceOf(generatedToken.publicKey, signature);

  // Decrypt the balance
  const balance = instance.decrypt(CONTRACT_ADDRESS, encryptedBalance);
  return balance;
};

getBalance().then((balance) => {
  console.log(balance);
});
```
