import {
  Contract,
  ContractFactory,
  type HDNodeWallet,
  JsonRpcProvider,
  Network,
  type TransactionReceipt,
  type TransactionResponse,
} from 'ethers';

import { deriveAccounts } from '../../../config/profiles/accounts.js';
import type { ResolvedChainSection } from '../../../config/profiles/types.js';
import type { Evidence } from '../../world/evidence.js';
import { type ContractArtifact, loadArtifact } from './artifacts.js';
import { EngineWallet } from './wallets.js';

export interface SendOptions {
  /** Human-readable label of the transaction, shown in the evidence and in error messages. */
  label?: string;
}

export interface DeployOptions extends SendOptions {
  /** Wallet role that deploys the contract. */
  from: string;
}

export interface ChainOptions {
  profileName: string;
  section: ResolvedChainSection;
  evidence: Pick<Evidence, 'record'>;
  /** How often the provider polls for receipts and blocks (default 1000 ms). */
  pollingIntervalMs?: number;
}

/** `from`/`deployer` of an evidence entry: the wallet role when the address is an engine wallet. */
export interface EvidenceAccount {
  role?: string;
  address: string;
}

const DEFAULT_POLLING_INTERVAL_MS = 1_000;
const DEFAULT_WAIT_FOR_BLOCK_TIMEOUT_MS = 60_000;

const errorMessage = (error: unknown): string =>
  (error as { shortMessage?: string }).shortMessage ?? (error instanceof Error ? error.message : String(error));

const described = (what: string, label: string | undefined): string => (label ? `${what} '${label}'` : what);

/**
 * Chain access for scenario steps, built from the profile's `chain` section, without Hardhat:
 * a provider, named wallets, transactions and deployments recorded as evidence (`chain.tx`,
 * `chain.deploy`). One instance per World; the provider is created on first use.
 */
export class Chain {
  /** The chain ID the profile expects. */
  readonly chainId: number;
  private readonly section: ResolvedChainSection;
  private readonly profileName: string;
  private readonly evidence: Pick<Evidence, 'record'>;
  private readonly pollingIntervalMs: number;
  private providerInstance: JsonRpcProvider | undefined;
  private readonly walletsByRole = new Map<string, EngineWallet>();
  private accounts: Record<string, HDNodeWallet> | undefined;
  private rolesByAddress: Map<string, string> | undefined;
  private connection: Promise<void> | undefined;

  constructor(options: ChainOptions) {
    this.section = options.section;
    this.profileName = options.profileName;
    this.evidence = options.evidence;
    this.pollingIntervalMs = options.pollingIntervalMs ?? DEFAULT_POLLING_INTERVAL_MS;
    this.chainId = options.section.chainId.value;
  }

  /** JSON-RPC provider of the profile. The network is fixed to `chainId`: it is never auto-detected. */
  get provider(): JsonRpcProvider {
    this.providerInstance ??= new JsonRpcProvider(this.section.rpcUrl.value, Network.from(this.chainId), {
      staticNetwork: true,
      pollingInterval: this.pollingIntervalMs,
    });
    return this.providerInstance;
  }

  /** The wallet roles of the profile (`alice` … `eve` by default). */
  roles(): string[] {
    return Object.keys(this.section.wallets);
  }

  /**
   * Wallet of a role, derived from the profile mnemonic. Transactions it sends get the gas buffer
   * and wait for the chain ID check.
   */
  wallet(role: string): EngineWallet {
    const existing = this.walletsByRole.get(role);
    if (existing) return existing;
    const account = this.accountsOfRoles()[role];
    if (!account) {
      throw new Error(`unknown wallet role '${role}'. Known roles: ${this.roles().join(', ')}`);
    }
    const wallet = new EngineWallet(role, account.privateKey, this.provider, this.section.gas, () =>
      this.verifyConnection(),
    );
    this.walletsByRole.set(role, wallet);
    return wallet;
  }

  /**
   * Checks that the RPC is reachable and reports the profile's chain ID. Engine wallets run it
   * before their first transaction, so nothing is sent to the wrong network. A success is kept for
   * the scenario; a failure is not, so the next transaction checks again.
   */
  verifyConnection(): Promise<void> {
    this.connection ??= this.checkChainId().catch((error: unknown) => {
      this.connection = undefined;
      throw error;
    });
    return this.connection;
  }

  /**
   * Waits for a transaction, records it as `chain.tx` evidence and returns its receipt. A reverted
   * transaction is recorded, then fails the step; so is a transaction rejected before broadcast
   * (e.g. a failed gas estimation).
   *
   * @param tx what an ethers contract call or `signer.sendTransaction` returns
   */
  async send(
    tx: Promise<TransactionResponse> | TransactionResponse,
    options: SendOptions = {},
  ): Promise<TransactionReceipt> {
    let response: TransactionResponse;
    try {
      response = await tx;
    } catch (error) {
      this.record('chain.tx', { chainId: this.chainId, status: 'rejected', error: errorMessage(error) }, options);
      throw new Error(`${described('transaction', options.label)} was rejected: ${errorMessage(error)}`, {
        cause: error,
      });
    }
    const receipt = await this.waitForReceipt(
      'chain.tx',
      response.hash,
      { from: this.account(response.from) },
      options,
    );
    const status = receipt.status === 1 ? 'success' : 'reverted';
    this.record(
      'chain.tx',
      {
        chainId: this.chainId,
        hash: receipt.hash,
        from: this.account(receipt.from),
        to: receipt.to,
        blockNumber: receipt.blockNumber,
        gasUsed: receipt.gasUsed.toString(),
        status,
      },
      options,
    );
    if (status !== 'success') {
      throw new Error(
        `${described('transaction', options.label)} reverted: ${receipt.hash} (block ${receipt.blockNumber})`,
      );
    }
    return receipt;
  }

  /**
   * Deploys a contract from a compiled artifact, records it as `chain.deploy` evidence and returns
   * the contract connected to the deployer's wallet.
   */
  async deploy(artifact: ContractArtifact, args: unknown[], options: DeployOptions): Promise<Contract> {
    const wallet = this.wallet(options.from);
    const deployer = { role: wallet.role, address: wallet.address };
    let hash: string;
    try {
      const contract = await new ContractFactory(artifact.abi, artifact.bytecode, wallet).deploy(...args);
      hash = contract.deploymentTransaction()!.hash;
    } catch (error) {
      this.record(
        'chain.deploy',
        {
          chainId: this.chainId,
          contract: artifact.contractName,
          deployer,
          status: 'rejected',
          error: errorMessage(error),
        },
        options,
      );
      throw new Error(
        `${described(`deployment of ${artifact.contractName}`, options.label)} was rejected: ${errorMessage(error)}`,
        {
          cause: error,
        },
      );
    }
    const receipt = await this.waitForReceipt(
      'chain.deploy',
      hash,
      { contract: artifact.contractName, deployer },
      options,
    );
    const status = receipt.status === 1 && receipt.contractAddress ? 'success' : 'reverted';
    this.record(
      'chain.deploy',
      {
        chainId: this.chainId,
        contract: artifact.contractName,
        address: receipt.contractAddress,
        hash,
        deployer,
        blockNumber: receipt.blockNumber,
        gasUsed: receipt.gasUsed.toString(),
        status,
      },
      options,
    );
    if (status !== 'success') {
      throw new Error(`${described(`deployment of ${artifact.contractName}`, options.label)} reverted: ${hash}`);
    }
    return new Contract(receipt.contractAddress!, artifact.abi, wallet);
  }

  /** Reads a Hardhat artifact JSON file (`contractName`, `abi`, `bytecode`). */
  loadArtifact(filePath: string): ContractArtifact {
    return loadArtifact(filePath);
  }

  /** Resolves once the chain reaches `blockNumber`; fails after `timeoutMs` (default 60 s). */
  async waitForBlock(blockNumber: number | bigint, timeoutMs = DEFAULT_WAIT_FOR_BLOCK_TIMEOUT_MS): Promise<number> {
    await this.verifyConnection();
    const deadline = Date.now() + timeoutMs;
    for (;;) {
      const current = await this.provider.getBlockNumber();
      if (BigInt(current) >= BigInt(blockNumber)) return current;
      if (Date.now() >= deadline) {
        throw new Error(`block ${blockNumber} not reached within ${timeoutMs} ms (current block ${current})`);
      }
      await this.sleep(this.pollingIntervalMs);
    }
  }

  sleep(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }

  /** Closes the provider. Called by the engine after each scenario. */
  destroy(): void {
    this.providerInstance?.destroy();
    this.providerInstance = undefined;
  }

  private get displayUrl(): string {
    return this.section.rpcUrl.secret ? '[REDACTED:chain.rpcUrl]' : this.section.rpcUrl.value;
  }

  private async checkChainId(): Promise<void> {
    let reported: string;
    try {
      reported = String(await this.provider.send('eth_chainId', []));
    } catch (error) {
      throw new Error(
        `profile '${this.profileName}': cannot reach the chain RPC at ${this.displayUrl}: ${errorMessage(error)}`,
        {
          cause: error,
        },
      );
    }
    if (BigInt(reported) !== BigInt(this.chainId)) {
      throw new Error(
        `profile '${this.profileName}' expects chain ID ${this.chainId}, but the RPC at ${this.displayUrl} reports ${BigInt(reported)}; no transaction was sent`,
      );
    }
  }

  /**
   * Waits for the receipt of a broadcast transaction. If the wait fails (RPC lost, provider closed),
   * the transaction is recorded with status `unknown` and its hash before the error is rethrown.
   */
  private async waitForReceipt(
    kind: string,
    hash: string,
    data: Record<string, unknown>,
    options: SendOptions,
  ): Promise<TransactionReceipt> {
    try {
      // Unlike `response.wait()`, this does not throw on a reverted receipt, so it can be recorded.
      const receipt = await this.provider.waitForTransaction(hash, 1);
      if (!receipt) throw new Error('no receipt returned');
      return receipt;
    } catch (error) {
      this.record(
        kind,
        { chainId: this.chainId, hash, ...data, status: 'unknown', error: errorMessage(error) },
        options,
      );
      throw new Error(`could not get the receipt of transaction ${hash}: ${errorMessage(error)}`, { cause: error });
    }
  }

  private accountsOfRoles(): Record<string, HDNodeWallet> {
    this.accounts ??= deriveAccounts(this.section.mnemonic.value, this.section.hdPath, this.section.wallets);
    return this.accounts;
  }

  private account(address: string): EvidenceAccount {
    this.rolesByAddress ??= new Map(
      Object.entries(this.accountsOfRoles()).map(([role, account]) => [account.address.toLowerCase(), role]),
    );
    const role = this.rolesByAddress.get(address.toLowerCase());
    return role ? { role, address } : { address };
  }

  private record(kind: string, data: Record<string, unknown>, options: SendOptions): void {
    this.evidence.record(kind, data, options.label !== undefined ? { label: options.label } : {});
  }
}
