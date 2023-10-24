# Transfer tokens (Node)

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
  const ret = await provider.call({
    // fhe lib address, may need to be changed depending on network
    to: "0x000000000000000000000000000000000000005d",
    // first four bytes of keccak256('fhePubKey(bytes1)') + 1 byte for library
    data: "0xd9d47bb001",
  });
  const decoded = ethers.AbiCoder.defaultAbiCoder().decode(["bytes"], ret);
  const publicKey = decoded[0];

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
