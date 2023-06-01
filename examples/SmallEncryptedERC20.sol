// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

import '../node_modules/@openzeppelin/contracts/utils/cryptography/ECDSA.sol';
import '../node_modules/@openzeppelin/contracts/utils/cryptography/EIP712.sol';
import '../lib/TFHE.sol';

contract SmallEncryptedERC20 is EIP712 {
  euint8 public totalSupply;
  string public name = 'Naraggara'; // City of Zama's battle
  string public symbol = 'NARA';
  uint8 public decimals = 18;

  // A mapping from address to an encrypted balance.
  mapping(address => euint8) internal balances;

  // A mapping of the form mapping(owner => mapping(spender => allowance)).
  mapping(address => mapping(address => euint8)) internal allowances;

  // The owner of the contract.
  address internal contractOwner;

  constructor() EIP712('Authorization token', '1') {
    contractOwner = msg.sender;
  }

  // Sets the balance of the owner to the given encrypted balance.
  function mint(bytes calldata encryptedAmount) public onlyContractOwner {
    euint8 amount = TFHE.asEuint8(encryptedAmount);
    balances[contractOwner] = amount;
    totalSupply = TFHE.add(totalSupply, amount);
  }

  // Transfers an encrypted amount from the message sender address to the `to` address.
  function transfer(address to, bytes calldata encryptedAmount) public {
    transfer(to, TFHE.asEuint8(encryptedAmount));
  }

  // Transfers an amount from the message sender address to the `to` address.
  function transfer(address to, euint8 amount) internal {
    _transfer(msg.sender, to, amount);
  }

  function getTotalSupply(
    bytes32 publicKey,
    bytes calldata signature
  ) public view onlyContractOwner onlySignedPublicKey(signature, publicKey) returns (bytes memory) {
    return TFHE.reencrypt(totalSupply, publicKey);
  }

  // Returns the balance of the caller under their public FHE key.
  // The FHE public key is automatically determined based on the origin of the call.
  function balanceOf(
    bytes32 publicKey,
    bytes calldata signature
  ) public view onlySignedPublicKey(signature, publicKey) returns (bytes memory) {
    return TFHE.reencrypt(balances[signer], publicKey);
  }

  // Sets the `encryptedAmount` as the allowance of `spender` over the caller's tokens.
  function approve(address spender, bytes calldata encryptedAmount) public {
    address owner = msg.sender;
    _approve(owner, spender, TFHE.asEuint8(encryptedAmount));
  }

  // Returns the remaining number of tokens that `spender` is allowed to spend
  // on behalf of the caller. The returned ciphertext is under the caller public FHE key.
  function allowance(
    address spender,
    bytes32 publicKey,
    bytes calldata signature
  ) public view onlySignedPublicKey(signature, publicKey) returns (bytes memory) {
    address owner = msg.sender;
    return TFHE.reencrypt(_allowance(owner, spender), publicKey);
  }

  // Transfers `encryptedAmount` tokens using the caller's allowance.
  function transferFrom(address from, address to, bytes calldata encryptedAmount) public {
    transferFrom(from, to, TFHE.asEuint8(encryptedAmount));
  }

  // Transfers `amount` tokens using the caller's allowance.
  function transferFrom(address from, address to, euint8 amount) public {
    address spender = msg.sender;
    _updateAllowance(from, spender, amount);
    _transfer(from, to, amount);
  }

  function _approve(address owner, address spender, euint8 amount) internal {
    allowances[owner][spender] = amount;
  }

  function _allowance(address owner, address spender) internal view returns (euint8) {
    return allowances[owner][spender];
  }

  function _updateAllowance(address owner, address spender, euint8 amount) internal {
    euint8 currentAllowance = _allowance(owner, spender);
    TFHE.requireCt(TFHE.lte(amount, currentAllowance));
    _approve(owner, spender, TFHE.sub(currentAllowance, amount));
  }

  // Transfers an encrypted amount.
  function _transfer(address from, address to, euint8 amount) internal {
    // Make sure the sender has enough tokens.
    TFHE.requireCt(TFHE.lte(amount, balances[from]));

    // Add to the balance of `to` and subract from the balance of `from`.
    balances[to] = TFHE.add(balances[to], amount);
    balances[from] = TFHE.sub(balances[from], amount);
  }

  modifier onlyContractOwner() {
    require(msg.sender == contractOwner);
    _;
  }

  modifier onlySignedPublicKey(bytes memory signature, bytes32 publicKey) {
    bytes32 digest = _hashTypedDataV4(keccak256(abi.encode(keccak256('Reencrypt(bytes32 publicKey)'), publicKey)));
    address signer = ECDSA.recover(digest, signature);
    require(signer == msg.sender, 'Invalid EIP712 signature');
    _;
  }
}
