# Decrypt and reencrypt

## How it's working

Validators of the blockchain do not own the blockchain's private key. Instead, the private key is owned by a Key Management Service (KMS). If the plaintext value is needed at some point, there are two ways to obtain it. Both methods are handled by a service called the Gateway.

- If the plaintext is needed for some logic in a contract, the Gateway acts as an oracle service: it will listen to decryption request events and return the decrypted value through a callback function.
- If the plaintext is needed by a dApp, the Gateway provides an API to reencrypt a ciphertext with the dApp's public key.

## Decryption

We allow explicit decryption requests for any encrypted type. The values are decrypted with the network private key.

![](asyncDecrypt.png)

### Example

The decryption operation is asynchronous. To use it, your contract must extend the `GatewayCaller` contract. This will import automatically the `Gateway` solidity library as well. See the following example:

```solidity
pragma solidity ^0.8.25;

import "fhevm/lib/TFHE.sol";
import "fhevm/gateway/GatewayCaller.sol";

contract TestAsyncDecrypt is GatewayCaller {
  ebool xBool;
  bool public yBool;

  constructor() {
      xBool = TFHE.asEbool(true);
  }

  function requestBool() public {
    ebool[] memory cts = new ebool[](1);
    cts[0] = xBool;
    Gateway.requestDecryption(cts, this.myCustomCallback.selector, 0, block.timestamp + 100);
  }

  function myCustomCallback(uint256 /*requestID*/, bool decryptedInput) public onlyGateway returns (bool) {
    yBool = decryptedInput;
    return yBool;
  }
```

Note that an [`GatewayContract`](../../gateway/GatewayContract.sol) contract is already predeployed on the fhEVM testnet, and a default relayer account is added through the specification of the environment variable `PRIVATE_KEY_GATEWAY_RELAYER` in the `.env` file. Relayers are the only accounts authorized to fulfil the decryption requests. However `GatewayContract` would still check the KMS signature during the fulfilment, so we trust the relayer only to forward the request on time, a rogue relayer could not cheat by sending fake decryption results (the KMS signature is in the works).

The interface of the `Gateway.requestDecryption` function from previous snippet is the following:

```solidity
function requestDecryption(
    eXXX[] memory ct,
    bytes4 callbackSelector,
    uint256 msgValue,
    uint256 maxTimestamp
) returns(uint256 requestID)
```

The first argument, `ct`, should be an array of ciphertexts of a single same type i.e `eXXX` stands for either `ebool`, `euint4`, `euint8`, `euint16`, `euint32`, `euint64` or `eaddress`. `ct` is the list of ciphertexts that are requested to be decrypted. Calling `requestDecryption` will emit an `EventDecryptionEXXX` on the `GatewayContract` contract which will be detected by a relayer. Then, the relayer will send the corresponding ciphertexts to the KMS for decryption before fulfilling the request.

`callbackSelector` is the function selector of the callback function which will be called by the `GatewayContract` contract once the relayer fulfils the decryption request. Notice that the callback function should always follow this convention:

```solidity
function [callbackName](uint256 requestID, XXX x_0, XXX x_1, ..., XXX x_N-1) external onlyGateway
```

Here `callbackName` is a custom name given by the developer to the callback function, `requestID` will be the request id of the decryption (could be commented if not needed in the logic, but must be present) and `x_0`, `x_1`, ... `x_N-1` are the results of the decryption of the `ct` array values, i.e their number should be the size of the `ct` array.

`msgValue` is the value in native tokens to be sent to the calling contract during fulfilment, i.e when the callback will be called with the results of decryption.

`maxTimestamp` is the maximum timestamp after which the callback will not be able to receive the results of decryption, i.e the fulfilment transaction will fail in this case. This can be used for time-sensitive applications, where we prefer to reject decryption results on too old, out-of-date, values.

> **_WARNING:_**
> Notice that the callback should be protected by the `onlyGateway` modifier to ensure security, as only the `GatewayContract` contract should be able to call it.

Finally, if you need to pass additional arguments to be used inside the callback, you could use any of the following utility functions during the request, which would store additional values in the storage of your smart contract:

```solidity
function addParamsEBool(uint256 requestID, ebool _ebool) internal;

function addParamsEUint4(uint256 requestID, euint4 _euint4) internal;

function addParamsEUint8(uint256 requestID, euint8 _euint8) internal;

function addParamsEUint16(uint256 requestID, euint16 _euint16) internal;

function addParamsEUint32(uint256 requestID, euint32 _euint32) internal;

function addParamsEUint64(uint256 requestID, euint64 _euint64) internal;

function addParamsEAddress(uint256 requestID, address _eaddress) internal;

function addParamsAddress(uint256 requestID, address _address) internal;

function addParamsUint(uint256 requestID, uint256 _uint) internal;
```

With their corresponding getter functions to be used inside the callback:

```solidity
function getParamsEBool(uint256 requestID) internal;

function getParamsEUint4(uint256 requestID) internal;

function getParamsEUint8(uint256 requestID) internal;

function getParamsEUint16(uint256 requestID) internal;

function getParamsEUint32(uint256 requestID) internal;

function getParamsEUint64(uint256 requestID) internal;

function getParamsEAddress(uint256 requestID) internal;

function getParamsAddress(uint256 requestID) internal;

function getParamsUint(uint256 requestID) internal;
```

For example, see this snippet where we add two `uint`s during the request call, to make them available later during the callback:

```solidity
pragma solidity ^0.8.25;

import "../lib/TFHE.sol";
import "../gateway/GatewayCaller.sol";

contract TestAsyncDecrypt is GatewayCaller {
  euint32 xUint32;
  uint32 public yUint32;

  constructor() {
      xUint32 = TFHE.asEuint32(32);
  }

  function requestUint32(uint32 input1, uint32 input2) public {
      euint32[] memory cts = new euint32[](1);
      cts[0] = xUint32;
      uint256 requestID = Gateway.requestDecryption(cts, this.callbackUint32.selector, 0, block.timestamp + 100);
      addParamsUint(requestID, input1);
      addParamsUint(requestID, input2);
  }

  function callbackUint32(uint256 requestID, uint32 decryptedInput) public onlyGateway returns (uint32) {
    uint256[] memory params = getParamsUint(requestID);
    unchecked {
        uint32 result = uint32(params[0]) + uint32(params[1]) + decryptedInput;
        yUint32 = result;
        return result;
    }
}
```

When the decryption request is fufilled by the relayer, the `GatewayContract` contract, when calling the callback function, will also emit one of the following events, depending on the type of requested ciphertext:

```solidity
event ResultCallbackBool(uint256 indexed requestID, bool success, bytes result);
event ResultCallbackUint4(uint256 indexed requestID, bool success, bytes result);
event ResultCallbackUint8(uint256 indexed requestID, bool success, bytes result);
event ResultCallbackUint16(uint256 indexed requestID, bool success, bytes result);
event ResultCallbackUint32(uint256 indexed requestID, bool success, bytes result);
event ResultCallbackUint64(uint256 indexed requestID, bool success, bytes result);
event ResultCallbackAddress(uint256 indexed requestID, bool success, bytes result);
```

The first argument is the `requestID` of the corresponding decryption request, `success` is a boolean assessing if the call to the callback succeeded, and `result` is the bytes array corresponding the to return data from the callback.

In your hardhat tests, if you sent some transactions which are requesting one or several decryptions and you wish to await the fulfilment of those decryptions, you should import the two helper methods `asyncDecrypt` and `awaitAllDecryptionResults` from the `asyncDecrypt.ts` utility file. This would work both when testing on an fhEVM node or in mocked mode. Here is a simple hardhat test for the previous `TestAsyncDecrypt` contract (more examples can be seen [here](../../test/gatewayDecrypt/testAsyncDecrypt.ts)):

```js
import { asyncDecrypt, awaitAllDecryptionResults } from "../asyncDecrypt";
import { getSigners, initSigners } from "../signers";
import { expect } from "chai";
import { ethers } from "hardhat";

describe("TestAsyncDecrypt", function () {
  before(async function () {
    await asyncDecrypt();
    await initSigners(3);
    this.signers = await getSigners();
  });

  beforeEach(async function () {
    const contractFactory = await ethers.getContractFactory("TestAsyncDecrypt");
    this.contract = await contractFactory.connect(this.signers.alice).deploy();
  });

  it("test async decrypt uint32", async function () {
    const tx2 = await this.contract.connect(this.signers.carol).requestUint32(5, 15, { gasLimit: 500_000 }); // custom gasLimit to avoid gas estimation error in fhEVM mode
    await tx2.wait();
    await awaitAllDecryptionResults();
    const y = await this.contract.yUint32();
    expect(y).to.equal(52); // 5+15+32
  });
});
```

You should setup the gateway handler by calling `asyncDecrypt` at the top of the `before` block.
Notice that when testing on the fhEVM, a decryption is fulfilled usually 2 blocks after the request, while in mocked mode the fulfilment will always happen as soon as you call the `awaitAllDecryptionResults` helper function. A good way to standardize hardhat tests is hence to always call`awaitAllDecryptionResults` which will ensure that all pending decryptions are fulfilled in both modes.

## Reencrypt

Reencryption is performed on the client side by calling the gateway service using the [fhevmjs](https://github.com/zama-ai/fhevmjs/) library. To do this, you need to provide a view function that returns the ciphertext to be reencrypted.

1. The dApp retrieves the ciphertext from the view function (e.g., balanceOf).
2. The dApp generates a keypair for the user and requests the user to sign the public key.
3. The dApp calls the gateway, providing the ciphertext, public key, user address, contract address, and the user's signature.
4. The dApp decrypts the received value with the private key.

You can read [our guide explaining how to use it](../guides/reencryption.md).
