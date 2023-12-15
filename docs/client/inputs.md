# Encrypt an input

The library provides a set of functions to encrypt integers of various sizes (8, 16, and 32 bits) using the blockchain's public key. These encrypted integers can then be securely used as parameters for smart contract methods within the blockchain ecosystem.

```javascript
const instance = getInstance();

const encryptedParam8 = instance.encrypt8(14);
const encryptedParam16 = instance.encrypt16(1234);
const encryptedParam32 = instance.encrypt32(71721075);
```
