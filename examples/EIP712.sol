// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

import '@openzeppelin/contracts/utils/cryptography/ECDSA.sol';
import '@openzeppelin/contracts/utils/cryptography/EIP712.sol';

contract AuthorizationToken is EIP712 {
  constructor() EIP712('Authorization token', '1') {}

  function verify(
    bytes32 publicKey,
    bytes calldata signature
  ) public view onlySignedPublicKey(signature, publicKey) returns (bool) {
    return true;
  }

  modifier onlySignedPublicKey(bytes memory signature, bytes32 publicKey) {
    bytes32 digest = _hashTypedDataV4(keccak256(abi.encode(keccak256('Reencrypt(bytes32 publicKey)'), publicKey)));
    address signer = ECDSA.recover(digest, signature);
    require(signer == msg.sender, 'Invalid EIP712 signature');
    _;
  }
}
