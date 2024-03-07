# Encrypt an input

The library provides a set of functions to encrypt integers of various sizes (Bool, 4, 8, 16, 32 and 64 bits) using the blockchain's public key. These encrypted integers can then be securely used as parameters for smart contract methods within the blockchain ecosystem.

```javascript
const instance = getInstance();

const encryptedParam8 = instance.encryptBool(true);
const encryptedParam8 = instance.encrypt4(3);
const encryptedParam8 = instance.encrypt8(14);
const encryptedParam16 = instance.encrypt16(650);
const encryptedParam32 = instance.encrypt32(71721057);
const encryptedParam64 = instance.encrypt64(71721075);
```
