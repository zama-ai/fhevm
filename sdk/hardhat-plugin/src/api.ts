import { type Signer, getAddress, hexlify } from "ethers";

import { formatDecrypted, generateKeypair, publicDecrypt, userDecrypt } from "./engine/fhe/decrypt";
import { type EncryptValue, encryptInput } from "./engine/fhe/encrypt";
import { FheType } from "./engine/fhe/fhetype";
import { type Eip1193Provider, FhevmNode } from "./engine/node";
import { ADDRESSES } from "./engine/stack/addresses";
import { defaultSigners } from "./engine/stack/config";
import { type DeployedEngine, assertEngineReady, deployCleartextEngine, isPlaced } from "./engine/stack/deploy";
import type {
  EncryptedInputResult,
  FhevmTypeEuint,
  HardhatFhevmRuntimeEnvironment,
  RelayerEncryptedInput,
} from "./types";

const HANDLE_VERSION = 0;

class EncryptedInputBuilder implements RelayerEncryptedInput {
  private readonly values: EncryptValue[] = [];

  constructor(
    private readonly api: FhevmApi,
    private readonly contractAddress: string,
    private readonly userAddress: string,
  ) {}

  addBool(value: boolean | number | bigint): RelayerEncryptedInput {
    this.values.push({ type: FheType.ebool, value: value ? 1n : 0n });
    return this;
  }
  add8(value: number | bigint): RelayerEncryptedInput {
    return this.push(FheType.euint8, value);
  }
  add16(value: number | bigint): RelayerEncryptedInput {
    return this.push(FheType.euint16, value);
  }
  add32(value: number | bigint): RelayerEncryptedInput {
    return this.push(FheType.euint32, value);
  }
  add64(value: number | bigint): RelayerEncryptedInput {
    return this.push(FheType.euint64, value);
  }
  add128(value: number | bigint): RelayerEncryptedInput {
    return this.push(FheType.euint128, value);
  }
  add256(value: number | bigint): RelayerEncryptedInput {
    return this.push(FheType.euint256, value);
  }
  addAddress(value: string): RelayerEncryptedInput {
    this.values.push({ type: FheType.eaddress, value: getAddress(value) });
    return this;
  }

  private push(type: FheType, value: number | bigint): RelayerEncryptedInput {
    this.values.push({ type, value: BigInt(value) });
    return this;
  }

  async encrypt(): Promise<EncryptedInputResult> {
    const engine = await this.api.ensureDeployed();
    const result = await encryptInput({
      values: this.values,
      aclAddress: engine.addresses.ACL,
      contractAddress: this.contractAddress,
      userAddress: this.userAddress,
      hostChainId: engine.node.chainId,
      handleVersion: HANDLE_VERSION,
      coprocessorSigners: [engine.signers.coprocessor],
      coprocessorThreshold: 1,
    });
    return { handles: result.handles.map((h) => hexlify(h)), inputProof: hexlify(result.inputProof) };
  }
}

/**
 * The `hre.fhevm` implementation. Hardhat-free by design: it takes an EIP-1193 provider and is handed
 * ethers `Signer`s by the caller. The Hardhat glue (extendEnvironment) lives in `index.ts`, so the HH2→
 * HH3 move is confined there.
 */
export class FhevmApi implements HardhatFhevmRuntimeEnvironment {
  readonly isMock: boolean;
  private readonly provider: Eip1193Provider;
  private engine: DeployedEngine | undefined;
  private deploying: Promise<DeployedEngine> | undefined;

  constructor(params: { provider: Eip1193Provider; isMock: boolean }) {
    this.provider = params.provider;
    this.isMock = params.isMock;
  }

  /**
   * Deploys the cleartext engine on first use and caches it. Idempotent across a persistent chain: if the
   * executor is already present and initialized (`getKmsSigners()` non-empty), it adopts the running
   * engine instead of re-initializing — a re-init on an already-initialized chain would revert.
   */
  async ensureDeployed(): Promise<DeployedEngine> {
    if (this.engine) {
      return this.engine;
    }
    // Coalesce concurrent first-use calls into a single deploy.
    if (!this.deploying) {
      this.deploying = this.deployOrAdopt();
    }
    this.engine = await this.deploying;
    return this.engine;
  }

  private async deployOrAdopt(): Promise<DeployedEngine> {
    const node = await FhevmNode.create(this.provider);
    if (await isPlaced(node, "FHEVMExecutor")) {
      const signers = defaultSigners();
      try {
        await assertEngineReady(node, signers);
        return { node, signers, addresses: ADDRESSES }; // adopt the already-initialized engine
      } catch {
        // present but not fully initialized — fall through to a full deploy
      }
    }
    return deployCleartextEngine(this.provider);
  }

  async initializeCLIApi(): Promise<void> {
    await this.ensureDeployed();
  }

  createEncryptedInput(contractAddress: string, userAddress: string): RelayerEncryptedInput {
    return new EncryptedInputBuilder(this, getAddress(contractAddress), getAddress(userAddress));
  }

  private async decryptOne(handleBytes32: string, contractAddress: string, user: Signer): Promise<bigint> {
    const engine = await this.ensureDeployed();
    const [value] = await userDecrypt(engine.node, {
      pairs: [{ handle: handleBytes32, contractAddress }],
      user,
      keypair: generateKeypair(),
    });
    return value;
  }

  async userDecryptEuint(
    _fhevmType: FhevmTypeEuint,
    handleBytes32: string,
    contractAddress: string,
    user: Signer,
  ): Promise<bigint> {
    return this.decryptOne(handleBytes32, contractAddress, user);
  }

  async userDecryptEbool(handleBytes32: string, contractAddress: string, user: Signer): Promise<boolean> {
    return formatDecrypted(await this.decryptOne(handleBytes32, contractAddress, user), FheType.ebool) as boolean;
  }

  async userDecryptEaddress(handleBytes32: string, contractAddress: string, user: Signer): Promise<string> {
    return formatDecrypted(await this.decryptOne(handleBytes32, contractAddress, user), FheType.eaddress) as string;
  }

  async publicDecryptEuint(_fhevmType: FhevmTypeEuint, handleBytes32: string): Promise<bigint> {
    const engine = await this.ensureDeployed();
    const [value] = await publicDecrypt(engine.node, [handleBytes32]);
    return value;
  }

  async publicDecryptEbool(handleBytes32: string): Promise<boolean> {
    const engine = await this.ensureDeployed();
    const [value] = await publicDecrypt(engine.node, [handleBytes32]);
    return value === 1n;
  }
}
