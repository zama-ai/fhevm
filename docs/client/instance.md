# Setup an instance

fhevmjs provides two features:

- Encrypt an input with the blockchain's FHE public key
- Generate a private key and a public key to provide to `TFHE.reencrypt()`

## Instance

Before using these features, you need to create an instance and provide some informations:

- The blockchain chainId
- The blockchain's FHE public key

You can get this information directly from the blockchain you're using. For example with `ethers`:

```javascript
import { BrowserProvider } from "ethers";
import { createInstance } from "fhevmjs";

const createFhevmInstance = async () => {
  const provider = new BrowserProvider(window.ethereum);

  // 1. Get chain id
  const network = await provider.getNetwork();
  const chainId = +network.chainId.toString();

  // 2. Get blockchain public key
  const ret = await provider.call({
    // fhe lib address, may need to be changed depending on network
    to: "0x000000000000000000000000000000000000005d",
    // first four bytes of keccak256('fhePubKey(bytes1)') + 1 byte for library
    data: "0xd9d47bb001",
  });
  const decoded = ethers.AbiCoder.defaultAbiCoder().decode(["bytes"], ret);
  const publicKey = decoded[0];

  // 3. Create instance
  instance = createInstance({ chainId, publicKey });
  return instance;
};
```

Important: Since the instance memorizes user's signature, you need to refresh the instance if the user uses another wallet address. Otherwise, you will encounter issue during reencryption.

## Export keypairs

When a user generate and sign an [EIP712 token](reencryption.md) for a contract, you can export these tokens with `instance.serializeKeypairs()`. This method will return all keypairs and signature associated with a contract.

```javascript
{
   '0x1c786b8ca49D932AFaDCEc00827352B503edf16c': {
     signature: '0x6214e232b2dae4d8d2c99837dd1af0...',
     publicKey: '7b2352b10cb4e379fc89094c445acb8b2161ec23a3694c309e01e797ab2bae22',
     privateKey: '764d194c6c686164fa5eb3c53ef3f7f5b90985723f19e865baf0961dd28991eb',
   }
}
```

You can store this object in the user's local storage, enabling you to load it in the next user session. You must save this information per wallet since it contains the user's signature.

```javascript
const keypairs = instance.serializeKeypairs();
window.localStorage.setItem(`fhevmKeypairs-${wallet}`, JSON.stringify(keypairs));
```

## Initialize instance with stored keypairs

You can load previously stored keypairs to initialize the instance.

```javascript
import { BrowserProvider } from "ethers";
import { createInstance } from "fhevmjs";

const createFhevmInstance = async (wallet) => {
  const provider = new BrowserProvider(window.ethereum);

  // 1. Get chain id
  const network = await provider.getNetwork();
  const chainId = +network.chainId.toString();

  // 2. Get blockchain public key
  const ret = await provider.call({
    // fhe lib address, may need to be changed depending on network
    to: "0x000000000000000000000000000000000000005d",
    // first four bytes of keccak256('fhePubKey(bytes1)') + 1 byte for library
    data: "0xd9d47bb001",
  });
  const decoded = ethers.AbiCoder.defaultAbiCoder().decode(["bytes"], ret);
  const publicKey = decoded[0];

  const storedKeypairs = window.localStorage.get(`fhevmKeypairs-${wallet}`) || null;
  const keypairs = storedKeypairs ? JSON.parse(storedKeypairs) : {};

  // 3. Create instance
  instance = createInstance({ chainId, publicKey, keypairs });
  return instance;
};
```
