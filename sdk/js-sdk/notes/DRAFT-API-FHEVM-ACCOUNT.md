```ts
const userDecryptionPermit: UserDecryptionPermit;

const fhevmClient = createFhevmClient({ chain: mainnet });
const fhevmUserClient = createFhevmUserClient({
  chain: mainnet,
  privateKey: tkmsPrivateKey | tkmsPrivateKeyBytes,
  address: userAddress,
});
const fhevmUserClient = createFhevmUserClient({
  chain: mainnet,
  privateKey: tkmsPrivateKey | tkmsPrivateKeyBytes,
  address: userAddress,
  permit: userDecryptionPermit,
});

usePermit(fhevmUserClient, permit);

await decrypt(fhevmUserClient, { handleContractPairs, options });
```

```ts
export type FhevmHostClient;
export type FhevmEncryptionClient = FhevmHostClient & { tfhe: TfheActions, relayer: RelayerActions };

export type FhevmDecryptionClient = FhevmHostClient & { tkms: TkmsActions, relayer: RelayerActions };
export type FhevmClient = FhevmHostClient & FhevmDecryptionClient & FhevmEncryptionClient;

export type FhevmUserDecryptionClient = FhevmHostClient & { tkms: TkmsUserActions, relayer: RelayerActions };
export type FhevmUserClient = FhevmHostClient & FhevmUserDecryptionClient & FhevmEncryptionClient;

export function createFhevmHostClient(): FhevmHostClient;
export function createFhevmDecryptionClient(): Promise<FhevmDecryptionClient>;
export function createFhevmEncryptionClient(): Promise<FhevmEncryptionClient>;

export function checkUserAllowedForDecryption(fhevmHostClient: FhevmHostClient, parameters: { userAddress: string, handle: string });
export function checkUserAllowedForDecryption(fhevmHostClient: FhevmHostClient, parameters: { userAddress: string, pair: { handle: string, contractAddress: string } });
export function checkUserAllowedForDecryption(fhevmHostClient: FhevmHostClient, parameters: { userAddress: string, pairs: { handle: string, contractAddress: string }[] });

```

```ts
export type FhevmHostClient = {
  readonly chain: ResolvedFhevmChain;
  readonly host: HostActions;
  readonly ethereum: EthereumActions;
  readonly options: {
    batchRpcCalls?: boolean;
  };
};

export type WithEncryption = { tfhe: TfheActions };
export type WithDecryption = { tkms: TkmsActions };
export type WithUserDecryption = {
  tkms: TkmsUserActions;
  userAddress: ChecksummedAddress;
};
export type WithRelayer = { relayer: RelayerActions };

// Full client
export function createFhevmClient(
  config: FhevmClientConfig,
): Promise<FhevmClient>;

// Add missing capabilities to existing FhevmHostClient
export function extendWithDecryption<T extends FhevmHostClient>(
  fhevmClient: T,
): Promise<T & WithDecryption>;
export function extendWithEncryption<T extends FhevmHostClient>(
  fhevmClient: T,
): Promise<T & WithEncryption>;
export function extendWithRelayer<T extends FhevmHostClient>(
  fhevmClient: T,
): T & WithRelayer;

// Full user client
export function createFhevmUserClient(
  config: FhevmClientConfig,
  parameters: FhevmUserClientParameters,
): Promise<FhevmUserClient>;

// Add user capabilities
export function extendWithUser<T extends FhevmHostClient>(
  fhevmClient: T,
  parameters: FhevmUserDecryptionParameters,
): Promise<T & WithUserDecryption>;

////////////////////////////////////////////////////////////////////////////////

export type FhevmClient = FhevmHostClient &
  WithDecryption &
  WithEncryption &
  WithRelayer;

export type FhevmRelayerClient = FhevmHostClient & WithRelayer;
export type FhevmEncryptionClient = FhevmHostClient & WithEncryption;
export type FhevmDecryptionClient = FhevmHostClient & WithDecryption;

export type FhevmUserDecryptionClient = FhevmHostClient & WithUserDecryption;
export type FhevmUserClient = FhevmUserDecryptionClient &
  WithEncryption &
  WithRelayer;

////////////////////////////////////////////////////////////////////////////////

export function fooAction1(
  fhevmClient: FhevmHostClient,
  parameters: FooAction1Parameters,
): FooAction1ReturnType;
export function fooAction2(
  fhevmClient: FhevmEncryptionClient,
  parameters: FooAction2Parameters,
): FooAction2ReturnType;
export function fooAction3(
  fhevmClient: FhevmEncryptionClient & WithRelayer,
  parameters: FooAction3Parameters,
): FooAction3ReturnType;

////////////////////////////////////////////////////////////////////////////////
// User (to hide privateKey)

export function fooUserAction1(
  fhevmUserClient: FhevmUserDecryptionClient,
  parameters: FooUserAction1Parameters,
): FooUserAction1ReturnType;
export function fooUserAction2(
  fhevmUserClient: FhevmUserDecryptionClient & WithRelayer,
  parameters: FooUserAction1Parameters,
): FooUserAction2ReturnType;
export function fooUserAction3(
  fhevmUserClient: FhevmUserDecryptionClient & WithEncryption & WithRelayer,
  parameters: FooUserAction3Parameters,
): FooUserAction3ReturnType;

////////////////////////////////////////////////////////////////////////////////
```
