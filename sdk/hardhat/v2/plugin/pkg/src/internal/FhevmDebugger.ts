import { ethers as EthersT } from 'ethers';

import { HardhatFhevmError } from '../error';
import type { HardhatFhevmRuntimeDebugger } from '../types';
import type { FhevmEnvironment } from './FhevmEnvironment';
import { FhevmType, type FhevmTypeEuint, isFhevmEuint } from './fheType';
import { assertHandleIsInitialized, parseFhevmHandle } from './fhevmHandle';
import type { FhevmHandleCoder } from './migration/placeholders';

/**
 * `fhevm.debugger` — reads cleartexts straight out of the chain, with no ACL check.
 *
 * This is the test-only escape hatch: `userDecrypt`/`publicDecrypt` deliberately enforce permissions,
 * but an operator test wants to assert that `FheAdd(a, b)` produced the right number without first
 * arranging for anyone to be allowed to see it.
 *
 * Under the old JavaScript mock engine that meant asking the in-process coprocessor over a custom
 * `fhevm_get_clear_text` RPC. The cleartext stack keeps the same values on-chain in `CleartextDB`, so
 * the debugger now just reads them — same capability, one `eth_call` instead of a fake relayer.
 */
export class FhevmDebugger implements HardhatFhevmRuntimeDebugger {
  readonly #fhevmEnv: FhevmEnvironment;

  constructor(fhevmEnv: FhevmEnvironment) {
    this.#fhevmEnv = fhevmEnv;
  }

  /**
   * `CleartextDB` only exists on the local cleartext stack. On a public network the values are
   * genuinely encrypted, so there is nothing for the debugger to read and saying so plainly beats
   * failing later inside an `eth_call`.
   *
   * The contract comes from the cleartext repository — which is the only repository that has one — so
   * both its address and its ABI track `@fhevm/host-contracts-cleartext` rather than being restated
   * here.
   */
  #db(): EthersT.Contract {
    if (!this.#fhevmEnv.cleartextProvider.isCleartext) {
      throw new HardhatFhevmError(
        `fhevm.debugger is only available on a cleartext network — on '${this.#fhevmEnv.cleartextProvider.info.networkName}' values are really encrypted. Use fhevm.userDecryptE*() or fhevm.publicDecryptE*() instead.`,
      );
    }
    return this.#fhevmEnv.getCleartextContractsRepository().cleartextDb.readonlyContract;
  }

  /** The raw cleartext behind a handle, after checking the handle really is of the expected type. */
  async #read(handleBytes32: EthersT.BigNumberish, expected: FhevmType, method: string): Promise<bigint> {
    const handle = EthersT.toBeHex(handleBytes32, 32);
    assertHandleIsInitialized(handle);

    const info = parseFhevmHandle(handle);
    if (info.fhevmType !== expected) {
      throw new HardhatFhevmError(
        `fhevm.debugger.${method}: handle '${handle}' is a ${info.typeName}, not a ${FhevmType[expected]}.`,
      );
    }

    // `getFunction` rather than `.get`: the latter comes off Contract's index signature, so it reads
    // as possibly-undefined. The ABI declares a uint256, and the result is checked rather than cast.
    const value: unknown = await this.#db().getFunction('get')(handle);
    if (typeof value !== 'bigint') {
      throw new HardhatFhevmError(`fhevm.debugger.${method}: CleartextDB.get('${handle}') did not return a uint256.`);
    }
    return value;
  }

  public async decryptEbool(handleBytes32: EthersT.BigNumberish): Promise<boolean> {
    return (await this.#read(handleBytes32, FhevmType.ebool, 'decryptEbool')) === 1n;
  }

  public async decryptEuint(fhevmType: FhevmTypeEuint, handleBytes32: EthersT.BigNumberish): Promise<bigint> {
    if (!isFhevmEuint(fhevmType)) {
      throw new HardhatFhevmError(`fhevm.debugger.decryptEuint: expected an euint type.`);
    }
    return await this.#read(handleBytes32, fhevmType, 'decryptEuint');
  }

  public async decryptEaddress(handleBytes32: EthersT.BigNumberish): Promise<`0x${string}`> {
    const value = await this.#read(handleBytes32, FhevmType.eaddress, 'decryptEaddress');
    return EthersT.getAddress(EthersT.toBeHex(value, 20)) as `0x${string}`;
  }

  public createHandleCoder(): FhevmHandleCoder {
    throw new HardhatFhevmError(
      `fhevm.debugger.createHandleCoder() is not implemented yet. See plans/MIGRATION_TO_FHEVM_SDK_CLEARTEXT.md.`,
    );
  }

  // eslint-disable-next-line @typescript-eslint/require-await
  public async createDecryptionSignatures(
    _handlesBytes32Hex: string[],
    _clearTextValues: Array<bigint | string | boolean>,
  ): Promise<string[]> {
    throw new HardhatFhevmError(
      `fhevm.debugger.createDecryptionSignatures() is not implemented yet: it forged KMS signatures through the JavaScript mock engine. See plans/MIGRATION_TO_FHEVM_SDK_CLEARTEXT.md.`,
    );
  }
}
