// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

address constant ACL_ADDRESS = address(0x1011121314151617181920212223242526272829);
address constant FHEVM_EXECUTOR_ADDRESS = address(0x2021222324252627282930313233343536373839);
address constant KMS_VERIFIER_ADDRESS = address(0x3031323334353637383940414243444546474849);
address constant INPUT_VERIFIER_ADDRESS = address(0x4041424344454647484950515253545556575859);
address constant HCU_LIMIT_ADDRESS = address(0x5051525354555657585960616263646566676869);
address constant PROTOCOL_CONFIG_ADDRESS = address(0x44aA028fd264C76BF4A8f8B4d8A5272f6AE25CAc);
address constant KMS_GENERATION_ADDRESS = address(0x216be43148dB537BeddBC268163deb1a802b5553);
address constant PAUSER_SET_ADDRESS = address(0xded0D2a71268DC12622BdD1b55d68a1CB5662327);
/// @dev v0.14 only. ConfidentialBridge is not deployed in the cleartext stack, but `ACL` bakes the
///      address in (it gates `allowTransient`), so the constant must exist. Defaults to the canonical
///      v0.14 address; still templated, so a consumer running against a real bridge can patch it.
address constant CONFIDENTIAL_BRIDGE_ADDRESS = address(0x812b06e1CDCE800494b79fFE4f925A504a9A9810);
address constant CLEARTEXT_ARITHMETIC_ADDRESS = address(0x7071727374757677787980818283848586878889);
address constant CLEARTEXT_DB_ADDRESS = address(0x8081828384858687888990919293949596979899);
