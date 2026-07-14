/**
 * Where the cleartext stack is placed.
 *
 * The first three are NOT free choices. `ZamaConfig._getLocalConfig()`
 * (`library-solidity/config/ZamaConfig.sol`, the `block.chainid == 31337` branch) hardcodes them, and that
 * config is compiled INTO the user's contracts. So the mock must put ACL / FHEVMExecutor / KMSVerifier at
 * exactly these addresses, or `FHE.*` calls in user code reach empty accounts.
 *
 * That constraint is the whole reason the engine places code with `setCode` rather than using
 * host-contracts-cleartext's own `deploy()` — see the note in `deploy.ts`.
 *
 * The rest are free. Nothing outside the host stack references them: the host contracts find each other
 * through the constants in `host-contracts-cleartext/config/addresses.sol`, which appear in the compiled
 * bytecode as placeholders that we rewrite to the values below (see `artifacts.ts`). Any consistent set
 * works.
 */
export const ADDRESSES = {
  // Pinned by ZamaConfig._getLocalConfig() — do not change.
  ACL: "0x50157CFfD6bBFA2DECe204a89ec419c23ef5755D",
  FHEVMExecutor: "0xe3a9105a3a932253A70F126eb1E3b589C643dD24", // ZamaConfig calls this "CoprocessorAddress"
  KMSVerifier: "0x901F8942346f7AB3a01F6D7613119Bca447Bb030",
  // Free choices — patched into the bytecode, referenced only by the host stack itself.
  InputVerifier: "0x36772142b74871f255CbD7A3e89B401d3e45825f",
  HCULimit: "0x233ff88A48c172d29F675403e6A8e302b0F032D9",
  ProtocolConfig: "0x44aA028fd264C76BF4A8f8B4d8A5272f6AE25CAc",
  KMSGeneration: "0x216be43148dB537BeddBC268163deb1a802b5553",
  PauserSet: "0xded0D2a71268DC12622BdD1b55d68a1CB5662327",
  CleartextArithmetic: "0x7071727374757677787980818283848586878889",
  CleartextDB: "0x8081828384858687888990919293949596979899",
} as const;

export type HostContractName = keyof typeof ADDRESSES;

/**
 * ConfidentialBridge is never deployed in the cleartext stack, but `ACL` bakes its address in (it gates
 * `allowTransient`), so the constant has to resolve to something. It is a reference, not a contract we
 * place — hence it lives here rather than in {@link ADDRESSES}.
 */
export const CONFIDENTIAL_BRIDGE = "0x812b06e1CDCE800494b79fFE4f925A504a9A9810";

/**
 * Maps each `addressReferences` key in the templates to the address we place that contract at. Every
 * reference in every template is rewritten: the templates ship placeholder values (`0x1011...2829`, etc.)
 * that are meaningless until patched.
 */
export const ADDRESS_REFERENCES: Readonly<Record<string, string>> = {
  CONFIDENTIAL_BRIDGE_ADDRESS: CONFIDENTIAL_BRIDGE,
  ACL_ADDRESS: ADDRESSES.ACL,
  FHEVM_EXECUTOR_ADDRESS: ADDRESSES.FHEVMExecutor,
  KMS_VERIFIER_ADDRESS: ADDRESSES.KMSVerifier,
  INPUT_VERIFIER_ADDRESS: ADDRESSES.InputVerifier,
  HCU_LIMIT_ADDRESS: ADDRESSES.HCULimit,
  PROTOCOL_CONFIG_ADDRESS: ADDRESSES.ProtocolConfig,
  KMS_GENERATION_ADDRESS: ADDRESSES.KMSGeneration,
  PAUSER_SET_ADDRESS: ADDRESSES.PauserSet,
  CLEARTEXT_ARITHMETIC_ADDRESS: ADDRESSES.CleartextArithmetic,
  CLEARTEXT_DB_ADDRESS: ADDRESSES.CleartextDB,
};
