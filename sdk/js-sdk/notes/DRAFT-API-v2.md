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
  userTkms: TkmsUserActions;
  userAddress: ChecksummedAddress;
};
export type WithRelayer = { relayer: RelayerActions };

// Minimal client (lightweight - sync)
export function createFhevmHostClient(
  config: FhevmClientConfig,
): FhevmHostClient;

// Can pre-init heavy WASM if needed so future extendWith will be fast
export async function initializeDecryptionModule();
export async function initializeEncryptionModule();

// Full client (CPU intensive the first time)
export function createFhevmClient(
  config: FhevmClientConfig,
): Promise<FhevmClient>;

// (Rarely used) Add missing capabilities to existing FhevmHostClient
// extendWith{Caps} should accept multiple calls - subsequent calls will have no effect
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
export function extendWithUserDecryption<T extends FhevmHostClient>(
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
  parameters: FooUserAction2Parameters,
): FooUserAction2ReturnType;
export function fooUserAction3(
  fhevmUserClient: FhevmUserDecryptionClient & WithEncryption & WithRelayer,
  parameters: FooUserAction3Parameters,
): FooUserAction3ReturnType;

// note: a FhevmUserClient can call functions that take a FhevmClient as argument
////////////////////////////////////////////////////////////////////////////////

export function fooDecrypt(
  fhevmClient: FhevmClient,
  privateKey: TkmsPrivateKey,
  userAddress: string,
  parameters: FooDecryptParameters,
);
export function fooUserDecrypt(
  fhevmUserClient: FhevmUserClient,
  parameters: FooDecryptParameters,
);

// Usage
const fhevmNoUserClient: FhevmClient;
const fhevmUserClient: FhevmUserClient;

fooDecrypt(fhevmNoUserClient, privateKey, userAddress, parameters);
fooUserDecrypt(fhevmNoUserClient, parameters); //Throws! Not possible

fooDecrypt(fhevmUserClient, privateKey, userAddress, parameters); // ok but weird ???
fooUserDecrypt(fhevmUserClient, parameters); //ok

////////////////////////////////////////////////////////////////////////////////
// Permit
// const walletClient = createWalletClient({ chain: mainnet, account // Eth privateKey });

// createClient();
// client.extent(walletActions)

// const client = createPublicClient();
// client.extend(walletActions)
// client.writeContract(...)

// walletClient.write(... no privateKey as arg) //do not expose the privateKey

const fhevmUserClient = createFhevmUserClient({
  chain: fhevmMainnet,
  userAddress,
  fhevmPrivateKeyBytes,
});

const permit = fhevmUserClient.signPermit(signer, parameters);
permit.toJSON();
const permit = parseJSON(permitStr)

// throw error invalid permit
// throw error permit expired
// throw error wrong signature etc.
fhevmUserClient.usePermit(permit);

const clearValue = await fhevmUserClient.userDecrypt({ handle, contractAddress });
userDecrypt(fhevmUserClient, { handle, contractAddress });

const clearValue = await fhevmUserClient.publicDecrypt({ handle });
publicDecrypt(fhevmUserClient, { handle });

// General purpose client, not bounded to a user
const fhevmClient = createFhevmClient();
const clearValue = await fhevmClient.publicDecrypt({ handle });
publicDecrypt(fhevmClient, { handle });
// Throw exception (missing private key)
userDecrypt(fhevmClient, { handle, contractAddress });
// OK
userDecrypt(fhevmClient, { privateKey, userAddress, handle, contractAddress });
// Lowlevel
userDecrypt(fhevmClient, { privateKey, ... });
// TODO


const fhevmHostClient = createFhevmHostClient({ chain: fhevmMainnet });
isAllowedForDecryption(fhevmHostClient, { handle }); // ACL call
isPersistAllowed(fhevmHostClient, { handle, user }); // ACL call
downloadZamaPublicKey(fhevmHostClient);

const fhevmDecryptionClient = await fhevmHostClient.withDecryption();

async function extendWithDecryption(fhevmHostClient: ): FhevmDecryptionClient | Promise<FhevmDecryptionClient> {
  fhevmHostclientextend(decryptionActions);
}

// await decryptionActions.preload(); + add the actions
// await fhevmHostClient.extendWithRelayer();
// await fhevmHostClient.extendWithEncryption();
// await fhevmHostClient.extendWithDecryption();

// import { relayerActions } from "..."
// fhevmHostClient.extend(relayerActions);
// fhevmHostClient.extend(encryptionActions);
// fhevmHostClient.extend(decryptionActions);

fhevmDecryptionClient.publicDecrypt(...)

// init TKMS (fast) - only once - 600KB (loading the WASM)
await initializeDecryptionModule();
// init TFHE (slower) - only once - 5MB
await initializeEncryptionModule();
await initializeZamaPublicKey(fhevmMainnet);

// 50MB
const bigZamaPublicKeyMain = await fetchZamaPublicKey({ chain: fhevmMainnet });
await fhevmMainnet.fetchZamaPublicKey();
await fetchZamaPublicKey(fhevmMainnet);

// Minimal client (zero cost - zero dependency).
const clientMainnet = createFhevmHostClient({ chain: fhevmMainnet });
/* -or- */ clientMainnet.fetchZamaPublicKey();

const clientTestnet = createFhevmHostClient({ chain: fhevmTestnet });
/* -or- */ clientTestnet.fetchZamaPublicKey();

await clientTestnet.extendWithEncryption();
const fhevmHandles = clientTestnet.encrypt(values);

// Throw
const fhevmHandles = clientMainnet.encrypt(values);

// await clientTestnet.extendWithDecryption();
// await clientTestnet.extendWithRelayer();

// Throw
await clientTestnet.publicDecrypt({ handles });

fhevmMainnet.fetchZamaPublicKeyBytes();
const fhevmHostClient = createFhevmHostClient({ chain: fhevmMainnet });
// lazy download if missing

// import { decryptionActions } from "decryptionModule"
const extendedClient1 = await fhevmHostClient.extendAsync(decryptionActions);
const extendedClient2 = await fhevmHostClient.extendAsync(encryptionActions); //bigZamaPublicKeyMainBytes
const extendedClient3 = fhevmHostClient.extend(relayerActions);

// extendWithUserDecryption()

// import { extendWithDecryption } from "decryptionModule"
// await extendWithDecryption(fhevmHostClient);

// decryptionModule.ts
// ===================
// export async function extendWithDecryption(fhevmClient: FhevmClient): Promise<void> {
//   // Preload
//   const tkmsModule = await initTkmsModule();
//   setActions(fhevmClient, "tkms", tkmsActions(tkmsModule));
// }










```
