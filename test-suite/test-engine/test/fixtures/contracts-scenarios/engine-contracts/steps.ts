import type { EngineWorld } from '#engine';
import { When } from '@cucumber/cucumber';
import { ContractFactory } from 'ethers';
import assert from 'node:assert/strict';

When(
  'the scenario encodes the deployment of TestInput with the coprocessor config',
  async function (this: EngineWorld) {
    const { contracts } = this.engine;
    const artifact = contracts.artifact('TestInput');
    const config = contracts.coprocessorConfig();
    const factory = new ContractFactory(artifact.abi, artifact.bytecode);
    const deployment = await factory.getDeployTransaction(config);
    assert.ok(deployment.data.startsWith(artifact.bytecode));
    this.engine.log(`loaded ${artifact.contractName} from ${contracts.directory}; ACL ${config.ACLAddress}`);
  },
);
