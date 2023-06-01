// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

import '../node_modules/@openzeppelin/contracts/utils/cryptography/ECDSA.sol';
import '../node_modules/@openzeppelin/contracts/utils/cryptography/EIP712.sol';
import '../lib/TFHE.sol';

// Shows the CMUX operation in Solidity.
contract CMUX is EIP712 {
  euint8 internal result;

  constructor() EIP712('Authorization token', '1') {}

  // Set result = (ifTrue - ifFalse) * control + ifFalse
  function cmux(bytes calldata controlBytes, bytes calldata ifTrueBytes, bytes calldata ifFalseBytes) public {
    euint8 control = TFHE.asEuint8(controlBytes);
    euint8 ifTrue = TFHE.asEuint8(ifTrueBytes);
    euint8 ifFalse = TFHE.asEuint8(ifFalseBytes);
    result = TFHE.cmux(control, ifTrue, ifFalse);
  }

  function getResult(
    bytes32 publicKey,
    bytes calldata signature
  ) public view onlyContractOwner onlySignedPublicKey(signature, publicKey) returns (bytes memory) {
    return TFHE.reencrypt(result, publicKey);
  }

  modifier onlySignedPublicKey(bytes memory signature, bytes32 publicKey) {
    bytes32 digest = _hashTypedDataV4(keccak256(abi.encode(keccak256('Reencrypt(bytes32 publicKey)'), publicKey)));
    address signer = ECDSA.recover(digest, signature);
    require(signer == msg.sender, 'Invalid EIP712 signature');
    _;
  }
}
