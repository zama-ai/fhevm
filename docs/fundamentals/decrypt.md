# Decrypt and reencrypt

## Decrypt

We allow explicit decryption requests for any encrypted type. The values are decrypted with the network private key (the threshold decryption protocol is in the works).

### Example

The decryption operation is asynchronous. To use it, your contract must extend the `OracleCaller` contract. This will import automatically the `Oracle` solidity library as well. See the following example:

```solidity
pragma solidity ^0.8.20;

import "fhevm/lib/TFHE.sol";
import "fhevm/oracle/OracleCaller.sol";

contract TestAsyncDecrypt is OracleCaller {
  ebool xBool;
  bool public yBool;

  constructor() {
      xBool = TFHE.asEbool(true);
  }

  function requestBool() public {
    ebool[] memory cts = new ebool[](1);
    cts[0] = xBool;
    Oracle.requestDecryption(cts, this.myCustomCallback.selector, 0, block.timestamp + 100);
  }

  function myCustomCallback(uint256 /*requestID*/, bool decryptedInput) public onlyOracle returns (bool) {
    yBool = decryptedInput;
    return yBool;
  }
```

Note that an [`OraclePredeploy`](../../oracle/OraclePredeploy.sol) contract is already predeployed on the fhEVM testnet, and a default relayer account is already added. Relayers are the only accounts authorized to fulfil the decryption requests. However `OraclePredeploy` would still check the KMS signature during the fulfilment, so we trust the relayer only to forward the request on time, a rogue relayer could not cheat by sending fake decryption results.

The interface of the `Oracle.requestDecryption` function from previous snippet is the following:

```solidity
function requestDecryption(
    eXXX[] memory ct,
    bytes4 callbackSelector,
    uint256 msgValue,
    uint256 maxTimestamp
) returns(uint256 requestID)
```

The first argument, `ct`, should be an array of ciphertexts of a single same type i.e `eXXX` stands for either `ebool`, `euint4`, `euint8`, `euint16`, `euint32` or `euint64`. `ct` is the list of ciphertexts that are requested to be decrypted. Calling `requestDecryption` will emit an `EventDecryptionEXXX` on the `OraclePredeploy` contract which will be detected by a relayer. Then, the relayer will send the corresponding ciphertexts to the KMS for decryption before fulfilling the request.

`callbackSelector` is the function selector of the callback function which will be called by the `OraclePredeploy` contract once the relayer fulfils the decryption request. Notice that the callback function should always follow this convention:

```solidity
function [callbackName](uint256 requestID, XXX x_0, XXX x_1, ..., XXX x_N-1) external onlyOracle
```

Here `callbackName` is a custom name given by the developer to the callback function, `requestID` will be the request id of the decryption (could be commented if not needed in the logic, but must be present) and `x_0`, `x_1`, ... `x_N-1` are the results of the decryption of the `ct` array values, i.e their number should be the size of the `ct` array.

`msgValue` is the value in native tokens to be sent to the calling contract during fulfilment, i.e when the callback will be called with the results of decryption.

`maxTimestamp` is the maximum timestamp after which the callback will not be able to receive the results of decryption, i.e the fulfilment transaction will fail in this case. This can be used for time-sensitive applications, where we prefer to reject decryption results on too old, out-of-date, values.

> **_WARNING:_**
> Notice that the callback should be protected by the `onlyOracle` modifier to ensure security, as only the `OraclePredeploy` contract should be able to call it.

Finally, if you need to pass additional arguments to be used inside the callback, you could use any of the following utility functions which would store additional values in the storage of your smart contract:

```solidity
function addParamsEBool(uint256 requestID, ebool _ebool) internal;

function addParamsEUint4(uint256 requestID, euint4 _euint4) internal;

function addParamsEUint8(uint256 requestID, euint8 _euint8) internal;

function addParamsEUint16(uint256 requestID, euint16 _euint16) internal;

function addParamsEUint32(uint256 requestID, euint32 _euint32) internal;

function addParamsEUint64(uint256 requestID, euint64 _euint64) internal;

function addParamsAddress(uint256 requestID, address _address) internal;

function addParamsUint(uint256 requestID, uint256 _uint) internal;

function getParamsEBool(uint256 requestID) internal;

function getParamsEUint4(uint256 requestID) internal;

function getParamsEUint8(uint256 requestID) internal;

function getParamsEUint16(uint256 requestID) internal;

function getParamsEUint32(uint256 requestID) internal;

function getParamsEUint64(uint256 requestID) internal;

function getParamsAddress(uint256 requestID) internal;

function getParamsUint(uint256 requestID) internal;
```

For example, see this snippet where we add two `uint`s during the request call, to make them available later during the callback:

```solidity
pragma solidity ^0.8.20;

import "../lib/TFHE.sol";
import "../oracle/OracleCaller.sol";

contract TestAsyncDecrypt is OracleCaller {
  euint32 xUint32;
  uint32 public yUint32;

  constructor() {
      xUint32 = TFHE.asEuint32(32);
  }

  function requestUint32(uint32 input1, uint32 input2) public {
      euint32[] memory cts = new euint32[](1);
      cts[0] = xUint32;
      uint256 requestID = Oracle.requestDecryption(cts, this.callbackUint32.selector, 0, block.timestamp + 100);
      addParamsUint(requestID, input1);
      addParamsUint(requestID, input2);
  }

  function callbackUint32(uint256 requestID, uint32 decryptedInput) public onlyOracle returns (uint32) {
    uint256[] memory params = getParamsUint(requestID);
    unchecked {
        uint32 result = uint32(params[0]) + uint32(params[1]) + decryptedInput;
        yUint32 = result;
        return result;
    }
}
```

When the decryption request is fufilled by the relayer, the `OraclePredeploy` contract, when calling the callback function, will also emit one of the following events, depending on the type of requested ciphertext:

```solidity
event ResultCallbackBool(uint256 indexed requestID, bool success, bytes result);
event ResultCallbackUint4(uint256 indexed requestID, bool success, bytes result);
event ResultCallbackUint8(uint256 indexed requestID, bool success, bytes result);
event ResultCallbackUint16(uint256 indexed requestID, bool success, bytes result);
event ResultCallbackUint32(uint256 indexed requestID, bool success, bytes result);
event ResultCallbackUint64(uint256 indexed requestID, bool success, bytes result);
```

The first argument is the `requestID` of the corresponding decryption request, `success` is a boolean assessing if the call to the callback succeeded, and `result` is the bytes array corresponding the to return data from the callback.

## Reencrypt

The reencrypt functions takes as inputs a ciphertext and a public encryption key (namely, a [NaCl box](https://nacl.cr.yp.to/index.html)).

During reencryption, the ciphertext is decrypted using the network private key (the threshold decryption protocol is in the works). Then, the decrypted result is encrypted under the user-provided public encryption key. The result of this encryption is sent back to the caller as `bytes memory`.

It is also possible to provide a default value to the `reencrypt` function. In this case, if the provided ciphertext is not initialized (i.e., if the ciphertext handle is `0`), the function will return an encryption of the provided default value.

### Example

```solidity
TFHE.reencrypt(balances[msg.sender], publicKey, 0);
```

> _**NOTE:**_ If one of the following operations is called with an uninitialized ciphertext handle as an operand, this handle will be made to point to a trivial encryption of `0` before the operation is executed.

### Handle private reencryption

In the example above (`balanceOf`), this view function need to validate the user to prevent anyone to reencrypt any user's balance. To prevent this, the user provides a signature of the given public key. The best way to do it is to use [EIP-712 standard](https://eips.ethereum.org/EIPS/eip-712). Since this is something very useful, fhEVM library provide an abstract to use in your contract:

```solidity
import "fhevm/abstracts/Reencrypt.sol";

contract EncryptedERC20 is Reencrypt {
  ...
}
```

When a contract uses `Reencrypt` abstract, a modifier is available to check user signature.

```solidity
function balanceOf(
  bytes32 publicKey,
  bytes calldata signature
) public view onlySignedPublicKey(publicKey, signature) returns (bytes memory) {
  return TFHE.reencrypt(balances[msg.sender], publicKey, 0);
}
```

This signature can be generated on client side using [fhevmjs library](../guides/reencryption.md).
