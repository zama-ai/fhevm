# Setup an instance

First, you need to create an instance. An instance allows you to:

- Encrypt inputs with blockchain public key
- Manage user keys to reencrypt contract's encrypted data

## createInstance

### Parameters

- `params` (required):

  - `chainId` (required): Id of the chain
  - `publicKey` (required): Public key of the blockchain
  - `keypairs` (optional): A list of keypairs associated with contract

### Returns

- `Promise<FhevmInstance>`

### Example

```javascript
import { createInstance } from 'fhevmjs';

const keypairs = {
  '0x1c786b8ca49D932AFaDCEc00827352B503edf16c': {
    publicKey:
      '7b2352b10cb4e379fc89094c445acb8b2161ec23a3694c309e01e797ab2bae22',
    privateKey:
      '764d194c6c686164fa5eb3c53ef3f7f5b90985723f19e865baf0961dd28991eb',
    signature:
      '0x5668c087804bd8b2f95b17d7f60599502bf7d539b0b19a4d989c3a5e422c77de37771be1f991223088e968a7e18330c7ece973f527eec03b97f219447d4833401b',
  },
};

const initInstance = async () => {
  // 1. Get chain id
  const chainIdHex = await window.ethereum.request({ method: 'eth_chainId' });
  const chainId = parseInt(chainIdHex, 16);

  // Get blockchain public key
  const publicKey = await window.ethereum.request({
    method: 'eth_call',
    params: [{ from: null, to: '0x0000000000000000000000000000000000000044' }],
  });

  // Create instance
  return createInstance({ chainId, publicKey, keypairs });
};

initInstance().then((instance) => {
  console.log(instance.serializeKeypairs());
});
```
