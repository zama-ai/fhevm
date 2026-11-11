/**
 * Shared code of the FHE fixture scenarios, which run against the local stack (see
 * test/integration/fhe/run.sh). The contracts are the engine's own, deployed with the profile's
 * coprocessor config.
 */
import { type EncryptedInput, EngineWorld } from '#engine';
import type { Contract } from 'ethers';

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

  /** Deploys one of the engine's contracts from `role`. */
  async deploy(contractName: string, role: string): Promise<void> {
    const { chain, contracts } = this.engine;
    this.contract = await chain.deploy(contracts.artifact(contractName), [contracts.coprocessorConfig()], {
      from: role,
      label: `Deploy ${contractName}`,
    });
  }

  /** Reads a handle from a contract getter. */
  async handle(getter: string): Promise<string> {
    return String(await this.deployed.getFunction(getter)());
  }
}
