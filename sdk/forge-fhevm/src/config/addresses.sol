// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

// Where the cleartext host stack lives.
//
// `host-contracts-cleartext` ships this file as PLACEHOLDERS behind the `fhevm-config-0.14.0/` remapping,
// precisely so a consumer can substitute its own. We do. This file is what that remapping resolves to (see
// remappings.txt), so every host contract compiles with the addresses below baked into its bytecode, and
// `FhevmTest` then deploys them at exactly those addresses.
//
// The first three are NOT free choices. A contract under test inherits `ZamaEthereumConfig`, whose
// `ZamaConfig._getLocalConfig()` hardcodes them for `block.chainid == 31337`. They are compiled into the
// contract under test, so `FHE.*` calls go to these addresses no matter what we do. Get them wrong and
// every FHE call lands on an empty account.
//
// The rest are free: nothing outside the host stack refers to them. They are the values the sibling package
// uses, kept identical so bytecode comparisons against it stay meaningful.

// Pinned by ZamaConfig._getLocalConfig() — do not change.
address constant ACL_ADDRESS = address(0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D);
address constant FHEVM_EXECUTOR_ADDRESS = address(0xe3a9105a3a932253A70F126eb1E3b589C643dD24); // "CoprocessorAddress"
address constant KMS_VERIFIER_ADDRESS = address(0x901F8942346f7AB3a01F6D7613119Bca447Bb030);

// Free choices — referenced only by the host stack itself.
address constant INPUT_VERIFIER_ADDRESS = address(0x36772142b74871f255CbD7A3e89B401d3e45825f);
address constant HCU_LIMIT_ADDRESS = address(0x233ff88A48c172d29F675403e6A8e302b0F032D9);
address constant PROTOCOL_CONFIG_ADDRESS = address(0x44aA028fd264C76BF4A8f8B4d8A5272f6AE25CAc);
address constant KMS_GENERATION_ADDRESS = address(0x216be43148dB537BeddBC268163deb1a802b5553);
address constant PAUSER_SET_ADDRESS = address(0xded0D2a71268DC12622BdD1b55d68a1CB5662327);

/// @dev ConfidentialBridge is never deployed in the cleartext stack, but `ACL` bakes the address in
///      (it gates `allowTransient`), so the constant must exist.
address constant CONFIDENTIAL_BRIDGE_ADDRESS = address(0x812b06e1CDCE800494b79fFE4f925A504a9A9810);

address constant CLEARTEXT_ARITHMETIC_ADDRESS = address(0x7071727374757677787980818283848586878889);
address constant CLEARTEXT_DB_ADDRESS = address(0x8081828384858687888990919293949596979899);
