/**
 * Shared code of the FHE fixture scenarios, which run against the local stack (see
 * test/integration/fhe/run.sh). The contract artifacts are those of the e2e image: their protocol
 * addresses are compiled in, and the image's match the local stack's.
 */
import { type EncryptedInput, EngineWorld, loadArtifact } from '#engine';
import type { Contract } from 'ethers';
import path from 'node:path';

/** Scenario state shared by the FHE fixtures. A fresh instance is created for every Gherkin scenario. */
export class FheFixtureWorld extends EngineWorld {
  contract: Contract | undefined;
  input: EncryptedInput | undefined;

  /** The deployed contract; a step that needs it before the deployment step is a fixture bug. */
  get deployed(): Contract {
    if (!this.contract) throw new Error('no contract deployed yet');
    return this.contract;
  }

  /** The encrypted input of an earlier step. */
  get encrypted(): EncryptedInput {
    if (!this.input) throw new Error('no input encrypted yet');
    return this.input;
  }

  /** Deploys a contract of the e2e image (no constructor arguments) from `role`. */
  async deploy(contractName: string, role: string): Promise<void> {
    const artifact = loadArtifact(path.join(this.engine.scenario.directory, '..', 'artifacts', `${contractName}.json`));
    this.contract = await this.engine.chain.deploy(artifact, [], { from: role, label: `Deploy ${contractName}` });
  }

  /** Reads a handle from a contract getter. */
  async handle(getter: string): Promise<string> {
    return String(await this.deployed.getFunction(getter)());
  }
}
