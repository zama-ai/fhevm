import { createFhevmClient, hasFhevmRuntimeConfig, initFhevmRuntime, setFhevmRuntimeConfig } from '@fhevm/sdk/ethers';
import { createFhevmCleartextClient } from '@fhevm/sdk/ethers/cleartext';
import debug from 'debug';
import type { ethers as EthersT } from 'ethers';
import { vars } from 'hardhat/config';
import type { HardhatRuntimeEnvironment } from 'hardhat/types';

import { HardhatFhevmError } from '../error';
import type { HardhatFhevmRuntimeEnvironment } from '../types';
import { FhevmDebugger } from './FhevmDebugger';
import { FhevmEnvironmentPaths } from './FhevmEnvironmentPaths';
import { FhevmExternalAPI } from './FhevmExternalAPI';
import { localCleartext, mainnet, sepolia } from './chains';
import constants from './constants';
import {
  FhevmCleartextContractsRepository,
  FhevmContractsRepository,
  isCleartextContractsRepository,
} from './contractsRepository';
import type { CoprocessorConfig } from './coprocessorConfig';
import { deployFhevmCleartextHostContracts } from './deploy/setup';
import { assertHHFhevm } from './error';
import type { FhevmContractName } from './migration/placeholders';
import { FhevmNetworkProvider, FhevmNetworkType } from './networkProvider';
import type { FhevmClient } from './sdkTypes';
import { getEnvString, getOptionalEnvString } from './utils/env';
import { assertIsAddress } from './utils/ethers';
import { checkHardhatRuntimeEnvironment } from './utils/hh';

const debugProvider = debug('@fhevm/hardhat:provider');
const debugInstance = debug('@fhevm/hardhat:instance');
const debugAddresses = debug('@fhevm/hardhat:addresses');

export type FhevmEnvironmentAddresses = {
  /**
   * Indicates the addresses stored in the solidity `CoprocessorConfig` struct used in the project.
   */
  CoprocessorConfig: CoprocessorConfig;
  /**
   * Indicates the address of the solidity contract `InputVerifier.sol` used in the project.
   */
  InputVerifierAddress: `0x${string}`;
  /**
   * Indicates the relayer url used in the project.
   */
  relayerUrl?: string;
  /**
   * Indicates whether the addresses were resolved using env variables.
   */
  resolvedUsingEnv: boolean;
};

export type FhevmSigners = {
  coprocessor: EthersT.Signer[];
  kms: EthersT.Signer[];
  zero: EthersT.Signer;
  zeroAddress: string;
  one: EthersT.Signer;
  oneAddress: string;
};

export type FhevmProviderInfo = {
  web3ClientVersion: string;
  url?: string;
  networkName: string;
  isNetworkHardhatNode: boolean;
  isAnvil: boolean;
  methods: {
    setCode?: string;
    impersonateAccount?: string;
    setBalance?: string;
  };
};

export type FhevmContractRecordEntry = {
  contractName: FhevmContractName;
  address: string;
  contract: EthersT.Contract;
  package: string;
};

export class FhevmEnvironment {
  private readonly _hre: HardhatRuntimeEnvironment;
  private _runningInHHNode: boolean | undefined;
  private _runningInHHTest: boolean | undefined;
  private readonly _paths: FhevmEnvironmentPaths;
  private _deployRunning: boolean = false;
  private _deployCompleted: boolean = false;
  private _cliAPIInitializing: boolean = false;
  private _cliAPIInitialized: boolean = false;
  private _setupAddressesRunning: boolean = false;
  private _setupAddressesCompleted: boolean = false;
  private _addresses: FhevmEnvironmentAddresses | undefined;
  private _fhevmCleartextProvider: FhevmNetworkProvider | undefined;
  private _minimalInitPromise: Promise<void> | undefined;
  private _initializeCLIApiPromise: Promise<void> | undefined;
  private _contractsRepository: FhevmContractsRepository | undefined;
  private _instance: FhevmClient | undefined;
  private readonly _fhevmAPI: FhevmExternalAPI;
  private readonly _fhevmDebugger: FhevmDebugger;

  /**
   * Constructor must be ultra-lightweight!
   */
  constructor(hre: HardhatRuntimeEnvironment) {
    //
    // Important node:
    // ===============
    // - calling `import { fhevm } from "hardhat"` does NOT call the `FhevmEnvironment` constructor
    // - since we are overriding the "hardhat test" command, the `FhevmEnvironment` is created
    //   in our builtin-task.ts/task(TASK_TEST, ...) command.
    //
    this._hre = hre;

    this._fhevmAPI = new FhevmExternalAPI(this);
    this._fhevmDebugger = new FhevmDebugger(this);
    this._paths = new FhevmEnvironmentPaths(hre.config.paths.root);

    checkHardhatRuntimeEnvironment(hre);
  }

  public setRunningInHHTest(): void {
    if (this._runningInHHTest !== undefined) {
      throw new HardhatFhevmError(`The fhevm hardhat plugin is already running inside a hardhat test command.`);
    }
    if (this._runningInHHNode !== undefined) {
      throw new HardhatFhevmError(`The fhevm hardhat plugin is already running inside a hardhat node command.`);
    }
    this._runningInHHTest = true;
  }

  /**
   * `npx hardhat node` only supports the 'hardhat' network
   * Running `npx hardhat node --network <anything-other-than-hardhat>` will raise the following error
   * Error HH605: Unsupported network for JSON-RPC server. Only hardhat is currently supported.
   * Note that `npx hardhat node --network localhost` also fails.
   */
  public setRunningInHHNode(): void {
    assertHHFhevm(
      this._hre.network.name === 'hardhat',
      `Expecting network 'hardhat'. Got '${this._hre.network.name}' instead.`,
    );
    if (this._runningInHHTest !== undefined) {
      throw new HardhatFhevmError(`The fhevm hardhat plugin is already running inside a hardhat test command.`);
    }
    if (this._runningInHHNode !== undefined) {
      throw new HardhatFhevmError(`The fhevm hardhat plugin is already running inside a hardhat node command.`);
    }
    this._runningInHHNode = true;
  }

  public get isRunningInHHTest(): boolean {
    return this._runningInHHTest === true;
  }

  public get isRunningInHHNode(): boolean {
    return this._runningInHHNode === true;
  }

  public get hre(): HardhatRuntimeEnvironment {
    return this._hre;
  }

  /*
    Warning: MUST BE instance of `HardhatEthersProvider`
    Same as `readonlyEthersProvider` but in `MinimalProvider` format
  */
  public get relayerProvider(): EthersT.Provider {
    return this.hre.ethers.provider;
  }

  /*
    Warning: MUST NOT BE window.ethereum!!!!!
    Same as `readonlyEthersProvider` but in `MinimalProvider` format
    To call view function on contracts
  */
  public get readonlyEthersProvider(): EthersT.Provider {
    return this.hre.ethers.provider;
  }

  /*
    Warning: MUST NOT BE window.ethereum!!!!!
    Same as `readonlyEthersProvider` but in `MinimalProvider` format
  */
  public get readonlyEip1193Provider(): EthersT.Eip1193Provider {
    return this.hre.network.provider;
  }

  // Should be replaced!
  public get cleartextProvider(): FhevmNetworkProvider {
    if (!this._fhevmCleartextProvider) {
      throw new HardhatFhevmError(`The Hardhat Fhevm plugin is not initialized.`);
    }
    return this._fhevmCleartextProvider;
  }

  public get paths(): FhevmEnvironmentPaths {
    return this._paths;
  }

  public get debugger(): FhevmDebugger {
    return this._fhevmDebugger;
  }

  public getInstanceOrUndefined(): FhevmClient | undefined {
    return this._instance;
  }

  public get instance(): FhevmClient {
    if (!this._instance) {
      throw new HardhatFhevmError(`The Hardhat Fhevm plugin is not initialized.`);
    }
    return this._instance;
  }

  public getRelayerUrl(): string {
    const relayerUrl = this.__getAddresses().relayerUrl;
    if (relayerUrl === undefined) {
      throw new HardhatFhevmError(`The relayerUrl is not initialized.`);
    }
    return relayerUrl;
  }

  public resolveRelayerUrl(aclAddress: string): string {
    if (this.cleartextProvider.isCleartext) {
      throw new HardhatFhevmError(`relayerUrl is not defined in cleartext mode.`);
    }

    // Public networks: the relayer url is part of `@fhevm/sdk`'s chain definition.
    for (const chain of [sepolia, mainnet]) {
      if (aclAddress === chain.fhevm.contracts.acl.address) {
        return chain.fhevm.relayerUrl;
      }
    }

    const dotEnvFile = this._paths.dotEnvFile;
    if (aclAddress === getEnvString({ name: 'ACL_CONTRACT_ADDRESS', dotEnvFile })) {
      return getEnvString({ name: 'RELAYER_URL', dotEnvFile });
    }

    throw new HardhatFhevmError(`There is no relayerUrl defined for ACL address '${aclAddress}'.`);
  }

  private __getAddresses(): FhevmEnvironmentAddresses {
    if (!this._addresses) {
      throw new HardhatFhevmError(`The Hardhat Fhevm plugin is not initialized.`);
    }
    return this._addresses;
  }

  public getACLAddress(): `0x${string}` {
    if (!this._addresses) {
      throw new HardhatFhevmError(`The Hardhat Fhevm plugin is not initialized.`);
    }
    return this._addresses.CoprocessorConfig.ACLAddress;
  }

  public getFHEVMExecutorAddress(): `0x${string}` {
    if (!this._addresses) {
      throw new HardhatFhevmError(`The Hardhat Fhevm plugin is not initialized.`);
    }
    return this._addresses.CoprocessorConfig.CoprocessorAddress;
  }

  public getInputVerifierAddress(): `0x${string}` {
    if (!this._addresses) {
      throw new HardhatFhevmError(`The Hardhat Fhevm plugin is not initialized.`);
    }
    return this._addresses.InputVerifierAddress;
  }

  public getKMSVerifierAddress(): `0x${string}` {
    if (!this._addresses) {
      throw new HardhatFhevmError(`The Hardhat Fhevm plugin is not initialized.`);
    }
    return this._addresses.CoprocessorConfig.KMSVerifierAddress;
  }

  public getACLReadOnly(): EthersT.Contract {
    if (!this._contractsRepository) {
      throw new HardhatFhevmError(`The Hardhat Fhevm plugin is not initialized.`);
    }
    return this._contractsRepository.acl.readonlyContract;
  }

  public getFHEVMExecutorReadOnly(): EthersT.Contract {
    if (!this._contractsRepository) {
      throw new HardhatFhevmError(`The Hardhat Fhevm plugin is not initialized.`);
    }
    return this._contractsRepository.fhevmExecutor.readonlyContract;
  }

  public getInputVerifierReadOnly(): EthersT.Contract {
    if (!this._contractsRepository) {
      throw new HardhatFhevmError(`The Hardhat Fhevm plugin is not initialized.`);
    }
    return this._contractsRepository.inputVerifier.readonlyContract;
  }

  public getKMSVerifierReadOnly(): EthersT.Contract {
    if (!this._contractsRepository) {
      throw new HardhatFhevmError(`The Hardhat Fhevm plugin is not initialized.`);
    }
    return this._contractsRepository.kmsVerifier.readonlyContract;
  }

  /** From the chain definition — `@fhevm/sdk` is the source of truth for gateway identity. */
  public getGatewayChainId(): number {
    return this.cleartextProvider.isEthereum
      ? (this.cleartextProvider.isEthereumMainnet ? mainnet : sepolia).fhevm.gateway.id
      : localCleartext.fhevm.gateway.id;
  }

  public get chainId(): number {
    if (!this._fhevmCleartextProvider) {
      throw new HardhatFhevmError(`The Hardhat Fhevm plugin is not initialized.`);
    }
    return this._fhevmCleartextProvider.chainId;
  }

  /**
   *  Client API
   */
  get externalFhevmAPI(): HardhatFhevmRuntimeEnvironment {
    if (this.isRunningInHHNode) {
      // Cannot be called from the server process
      throw new HardhatFhevmError(
        `the HardhatFhevmRuntimeEnvironment 'fhevm' is not accessible from the 'hardhat node' server`,
      );
    }
    return this._fhevmAPI;
  }

  // Accessible after _deloyCore
  public getContractsRepository(): FhevmContractsRepository {
    if (!this._contractsRepository) {
      throw new HardhatFhevmError(`The Hardhat Fhevm plugin is not initialized.`);
    }
    return this._contractsRepository;
  }

  public get isDeployed(): boolean {
    return this._deployCompleted;
  }

  public async initializeCLIApi(): Promise<void> {
    if (this._initializeCLIApiPromise !== undefined) {
      return this._initializeCLIApiPromise;
    }

    // Create one in-flight promise and allow retries on failure
    this._initializeCLIApiPromise = (async () => {
      try {
        await this.__initializeCLIApi();
      } finally {
        // Clear whether success or failure, so callers can retry if it failed.
        this._initializeCLIApiPromise = undefined;
      }
    })();

    return this._initializeCLIApiPromise;
  }

  private async __initializeCLIApi(): Promise<void> {
    // Allow multiple calls
    if (this._cliAPIInitialized) {
      return;
    }
    // Defensive: this should already be guaranteed by _initializeCLIApiPromise
    if (this._cliAPIInitializing) {
      throw new HardhatFhevmError(`The Fhevm CLI initialization is already in progress.`);
    }

    this._cliAPIInitializing = true;

    try {
      if (this.isDeployed) {
        return;
      }

      if (this.hre.network.name === 'hardhat') {
        throw new HardhatFhevmError(
          `The Fhevm CLI only supports the Hardhat Node (--network localhost) or Sepolia (--network sepolia) networks.`,
        );
      }

      await this.minimalInit();

      if (
        this.cleartextProvider.info.type !== FhevmNetworkType.HardhatNode &&
        this.cleartextProvider.info.type !== FhevmNetworkType.SepoliaEthereumTestnet &&
        this.cleartextProvider.info.type !== FhevmNetworkType.EthereumMainnet
      ) {
        throw new HardhatFhevmError(
          `The Fhevm CLI only supports the Hardhat Node (--network localhost), Sepolia (--network sepolia) or Mainnet (--network mainnet) networks.`,
        );
      }

      // TODO: should improve deploy() (see function commentary)
      await this.deploy();

      this._cliAPIInitialized = true;
    } finally {
      this._cliAPIInitializing = false;
    }
  }

  /**
   * TODO: Should be improved:
   * - if `Sepolia`: no need to deploy! just create instance and pick up addresses
   *   Ex: `npx hardhat fhevm user-decrypt --network sepolia ...`
   * - if `Hardhat Node`: it's already deployed (because of `npx hardhat node` CLI auto deploy)
   * - if `Anvil`: may no be deployed (maybe add `npx hardhat fhevm anvil`)
   */
  public async deploy(): Promise<void> {
    if (this._deployCompleted) {
      throw new HardhatFhevmError('The Fhevm environment is already initialized.');
    }
    if (this._deployRunning) {
      throw new HardhatFhevmError(`The Fhevm environment initialization is already in progress.`);
    }

    this._deployRunning = true;

    try {
      await this._deployCore();

      this._deployCompleted = true;
    } finally {
      this._deployRunning = false;
    }
  }

  private __guessDefaultProvider(): {
    networkName: string;
    type: FhevmNetworkType;
    chainId: number | undefined;
    url: string | undefined;
  } {
    const url: string | undefined = 'url' in this.hre.network.config ? this.hre.network.config.url : undefined;

    if (this.hre.network.name === 'hardhat') {
      assertHHFhevm(url === undefined);
      return {
        networkName: this.hre.network.name,
        type: FhevmNetworkType.Hardhat,
        chainId: this.hre.network.config.chainId,
        url,
      };
    }

    if (url === undefined || url.length === 0) {
      throw new HardhatFhevmError(`Missing network url`);
    }

    // Check if url is well formed
    const urlObj = new URL(url);

    // Note: specifying the chainId in the HardhatUserConfig for network "localhost"
    // has no effect when running the 'npx hardhat node' command
    // the chainId is automatically set to 31337
    if (this.hre.network.name === 'localhost') {
      assertHHFhevm(urlObj.port === '8545');
      return {
        networkName: 'localhost',
        type: FhevmNetworkType.HardhatNode,
        chainId: 31337,
        url,
      };
    }

    if (this.hre.network.name === 'anvil') {
      return {
        networkName: this.hre.network.name,
        type: FhevmNetworkType.Anvil,
        chainId: this.hre.network.config.chainId,
        url,
      };
    }

    return {
      networkName: this.hre.network.name,
      type: FhevmNetworkType.Unknown,
      chainId: this.hre.network.config.chainId,
      url,
    };
  }

  //////////////////////////////////////////////////////////////////////////////
  // MinimalInit
  //////////////////////////////////////////////////////////////////////////////

  // Can be called multiple times
  public async minimalInitWithAddresses(): Promise<void> {
    return this.__minimalInit({ initializeAddresses: true });
  }

  // Can be called multiple times
  public async minimalInit(): Promise<void> {
    return this.__minimalInit();
  }

  // Can be called multiple times
  private async __minimalInit(options?: { initializeAddresses?: boolean }): Promise<void> {
    if (this._minimalInitPromise !== undefined) {
      return this._minimalInitPromise;
    }

    // Create one in-flight promise and allow retries on failure
    this._minimalInitPromise = (async () => {
      try {
        await this.__minimalInitCore(options);
      } finally {
        // Clear whether success or failure, so callers can retry if it failed.
        this._minimalInitPromise = undefined;
      }
    })();

    return this._minimalInitPromise;
  }

  private async __minimalInitCore(options?: { initializeAddresses?: boolean }): Promise<void> {
    if (this._fhevmCleartextProvider === undefined) {
      const defaults = this.__guessDefaultProvider();

      debugProvider(`Default provider network: ${defaults.networkName}, type: ${defaults.type}, url: ${defaults.url}`);
      debugProvider(`Default provider type   : ${defaults.type}, url: ${defaults.url}`);
      debugProvider(`Default provider url    : ${defaults.url}`);
      debugProvider('Resolving provider...');

      this._fhevmCleartextProvider = await FhevmNetworkProvider.resolve({
        readonlyEthersProvider: this.hre.ethers.provider,
        networkName: this.hre.network.name,
        configChainId: defaults.chainId,
        url: defaults.url,
      });

      debugProvider(
        `Provider name: ${this._fhevmCleartextProvider.info.networkName} chainId: ${this._fhevmCleartextProvider.info.chainId} type: ${this._fhevmCleartextProvider.info.type}`,
      );
    }

    if (!this.cleartextProvider.isCleartext && !this.cleartextProvider.isPublicNetwork) {
      throw new HardhatFhevmError(
        "The current version of the fhevm hardhat plugin only supports the 'hardhat' network, 'localhost' hardhat node, anvil, sepolia, mainnet, polygon or polygon amoy.",
      );
    }

    if (options?.initializeAddresses === true) {
      // Can be called multiple times
      await this.__initializeAddresses();
    }
  }

  //////////////////////////////////////////////////////////////////////////////

  private async _deployCore(): Promise<void> {
    await this.minimalInitWithAddresses();

    const fhevmAddresses = this.__getAddresses();

    if (this.cleartextProvider.isDevelopment) {
      // a cleartext stack can only be deployed on a development node (anvil, hardhat, hardhat node)
      await deployFhevmCleartextHostContracts(this.hre.ethers.provider);
      this._contractsRepository = this.__createContractsRepository();
    } else {
      // use the existing deployed stack
      this._contractsRepository = this.__createContractsRepository();
      debugAddresses(`ACL: ${fhevmAddresses.CoprocessorConfig.ACLAddress}`);
    }

    if (!this.isRunningInHHNode) {
      this._instance = await this.createFhevmClient();
    }
  }

  /**
   * Initializes the process-wide `@fhevm/sdk` runtime config.
   *
   * `setFhevmRuntimeConfig` is a singleton that throws if called again with different parameters, and
   * both the cleartext and the real runtime refuse to build a client until it has been called. So it
   * happens exactly once, here, before any client exists — which is also why the API key has to be
   * read now: in `@fhevm/sdk` `auth` belongs to the runtime config, not to the individual client as it
   * did in `createInstance({ auth })`.
   */
  private __initFhevmRuntimeConfig(): void {
    if (hasFhevmRuntimeConfig()) {
      return;
    }

    const ZAMA_FHEVM_API_KEY: string | undefined = vars.has('ZAMA_FHEVM_API_KEY')
      ? vars.get('ZAMA_FHEVM_API_KEY')
      : undefined;

    // Note the discriminant is `type`, not the relayer-sdk's `__type`. `ApiKeyHeader` is the only
    // form Zama's hosted relayer accepts.
    // An empty key counts as absent: sending `value: ''` would fail at the relayer, not here.
    setFhevmRuntimeConfig(
      ZAMA_FHEVM_API_KEY === undefined || ZAMA_FHEVM_API_KEY === ''
        ? {}
        : { auth: { type: 'ApiKeyHeader', header: 'x-api-key', value: ZAMA_FHEVM_API_KEY } },
    );
  }

  /**
   * Builds the `@fhevm/sdk` client for the current network.
   *
   * Both factories take the same parameters and return the same `FhevmClient`, so nothing downstream
   * branches — only the factory and the chain differ:
   *
   *   - local (31337)
   *      `createFhevmCleartextClient` + the `localCleartext` chain. Reads cleartexts
   *      straight off `CleartextDB`; no relayer, no WASM.
   *
   *   - sepolia / mainnet / polygon / polygon amoy
   *      `createFhevmClient` + the SDK's own chain definitions. Talks to the real
   *      relayer, so the TFHE/TKMS WASM must be loaded first via `initFhevmRuntime()`.
   *
   * The host contracts, by ABI. Used only to decode reverts into named custom errors, so it needs
   * addresses and ABIs and nothing else.
   */
  private __createContractsRepository(): FhevmContractsRepository {
    const addresses = this.__getAddresses();

    // The four core contracts are known on every network.
    const host = {
      aclAddress: addresses.CoprocessorConfig.ACLAddress,
      fhevmExecutorAddress: addresses.CoprocessorConfig.CoprocessorAddress,
      inputVerifierAddress: addresses.InputVerifierAddress,
      kmsVerifierAddress: addresses.CoprocessorConfig.KMSVerifierAddress,
    };

    // A public network gets the host repository and nothing more: the rest of the v13 stack sits at
    // addresses the plugin cannot derive there, and the cleartext contracts do not exist at all.
    // Leaving them unregistered beats pointing a Sepolia lookup at localhost addresses.
    if (!this.cleartextProvider.isCleartext) {
      return new FhevmContractsRepository(this.readonlyEthersProvider, host);
    }

    const cleartext = constants.FHEVM_HOST_CONTRACTS_CLEARTEXT_PACKAGE;

    // Only *development* cleartext networks are supported for now. A remote cleartext deployment (a
    // cleartext Hoodi testnet, say) is a real possibility, and it would reach this point: it is a
    // cleartext network, so the branch above lets it through. What it does not have is the address
    // set below, which is precomputed for the local stack only.
    //
    // TODO: resolve the cleartext addresses from the chain (or from configuration) instead of the
    // precomputed constants, and this restriction goes away.
    if (!this.cleartextProvider.isDevelopment) {
      const { networkName, chainId } = this.cleartextProvider.info;
      throw new HardhatFhevmError(
        `Unsupported cleartext network '${networkName}' (chainId ${chainId}).\n` +
          `\n` +
          `Values on '${networkName}' are in cleartext, but the plugin only knows the contract addresses of ` +
          `the *local* cleartext stack` +
          `A remote cleartext deployment has its own addresses, which the plugin cannot derive yet.\n` +
          `\n` +
          `Use the 'hardhat' network, a 'hardhat node' ('--network localhost'), or anvil.`,
      );
    }

    return new FhevmCleartextContractsRepository(this.readonlyEthersProvider, {
      ...host,
      hcuLimitAddress: cleartext.fhevmAddresses.hcuLimitAddress,
      protocolConfigAddress: cleartext.fhevmAddresses.protocolConfigAddress,
      kmsGenerationAddress: cleartext.fhevmAddresses.kmsGenerationAddress,
      pauserSetAddress: cleartext.pauserSetAddress,
      cleartextArithmeticAddress: cleartext.cleartextAddresses.cleartextArithmeticAddress,
      cleartextDbAddress: cleartext.cleartextAddresses.cleartextDbAddress,
    });
  }

  /**
   * The contracts repository, narrowed to the cleartext one.
   *
   * Throws off a public network, where `CleartextArithmetic`/`CleartextDB` genuinely do not exist —
   * the caller is asking for something the chain cannot provide, so saying so plainly beats failing
   * later inside an `eth_call`.
   */
  public getCleartextContractsRepository(): FhevmCleartextContractsRepository {
    const repository = this.getContractsRepository();
    if (!isCleartextContractsRepository(repository)) {
      const { networkName, chainId } = this.cleartextProvider.info;
      throw new HardhatFhevmError(
        `'${networkName}' (chainId ${chainId}) is not a cleartext network: values there are really encrypted, ` +
          `so 'CleartextArithmetic' and 'CleartextDB' are not deployed and there is nothing to read. ` +
          `Decrypt through the relayer instead — fhevm.userDecryptE*() or fhevm.publicDecryptE*().`,
      );
    }
    return repository;
  }

  public async createFhevmClient(): Promise<FhevmClient> {
    assertHHFhevm(!this.isRunningInHHNode, "Cannot create an FhevmClient object in the 'hardhat node' server");

    this.__initFhevmRuntimeConfig();

    if (this.cleartextProvider.isCleartext) {
      debugInstance(`Creating @fhevm/sdk cleartext client (chain ${localCleartext.id})...`);
      const client = createFhevmCleartextClient({ provider: this.hre.ethers.provider, chain: localCleartext });
      // Resolving the on-chain context is a prerequisite for every action; without it the first call
      // fails with "Fhevm context has not been resolved".
      await client.ready;
      debugInstance('@fhevm/sdk cleartext client created.');
      return client;
    }

    if (this.cleartextProvider.isEthereum) {
      const chain = this.cleartextProvider.isEthereumMainnet ? mainnet : sepolia;

      debugInstance('Loading @fhevm/sdk runtime (WASM, might take some time)...');
      await initFhevmRuntime();

      debugInstance(`Creating @fhevm/sdk client (chain ${chain.id})...`);
      const client = createFhevmClient({ provider: this.hre.ethers.provider, chain });
      await client.ready;
      debugInstance('@fhevm/sdk client created.');
      return client;
    }

    throw new HardhatFhevmError(`Unsupported network.`);
  }

  /**
   * Generates:
   *  - `/path/to/user-package/fhevmTemp/@fhevm/solidity/config/ZamaConfig.sol`
   */
  // eslint-disable-next-line @typescript-eslint/require-await
  private async __initializeAddresses(): Promise<FhevmEnvironmentAddresses> {
    if (this._addresses !== undefined) {
      return this._addresses;
    }

    // Prevent multiple calls.
    if (this._setupAddressesCompleted) {
      throw new HardhatFhevmError('The Fhevm environment addresses are already initialized.');
    }
    if (this._setupAddressesRunning) {
      throw new HardhatFhevmError('The Fhevm environment addresses are already being initialized.');
    }

    this._setupAddressesRunning = true;

    {
      let addresses: FhevmEnvironmentAddresses;
      if (this.cleartextProvider.isSepoliaEthereumTestnet) {
        const envNetworkName = getOptionalEnvString({
          name: 'FHEVM_HARDHAT_NETWORK',
          dotEnvFile: this._paths.dotEnvFile,
        });
        // Could be removed in the future.
        // This is a security check to prevent invalid contract configuration
        if (this.cleartextProvider.info.networkName === 'devnet' && envNetworkName !== 'devnet') {
          throw new HardhatFhevmError(
            `Network 'devnet' requires an .env file. File '${this._paths.dotEnvFile}' does not exist or is invalid.`,
          );
        }
        if (envNetworkName === this.cleartextProvider.info.networkName) {
          addresses = this._initializeAddressesEnv();
        } else {
          addresses = this._initializeAddressesSepolia();
        }
      } else if (this.cleartextProvider.isEthereumMainnet) {
        addresses = this._initializeAddressesMainnet();
      } else {
        addresses = this._initializeAddressesMock();
      }
      Object.freeze(addresses);
      Object.freeze(addresses.CoprocessorConfig);

      this._addresses = addresses;
    }

    this._setupAddressesCompleted = true;
    this._setupAddressesRunning = false;

    return this._addresses;
  }

  private _initializeAddressesEnv(): FhevmEnvironmentAddresses {
    const dotEnvFile = this._paths.dotEnvFile;

    debugAddresses(`Resolving addresses using ${dotEnvFile}`);

    const ACLAddress = getEnvString({ name: 'ACL_CONTRACT_ADDRESS', dotEnvFile });
    const CoprocessorAddress = getEnvString({ name: 'FHEVM_EXECUTOR_CONTRACT_ADDRESS', dotEnvFile });
    const KMSVerifierAddress = getEnvString({ name: 'KMS_VERIFIER_CONTRACT_ADDRESS', dotEnvFile });
    const InputVerifierAddress = getEnvString({ name: 'INPUT_VERIFIER_CONTRACT_ADDRESS', dotEnvFile });
    const HCULimitAddress = getEnvString({ name: 'HCU_LIMIT_CONTRACT_ADDRESS', dotEnvFile });
    const relayerUrl = getEnvString({ name: 'RELAYER_URL', dotEnvFile });

    assertIsAddress(ACLAddress, 'Environment variable ACL_CONTRACT_ADDRESS');
    assertIsAddress(CoprocessorAddress, 'Environment variable FHEVM_EXECUTOR_CONTRACT_ADDRESS');
    assertIsAddress(KMSVerifierAddress, 'Environment variable KMS_VERIFIER_CONTRACT_ADDRESS');
    assertIsAddress(InputVerifierAddress, 'Environment variable INPUT_VERIFIER_CONTRACT_ADDRESS');
    assertIsAddress(HCULimitAddress, 'Environment variable HCU_LIMIT_CONTRACT_ADDRESS');

    debugAddresses(`Using relayerUrl: ${relayerUrl}`);

    const envCoprocessorConfig: CoprocessorConfig = {
      ACLAddress,
      CoprocessorAddress,
      KMSVerifierAddress,
    };

    return {
      CoprocessorConfig: envCoprocessorConfig,
      InputVerifierAddress: InputVerifierAddress,
      relayerUrl,
      resolvedUsingEnv: true,
    };
  }

  /**
   * Sepolia addresses come from `@fhevm/sdk`'s own `sepolia` chain definition — the SDK is the source
   * of truth, so the plugin no longer keeps a copy that can (and did) go stale.
   *
   * `FhevmChain` does not model `FHEVMExecutor`, so the coprocessor address is taken from
   * `@fhevm/solidity`'s `ZamaConfig.sol`, which is the contract dApps compile against and which
   * `generateZamaConfigDotSol` already validates byte-for-byte against the installed package.
   */
  private _initializeAddressesSepolia(): FhevmEnvironmentAddresses {
    debugAddresses(`Resolving addresses using @fhevm/sdk's sepolia chain definition`);

    const sepoliaCoprocessorConfig: CoprocessorConfig = {
      ACLAddress: sepolia.fhevm.contracts.acl.address,
      CoprocessorAddress: constants.FHEVM_SOLIDITY_PACKAGE.SepoliaConfig.CoprocessorAddress,
      KMSVerifierAddress: sepolia.fhevm.contracts.kmsVerifier.address,
    };

    const relayerUrl = sepolia.fhevm.relayerUrl;
    debugAddresses(`Using relayerUrl: ${relayerUrl}`);

    return {
      CoprocessorConfig: sepoliaCoprocessorConfig,
      InputVerifierAddress: sepolia.fhevm.contracts.inputVerifier.address,
      relayerUrl,
      resolvedUsingEnv: false,
    };
  }

  /** Mainnet, same sourcing rules as `_initializeAddressesSepolia`. */
  private _initializeAddressesMainnet(): FhevmEnvironmentAddresses {
    debugAddresses(`Resolving addresses using @fhevm/sdk's mainnet chain definition`);

    const mainnetCoprocessorConfig: CoprocessorConfig = {
      ACLAddress: mainnet.fhevm.contracts.acl.address,
      CoprocessorAddress: constants.FHEVM_SOLIDITY_PACKAGE.EthereumConfig.CoprocessorAddress,
      KMSVerifierAddress: mainnet.fhevm.contracts.kmsVerifier.address,
    };

    const relayerUrl = mainnet.fhevm.relayerUrl;
    debugAddresses(`Using relayerUrl: ${relayerUrl}`);

    return {
      CoprocessorConfig: mainnetCoprocessorConfig,
      InputVerifierAddress: mainnet.fhevm.contracts.inputVerifier.address,
      relayerUrl,
      resolvedUsingEnv: false,
    };
  }

  /**
   * The canonical localhost cleartext stack. Every address is a constant: the stack is deployed from a
   * fixed account at a fixed start nonce, so `CREATE(deployer, nonce)` fixes all of them, and
   * `@fhevm/solidity/config/ZamaConfig.sol` compiles the ACL/FHEVMExecutor/KMSVerifier triple straight
   * into consumer contracts.
   *
   * This replaces the old discovery dance — etching `ACL` at a dummy address to read
   * `getFHEVMExecutorAddress()`, caching the result to JSON, and re-entering through a child
   * `hardhat fhevm install-solidity` process when the network was not `hardhat`. None of that is
   * needed once the addresses are known up front.
   */
  private _initializeAddressesMock(): FhevmEnvironmentAddresses {
    const cleartext = constants.FHEVM_HOST_CONTRACTS_CLEARTEXT_PACKAGE;

    debugAddresses(`Resolving addresses using the canonical ${cleartext.name}@${cleartext.version} local stack`);

    const mockCoprocessorConfig: CoprocessorConfig = {
      ACLAddress: cleartext.fhevmAddresses.aclAddress as `0x${string}`,
      CoprocessorAddress: cleartext.fhevmAddresses.fhevmExecutorAddress as `0x${string}`,
      KMSVerifierAddress: cleartext.fhevmAddresses.kmsVerifierAddress as `0x${string}`,
    };

    debugAddresses(`No relayerUrl in Mock config`);

    // No relayerUrl in Mock config
    return {
      CoprocessorConfig: mockCoprocessorConfig,
      InputVerifierAddress: cleartext.fhevmAddresses.inputVerifierAddress as `0x${string}`,
      resolvedUsingEnv: true,
    };
  }

  public getSoliditySourcePaths(): string[] {
    return [];
  }
}
