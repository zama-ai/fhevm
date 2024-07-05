# Decrypt

The decryption operation is asynchronous. To use it, your contract must extend the `GatewayCaller` contract. This will import automatically the `Gateway` solidity library as well. See the following example:

```solidity
pragma solidity ^0.8.24;

import "fhevm/lib/TFHE.sol";
import "fhevm/gateway/GatewayCaller.sol";

contract TestAsyncDecrypt is GatewayCaller {
  ebool xBool;
  bool public yBool;

  constructor() {
      xBool = TFHE.asEbool(true);
      TFHE.allow(xBool, address(this));
  }

  function requestBool() public {
    uint256[] memory cts = new uint256[](1);
    cts[0] = Gateway.toUint256(xBool);
    Gateway.requestDecryption(cts, this.myCustomCallback.selector, 0, block.timestamp + 100, false);
  }

  function myCustomCallback(uint256 /*requestID*/, bool decryptedInput) public onlyGateway returns (bool) {
    yBool = decryptedInput;
    return yBool;
  }
```

Note that a [`GatewayContract`](../../gateway/GatewayContract.sol) contract is already predeployed on the fhEVM testnet, and a default relayer account is added through the specification of the environment variable `PRIVATE_KEY_GATEWAY_RELAYER` in the `.env` file. Relayers are the only accounts authorized to fulfil the decryption requests. However `GatewayContract` would still check the KMS signature during the fulfilment, so we trust the relayer only to forward the request on time, a rogue relayer could not cheat by sending fake decryption results.

The interface of the `Gateway.requestDecryption` function from previous snippet is the following:

```solidity
function requestDecryption(
    uint256[] memory ct,
    bytes4 callbackSelector,
    uint256 msgValue,
    uint256 maxTimestamp,
    bool passSignaturesToCaller
) returns(uint256 requestID)
```

The first argument, `ct`, should be an array of ciphertexts handles which could be of different types, i.e `uint256` values coming from unwrapping handles of type either `ebool`, `euint4`, `euint8`, `euint16`, `euint32`, `euint64` or `eaddress`. `ct` is the list of ciphertexts that are requested to be decrypted. Calling `requestDecryption` will emit an `EventDecryption` on the `GatewayContract` contract which will be detected by a relayer. Then, the relayer will send the corresponding ciphertexts to the KMS for decryption before fulfilling the request.

`callbackSelector` is the function selector of the callback function which will be called by the `GatewayContract` contract once the relayer fulfils the decryption request. Notice that the callback function should always follow this convention, if `passSignaturesToCaller` is set to `false`:

```solidity
function [callbackName](uint256 requestID, XXX x_0, XXX x_1, ..., XXX x_N-1) external onlyGateway
```

Or, alternatively, if `passSignaturesToCaller` is set to `true`:

```solidity
function [callbackName](uint256 requestID, XXX x_0, XXX x_1, ..., XXX x_N-1, bytes[] memory signatures) external onlyGateway
```

Notice that `XXX` should be the decrypted type, which is a native Solidity type corresponding to the original ciphertext type, following this table of conventions:

| Ciphertext type | Decrypted type |
| --------------- | -------------- |
| ebool           | bool           |
| euint4          | uint8          |
| euint8          | uint8          |
| euint16         | uint16         |
| euint32         | uint32         |
| euint64         | uint64         |
| eaddress        | address        |

Here `callbackName` is a custom name given by the developer to the callback function, `requestID` will be the request id of the decryption (could be commented if not needed in the logic, but must be present) and `x_0`, `x_1`, ... `x_N-1` are the results of the decryption of the `ct` array values, i.e their number should be the size of the `ct` array.

`msgValue` is the value in native tokens to be sent to the calling contract during fulfilment, i.e when the callback will be called with the results of decryption.

`maxTimestamp` is the maximum timestamp after which the callback will not be able to receive the results of decryption, i.e the fulfilment transaction will fail in this case. This can be used for time-sensitive applications, where we prefer to reject decryption results on too old, out-of-date, values.

`passSignaturesToCaller` determines whether the callback needs to transmit signatures from the KMS or not. This is useful if the dApp developer wants to remove trust from the Gateway service and prefers to check the KMS signatures directly from within his dApp smart contract. A concrete example of how to verify the KMS signatures inside a dApp is available [here](../../examples/TestAsyncDecrypt.sol#L82-L94) in the `requestBoolTrustless` function.

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

function addParamsUint256(uint256 requestID, uint256 _uint) internal;
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

function getParamsUint256(uint256 requestID) internal;
```

For example, see this snippet where we add two `uint256`s during the request call, to make them available later during the callback:

```solidity
pragma solidity ^0.8.24;

import "../lib/TFHE.sol";
import "../gateway/GatewayCaller.sol";

contract TestAsyncDecrypt is GatewayCaller {
  euint32 xUint32;
  uint32 public yUint32;

  constructor() {
      xUint32 = TFHE.asEuint32(32);
      TFHE.allow(xUint32, address(this));
  }

  function requestUint32(uint32 input1, uint32 input2) public {
      uint256[] memory cts = new uint256[](1);
      cts[0] = Gateway.toUint256(xUint32);
      uint256 requestID = Gateway.requestDecryption(cts, this.callbackUint32.selector, 0, block.timestamp + 100, false);
      addParamsUint256(requestID, input1);
      addParamsUint256(requestID, input2);
  }

  function callbackUint32(uint256 requestID, uint32 decryptedInput) public onlyGateway returns (uint32) {
    uint256[] memory params = getParamsUint256(requestID);
    unchecked {
        uint32 result = uint32(params[0]) + uint32(params[1]) + decryptedInput;
        yUint32 = result;
        return result;
    }
}
```

When the decryption request is fufilled by the relayer, the `GatewayContract` contract, when calling the callback function, will also emit the following event:

```solidity
event ResultCallback(uint256 indexed requestID, bool success, bytes result);
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
