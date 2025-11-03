import path from 'path';

import { BaseStep, DeploymentContext, StepExecutionResult } from './base-step.js';
import { ValidationError } from '../utils/errors.js';
import { TaskOutputReader } from '../utils/task-output-reader.js';
import { resolveProjectRoot } from '../utils/project-paths.js';

export class Step07HostContracts extends BaseStep {
  public readonly id = 'step-07';
  public readonly name = 'Deploy Host Contracts on Ethereum';
  public readonly description =
    'Deploys host contract proxies, initializes implementations, and configures pausers on Ethereum.';
  public readonly dependencies = ['step-06'] as const;
  public readonly pkgName = 'host-contracts' as const;

  protected async execute(ctx: DeploymentContext): Promise<StepExecutionResult> {
    const ethereum = ctx.networks.getEthereum();
    const gateway = ctx.networks.getGateway();
    const deployerPk = ctx.env.resolveWalletPrivateKey('deployer');

    // Do RPC call to Gateway rpcUrl to get the chain id
    const response = await fetch(gateway.rpcUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        method: 'eth_chainId',
        params: [],
        id: 1,
        jsonrpc: '2.0'
      })
    });
    const data = await response.json();
    const gatewayChainId = data.result;
    if (!gatewayChainId) {
      throw new ValidationError('Failed to get chain id from Gateway RPC URL');
    }

    // Build environment variables from config
    const baseEnvVars: Record<string, string> = {
      DEPLOYER_PRIVATE_KEY: deployerPk,
      RPC_URL: ethereum.rpcUrl,
      CHAIN_ID_GATEWAY: gatewayChainId.toString(),
      DECRYPTION_ADDRESS: ctx.env.getAddress('DECRYPTION')!,
      INPUT_VERIFICATION_ADDRESS: ctx.env.getAddress('INPUT_VERIFICATION')!,
    };

    const thresholds = ctx.config.protocol.thresholds;
    if (thresholds.public_decryption !== undefined) {
      baseEnvVars.PUBLIC_DECRYPTION_THRESHOLD = thresholds.public_decryption.toString();
    }
    if (thresholds.coprocessor !== undefined) {
      baseEnvVars.COPROCESSOR_THRESHOLD = thresholds.coprocessor.toString();
    }

    const kmsNodes = ctx.config.protocol.kms_nodes;
    baseEnvVars.NUM_KMS_NODES = kmsNodes.length.toString();
    kmsNodes.forEach((node, idx) => {
      baseEnvVars[`KMS_SIGNER_ADDRESS_${idx}`] = node.signer_address;
    });

    const coprocessors = ctx.config.protocol.coprocessors;
    baseEnvVars.NUM_COPROCESSORS = coprocessors.length.toString();
    coprocessors.forEach((coprocessor, idx) => {
      baseEnvVars[`COPROCESSOR_SIGNER_ADDRESS_${idx}`] = coprocessor.signer_address;
    });

    const baseEnv = ctx.env.buildTaskEnv(baseEnvVars);

    const pauserAddresses = ctx.config.operators
      .map((operator) => operator.pauser?.address)
      .filter((value): value is string => Boolean(value));

    if (pauserAddresses.length === 0) {
      throw new ValidationError('At least one pauser address is required for Step 7.');
    }

    await ctx.hardhat.runTask({
      pkg: this.pkgName,
      task: 'task:deployAllHostContracts',
      args: ['--network', ethereum.hostPkgName!],
      env: baseEnv
    });

    // Extract addresses from .env.host and .env.decryptionoracle files (written by deployment tasks)
    const projectRoot = resolveProjectRoot();
    const reader = new TaskOutputReader(projectRoot);
    const hostEnvPath = path.join(projectRoot, this.pkgName, 'addresses', '.env.host');
    const decryptionOracleEnvPath = path.join(
      projectRoot,
      this.pkgName,
      'addresses',
      '.env.decryptionoracle'
    );

    const hostAddresses = reader.readEnvFile(hostEnvPath, {
      'ACL_CONTRACT_ADDRESS': 'ACL_ADDRESS',
      'FHEVM_EXECUTOR_CONTRACT_ADDRESS': 'FHEVM_EXECUTOR_ADDRESS',
      'KMS_VERIFIER_CONTRACT_ADDRESS': 'KMS_VERIFIER_ADDRESS',
      'INPUT_VERIFIER_CONTRACT_ADDRESS': 'INPUT_VERIFIER_ADDRESS',
      'HCU_LIMIT_CONTRACT_ADDRESS': 'HCU_LIMIT_ADDRESS',
      'PAUSER_SET_CONTRACT_ADDRESS': 'PAUSER_SET_HOST'
    });

    const decryptionOracleAddresses = reader.readEnvFile(decryptionOracleEnvPath, {
      'DECRYPTION_ORACLE_ADDRESS': 'DECRYPTION_ORACLE_ADDRESS'
    });

    const addressMap = { ...hostAddresses, ...decryptionOracleAddresses };

    const pauserEnv: Record<string, string> = {
      NUM_PAUSERS: pauserAddresses.length.toString(),
      PAUSER_SET_CONTRACT_ADDRESS: hostAddresses.PAUSER_SET_HOST!
    };

    pauserAddresses.forEach((address, idx) => {
      pauserEnv[`PAUSER_ADDRESS_${idx}`] = address;
    });


    // Configure pausers
    await ctx.hardhat.runTask({
      pkg: this.pkgName,
      task: 'task:addHostPausers',
      args: ['--network', ethereum.hostPkgName!],
      env: {
        ...baseEnv,
        ...pauserEnv,
      },
    });

    for (const [key, value] of Object.entries(addressMap)) {
      ctx.env.recordAddress(key, value, this.id);
    }

    // Verify host contracts if auto verification is enabled
    if (ctx.config.options.auto_verify_contracts) {
      ctx.logger.info('Verifying host contracts...');
      try {
        await ctx.hardhat.runTask({
          pkg: this.pkgName,
          task: 'task:verifyAllHostContracts',
          args: ['--network', ethereum.hostPkgName!, '--useInternalProxyAddress', 'true'],
          env: baseEnv,
        });
        ctx.logger.success('Host contracts verified successfully');
      } catch (error) {
        ctx.logger.warn('Host contracts verification failed (this may be acceptable if already verified)');
        ctx.logger.error(error instanceof Error ? error.message : String(error));

      }
    }

    return {
      status: 'completed',
      addresses: addressMap
    };
  }

}
