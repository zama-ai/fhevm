import { mainnet, polygon, polygonAmoy, sepolia } from '@fhevm/sdk/chains';
import type { ethers as EthersT } from 'ethers';

import { HardhatFhevmError } from '../error';
import constants from './constants';

////////////////////////////////////////////////////////////////////////////////

/**
 * Which kind of node the plugin is talking to.
 *
 * Replaces `@fhevm/mock-utils`' `FhevmMockProvider`. The name drops "mock": nothing is mocked any
 * more — the local networks run a real cleartext FHEVM stack. What this still has to decide is which
 * *kind* of node is on the other end, because that selects the client factory (`cleartext` vs real)
 * and whether the plugin may deploy the stack itself.
 *
 * The detection is deliberately thinner than the version it replaces: the old one probed
 * `web3_clientVersion` and negotiated which `setCode`/`setBalance`/`impersonateAccount` spelling the
 * node accepted, all of which existed to drive the JS mock engine. That engine is gone, so the
 * network name and chain id are enough.
 */
export enum FhevmNetworkType {
  Unknown = 0,
  /** In-process `hardhat` network. */
  Hardhat = 1,
  /** A `hardhat node` server, reached over `--network localhost`. */
  HardhatNode = 2,
  Anvil = 3,
  SepoliaEthereumTestnet = 4,
  EthereumMainnet = 5,
  PolygonAmoyTestnet = 6,
  PolygonMainnet = 7,
}

/**
 * Chain ids come from `@fhevm/sdk`'s own chain definitions — it is the source of truth, so the
 * plugin reads them rather than restating numbers that can drift.
 */
const CHAIN_ID = {
  ethereumMainnet: mainnet.id,
  sepolia: sepolia.id,
  polygonMainnet: polygon.id,
  polygonAmoy: polygonAmoy.id,
} as const;

export type FhevmNetworkInfo = {
  readonly networkName: string;
  readonly chainId: number;
  readonly type: FhevmNetworkType;
  readonly url: string | undefined;
};

////////////////////////////////////////////////////////////////////////////////

export class FhevmNetworkProvider {
  readonly #info: FhevmNetworkInfo;
  readonly #readonlyEthersProvider: EthersT.Provider;

  private constructor(info: FhevmNetworkInfo, readonlyEthersProvider: EthersT.Provider) {
    this.#info = info;
    this.#readonlyEthersProvider = readonlyEthersProvider;
  }

  /**
   * Resolves the node kind from the Hardhat network name plus the live chain id.
   *
   * The chain id is read from the node rather than from the Hardhat config, because for
   * `--network localhost` the config value is ignored (a `hardhat node` is always 31337), and for
   * public networks it may simply be absent.
   */
  public static async resolve(parameters: {
    readonly readonlyEthersProvider: EthersT.Provider;
    readonly networkName: string;
    readonly configChainId: number | undefined;
    readonly url: string | undefined;
  }): Promise<FhevmNetworkProvider> {
    const { networkName, url, configChainId, readonlyEthersProvider } = parameters;

    const chainId = Number((await readonlyEthersProvider.getNetwork()).chainId);
    if (configChainId !== undefined && configChainId !== chainId && networkName !== 'localhost') {
      throw new HardhatFhevmError(
        `Network '${networkName}' is configured with chainId ${configChainId}, but the node reports ${chainId}.`,
      );
    }

    return new FhevmNetworkProvider(
      { networkName, chainId, url, type: __resolveType(networkName, chainId) },
      readonlyEthersProvider,
    );
  }

  public get info(): FhevmNetworkInfo {
    return this.#info;
  }

  public get chainId(): number {
    return this.#info.chainId;
  }

  public get readonlyEthersProvider(): EthersT.Provider {
    return this.#readonlyEthersProvider;
  }

  /**
   * A development node the plugin may deploy the cleartext stack onto, and which the SDK talks to in
   * cleartext mode.
   */
  public get isCleartext(): boolean {
    return (
      this.#info.type === FhevmNetworkType.Hardhat ||
      this.#info.type === FhevmNetworkType.HardhatNode ||
      this.#info.type === FhevmNetworkType.Anvil
    );
  }

  /**
   * A development node. Can deploy a cleartext stack if missing
   */
  public get isDevelopment(): boolean {
    return (
      this.#info.type === FhevmNetworkType.Hardhat ||
      this.#info.type === FhevmNetworkType.HardhatNode ||
      this.#info.type === FhevmNetworkType.Anvil
    );
  }

  /**
   * Any public network served by the real relayer — Ethereum or Polygon.
   *
   * This is the predicate that matters for behaviour: such a network is never deployed to and always
   * uses `createFhevmClient` rather than the cleartext factory.
   */
  public get isPublicNetwork(): boolean {
    return this.isEthereum || this.isPolygon;
  }

  /**
   * Ethereum specifically: mainnet or Sepolia.
   */
  public get isEthereum(): boolean {
    return this.isEthereumMainnet || this.isSepoliaEthereumTestnet;
  }

  public get isSepoliaEthereumTestnet(): boolean {
    return this.#info.type === FhevmNetworkType.SepoliaEthereumTestnet;
  }

  /**
   * Kept as an alias of `isSepoliaEthereumTestnet` for existing call sites.
   */
  public get isSepoliaEthereum(): boolean {
    return this.isSepoliaEthereumTestnet;
  }

  public get isEthereumMainnet(): boolean {
    return this.#info.type === FhevmNetworkType.EthereumMainnet;
  }

  /**
   * Any Polygon network: mainnet or the Amoy testnet.
   *
   * Left without a grouping suffix on purpose, to stay symmetric with {@link isEthereum} — a bare
   * chain-family name already reads as "any network in this family".
   */
  public get isPolygon(): boolean {
    return this.isPolygonMainnet || this.isPolygonAmoyTestnet;
  }

  public get isPolygonMainnet(): boolean {
    return this.#info.type === FhevmNetworkType.PolygonMainnet;
  }

  public get isPolygonAmoyTestnet(): boolean {
    return this.#info.type === FhevmNetworkType.PolygonAmoyTestnet;
  }

  /**
   * Whether this network runs against the *production* protocol deployment rather than the test one.
   *
   * This cuts across the chain families: Ethereum mainnet and Polygon both use gateway 261131, while
   * Sepolia and Polygon Amoy both use gateway 10901. So "which protocol" is not derivable from
   * "which chain family", and neither `isEthereum` nor `isPolygon` answers it.
   *
   * Local development nodes are neither production nor testnet — this is `false` for them, and so is
   * {@link isTestnetNetwork}. Do not read `!isProductionNetwork` as "testnet".
   */
  public get isProductionNetwork(): boolean {
    return this.isEthereumMainnet || this.isPolygonMainnet;
  }

  /**
   * The counterpart of {@link isProductionNetwork}: public networks on the test protocol.
   */
  public get isTestnetNetwork(): boolean {
    return this.isSepoliaEthereumTestnet || this.isPolygonAmoyTestnet;
  }

  public async getCodeAt(address: string): Promise<string> {
    return await this.#readonlyEthersProvider.getCode(address);
  }
}

////////////////////////////////////////////////////////////////////////////////

// eslint-disable-next-line @typescript-eslint/naming-convention
function __resolveType(networkName: string, chainId: number): FhevmNetworkType {
  if (networkName === 'hardhat') {
    return FhevmNetworkType.Hardhat;
  }
  if (networkName === 'localhost') {
    return FhevmNetworkType.HardhatNode;
  }
  if (networkName === 'anvil') {
    return FhevmNetworkType.Anvil;
  }
  if (chainId === CHAIN_ID.ethereumMainnet) {
    return FhevmNetworkType.EthereumMainnet;
  }
  if (chainId === CHAIN_ID.sepolia) {
    return FhevmNetworkType.SepoliaEthereumTestnet;
  }
  if (chainId === CHAIN_ID.polygonMainnet) {
    return FhevmNetworkType.PolygonMainnet;
  }
  if (chainId === CHAIN_ID.polygonAmoy) {
    return FhevmNetworkType.PolygonAmoyTestnet;
  }
  // A named network on 31337 (the `devnet` .env flow) is still a local development node.
  if (chainId === constants.DEVELOPMENT_NETWORK_CHAINID) {
    return FhevmNetworkType.Anvil;
  }
  return FhevmNetworkType.Unknown;
}
