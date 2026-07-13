export type FhevmAddressesV12 = {
  readonly aclAddress: string;
  readonly fhevmExecutorAddress: string;
  readonly kmsVerifierAddress: string;
  readonly inputVerifierAddress: string;
  readonly hcuLimitAddress: string;
};

export type FhevmAddressesV13 = FhevmAddressesV12 & {
  readonly protocolConfigAddress: string;
  readonly kmsGenerationAddress: string;
};

/**
 * Cleartext-only infrastructure addresses (test stack): the arithmetic/persistence contract and the
 * shared cleartext store. Kept separate from `FhevmAddressesV13` so the real host address set stays clean.
 */
export type CleartextAddresses = {
  readonly cleartextArithmeticAddress: string;
  readonly cleartextDbAddress: string;
};

export type EncodeCallParameters = {
  readonly abi: readonly unknown[];
  readonly functionName: string;
  readonly args?: readonly unknown[];
};

export type DeployParameters = {
  readonly abi?: readonly unknown[];
  readonly bytecode: string;
  readonly args?: readonly unknown[];
};

export interface AbstractEthereumUtils {
  getContractAddress(parameters: { readonly from: string; readonly nonce: bigint }): `0x${string}`;

  // Pure ABI encoding. No signer/caller/msg.sender.
  encodeCall(parameters: EncodeCallParameters): Promise<`0x${string}`>;
}

export interface AbstractEthereumProvider {
  // Privileged dev-node RPC operation, not signer-based.
  setCodeAt(parameters: { readonly address: string; readonly bytecode: string }): Promise<void>;

  // Pure ABI encoding. No signer/caller/msg.sender.
  getCodeAt(parameters: { readonly address: string }): Promise<string>;
}

export type DeployReturnType = { contractAddress: string };

export interface AbstractEthereumSigner {
  // Signer/account address. Used as msg.sender-equivalent ownership input where deployment calldata needs it.
  getAddress(): Promise<string>;

  // Signer/account-based transaction. Deployer is msg.sender in constructor.
  deploy(parameters: DeployParameters): Promise<DeployReturnType>;

  // Signer/account-based transaction. msg.sender is the signer/account.
  writeContract(parameters: unknown): Promise<unknown>;
}
