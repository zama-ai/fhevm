// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity ^0.8.24;

// The `fhevm-config-1.0.0/addresses.sol` of the v1.0 cleartext host contracts, imported (relatively) by
// ../host-contracts/addresses/FHEVMHostAddresses.sol. Same shape as the file fhevm-mocks'
// pkg/forge/script/ComputeAddresses.s.sol writes. See ../DEPLOY.md.
//
// Real addresses, not placeholder markers. Valid ONLY for this deployer at this start nonce:
//   deployer    0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4
//               (mnemonic "adapt mosquito ... father hope layer", m/44'/60'/0'/0/5)
//   start nonce 0
// Every address is CREATE(deployer, startNonce + offset); regenerate this file (and rebuild) for any
// other deployer or start nonce.

address constant ACL_ADDRESS = address(0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D); // nonce 1

address constant FHEVM_EXECUTOR_ADDRESS = address(0xe3a9105a3a932253A70F126eb1E3b589C643dD24); // nonce 3

address constant KMS_VERIFIER_ADDRESS = address(0x901F8942346f7AB3a01F6D7613119Bca447Bb030); // nonce 4

address constant INPUT_VERIFIER_ADDRESS = address(0x36772142b74871f255CbD7A3e89B401d3e45825f); // nonce 5

address constant HCU_LIMIT_ADDRESS = address(0x233ff88A48c172d29F675403e6A8e302b0F032D9); // nonce 6

address constant PROTOCOL_CONFIG_ADDRESS = address(0x44aA028fd264C76BF4A8f8B4d8A5272f6AE25CAc); // nonce 7

address constant KMS_GENERATION_ADDRESS = address(0x216be43148dB537BeddBC268163deb1a802b5553); // nonce 8

address constant PADDED_INPUTTER_ADDRESS = address(0xded0D2a71268DC12622BdD1b55d68a1CB5662327); // nonce 9

address constant CLEARTEXT_ARITHMETIC_ADDRESS = address(0x6933Afcf0F4bCE1A611baD0A6FaafF0337a7ba1E); // nonce 10

address constant CLEARTEXT_DB_ADDRESS = address(0x590e3330386Fa042843773541aaBb3a45EC3164D); // nonce 11

address constant PAUSER_SET_ADDRESS = address(0xcF5De2DbC4Ea85426db47845537A90ec4e6316E2); // nonce 12

address constant PROTOCOL_CONFIG_REPLICA_ADDRESS = address(0x492e52843AE2eCF9daae6B951E12C3614Ee115C1); // nonce 13
