import {
  CLEARTEXT_DECRYPTION_ADDRESS,
  CLEARTEXT_GATEWAY_CHAIN_ID,
  CLEARTEXT_INPUT_VERIFICATION_ADDRESS,
} from '@fhevm/host-contracts-cleartext/ts';

// npmjs.com packages url:
// =======================

// https://www.npmjs.com/package/@fhevm/solidity?activeTab=versions
// https://www.npmjs.com/package/@fhevm/sdk?activeTab=versions
// https://www.npmjs.com/package/@fhevm/host-contracts-cleartext?activeTab=versions

const constants = {
  DECRYPTION_ADDRESS: '0x5ffdaAB0373E62E2ea2944776209aEf29E631A64',
  INPUT_VERIFICATION_ADDRESS: '0x812b06e1CDCE800494b79fFE4f925A504a9A9810',
  FHEVM_HANDLE_VERSION: 0,
  HARDHAT_PLUGIN_NAME: '@fhevm/hardhat-plugin',
  SOLIDITY_COVERAGE_PACKAGE_NAME: 'solidity-coverage',
  TRACE_DECRYPTION_REQUEST_EVENTS: false,
  DEVELOPMENT_NETWORK_CHAINID: 31337,
  // https://www.npmjs.com/package/@fhevm/solidity?activeTab=versions
  // @fhevm/solidity@0.13.3
  FHEVM_SOLIDITY_PACKAGE: {
    version: '0.13.3',
    name: '@fhevm/solidity',
    configFile: 'config/ZamaConfig.sol',
    configContractName: 'EthereumConfig',
    // `EthereumConfig`, `SepoliaConfig`, `LocalConfig` must match the exact configuration
    // defined in `config/ZamaConfig.sol`.
    // It is essentially used to detect any mismatch with `config/ZamaConfig.sol`
    EthereumConfig: {
      ACLAddress: '0xcA2E8f1F656CD25C01F05d0b243Ab1ecd4a8ffb6',
      CoprocessorAddress: '0xD82385dADa1ae3E969447f20A3164F6213100e75',
      KMSVerifierAddress: '0x77627828a55156b04Ac0DC0eb30467f1a552BB03',
    },
    SepoliaConfig: {
      ACLAddress: '0xf0Ffdc93b7E186bC2f8CB3dAA75D86d1930A433D',
      CoprocessorAddress: '0x92C920834Ec8941d2C77D188936E1f7A6f49c127',
      KMSVerifierAddress: '0xbE0E383937d564D7FF0BC3b46c51f0bF8d5C311A',
    },
    LocalConfig: {
      ACLAddress: '0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D',
      CoprocessorAddress: '0xe3a9105a3a932253A70F126eb1E3b589C643dD24',
      KMSVerifierAddress: '0x901F8942346f7AB3a01F6D7613119Bca447Bb030',
    },
  },
  // @fhevm/host-contracts-cleartext@0.13.0
  //
  // The canonical localhost cleartext stack, mirroring the package's generated
  // `pkg/forge/src/_internal/LocalHostAddresses.sol`. Every address below is
  // `CREATE(deployerAddress, nonce)` — deploying from any other account, or from a non-zero start
  // nonce, moves the whole stack while the implementations' baked-in addresses stay put.
  //
  // `ACLAddress`, `CoprocessorAddress` and `KMSVerifierAddress` are exactly the three values
  // `@fhevm/solidity/config/ZamaConfig.sol` compiles into every dApp inheriting its local config
  // (see FHEVM_SOLIDITY_PACKAGE.LocalConfig below) — which is why none of this may drift.
  FHEVM_HOST_CONTRACTS_CLEARTEXT_PACKAGE: {
    version: '0.13.0',
    name: '@fhevm/host-contracts-cleartext',
    // BIP-39 mnemonic the local stack is *deployed* from. This is NOT the signer mnemonic: the KMS
    // and coprocessor signing keys are derived and owned by `@fhevm/sdk`. Two mnemonics, two jobs.
    mnemonic: 'adapt mosquito move limb mobile illegal tree voyage juice mosquito burger raise father hope layer',
    deployerAddressIndex: 5,
    deployerPath: "m/44'/60'/0'/0/5",
    deployerAddress: '0x8B8f5091f8b9817EF69cFC1E8B2f721BafF60DF4',
    deployerStartNonce: 0,
    // nonces 1, 3, 4, 5, 6, 7, 8
    fhevmAddresses: {
      aclAddress: '0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D',
      fhevmExecutorAddress: '0xe3a9105a3a932253A70F126eb1E3b589C643dD24',
      kmsVerifierAddress: '0x901F8942346f7AB3a01F6D7613119Bca447Bb030',
      inputVerifierAddress: '0x36772142b74871f255CbD7A3e89B401d3e45825f',
      hcuLimitAddress: '0x233ff88A48c172d29F675403e6A8e302b0F032D9',
      protocolConfigAddress: '0x44aA028fd264C76BF4A8f8B4d8A5272f6AE25CAc',
      kmsGenerationAddress: '0x216be43148dB537BeddBC268163deb1a802b5553',
    },
    // nonces 9, 10
    cleartextAddresses: {
      cleartextArithmeticAddress: '0xded0D2a71268DC12622BdD1b55d68a1CB5662327',
      cleartextDbAddress: '0x6933Afcf0F4bCE1A611baD0A6FaafF0337a7ba1E',
    },
    // nonce 11
    pauserSetAddress: '0x590e3330386Fa042843773541aaBb3a45EC3164D',
    // The gateway the stack is bootstrapped against. IMPORTED, never transcribed: these three feed the
    // EIP-712 domain the coprocessor signatures are recovered against, so a stale copy does not fail
    // loudly — `ecrecover` just returns a different junk address on every run. That is exactly what a
    // hardcoded `chainId: 654321` did here while the stack had moved to 100733346448153.
    gateway: {
      chainId: Number(CLEARTEXT_GATEWAY_CHAIN_ID),
      decryptionAddress: CLEARTEXT_DECRYPTION_ADDRESS,
      inputVerificationAddress: CLEARTEXT_INPUT_VERIFICATION_ADDRESS,
    },
  },
  // https://www.npmjs.com/package/@fhevm/sdk?activeTab=versions
  // @fhevm/sdk@0.13.3
  FHEVM_SDK_PACKAGE: {
    version: '0.13.3',
    name: '@fhevm/sdk',
  },
};
Object.freeze(constants);
Object.freeze(constants.FHEVM_HOST_CONTRACTS_CLEARTEXT_PACKAGE);
Object.freeze(constants.FHEVM_HOST_CONTRACTS_CLEARTEXT_PACKAGE.fhevmAddresses);
Object.freeze(constants.FHEVM_HOST_CONTRACTS_CLEARTEXT_PACKAGE.cleartextAddresses);
Object.freeze(constants.FHEVM_HOST_CONTRACTS_CLEARTEXT_PACKAGE.gateway);
Object.freeze(constants.FHEVM_SDK_PACKAGE);
Object.freeze(constants.FHEVM_SOLIDITY_PACKAGE);
Object.freeze(constants.FHEVM_SOLIDITY_PACKAGE.SepoliaConfig);
Object.freeze(constants.FHEVM_SOLIDITY_PACKAGE.EthereumConfig);
Object.freeze(constants.FHEVM_SOLIDITY_PACKAGE.LocalConfig);

export default constants;
