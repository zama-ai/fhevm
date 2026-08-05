import { expect, test } from 'bun:test';

import {
  hostContractUpgradeOrder,
  phaseOrder,
  resolveRolloutTestMode,
  rolloutPhaseTestProfiles,
} from '../rollouts/v0.13-to-v0.14/run';
import {
  connectorKeys,
  from,
  hostContractsTargetTag,
  phaseVersions,
  scenario,
  targetTag,
  testSuiteTargetTag,
  to,
} from '../rollouts/v0.13-to-v0.14/versions';

test('rehearses the upgrade on a multi-chain topology with consensus-capable coprocessors', () => {
  // The canonical ProtocolConfig mirror needs a second host chain to mirror onto, and the
  // drift/consensus checks need 3 coprocessors at threshold 2.
  expect(scenario).toBe('two-of-three-multi-chain');
});

test('keeps the kms-core PRSS bridge as a mandatory stop between 0.13.20 and 0.14', () => {
  expect(phaseVersions.baseline.CORE_VERSION).toBe('v0.13.20');
  expect(phaseVersions.kmsPrssBridge.CORE_VERSION).toBe('v0.13.22');
  expect(phaseVersions.kms.CORE_VERSION).toBe('v0.14.0-1');
  // A cluster must never jump straight from the pre-hotfix to the post-hotfix core.
  expect(phaseVersions.relayer.CORE_VERSION).toBe(phaseVersions.baseline.CORE_VERSION);
});

test('moves kms-core alone in the PRSS bridge phase, leaving every connector on 0.13', () => {
  for (const key of connectorKeys) {
    expect(phaseVersions.kmsPrssBridge[key]).toBe(from[key]);
  }
  // ...and moves them with the core only in the kms phase.
  for (const key of connectorKeys) {
    expect(phaseVersions.kms[key]).toBe(to[key]);
  }
});

test('upgrades the gateway chain strictly before the host chain', () => {
  expect(phaseVersions.gatewayContracts.GATEWAY_VERSION).toBe(to.GATEWAY_VERSION);
  expect(phaseVersions.gatewayContracts.HOST_VERSION).toBe(from.HOST_VERSION);
  expect(phaseVersions.hostContracts.HOST_VERSION).toBe(to.HOST_VERSION);
});

test('splits listener-core from the coprocessor image bump', () => {
  expect(phaseVersions.listenerCore.LISTENER_CORE_VERSION).toBe(to.LISTENER_CORE_VERSION);
  expect(phaseVersions.listenerCore.COPROCESSOR_DB_MIGRATION_VERSION).toBe(from.COPROCESSOR_DB_MIGRATION_VERSION);
  expect(phaseVersions.coprocessor.COPROCESSOR_DB_MIGRATION_VERSION).toBe(to.COPROCESSOR_DB_MIGRATION_VERSION);
});

test('runs the harness at the target tag from the first phase and leaves the SDK on @fhevm/sdk', () => {
  expect(phaseVersions.baseline.TEST_SUITE_VERSION).toBe(to.TEST_SUITE_VERSION);
  // An unset RELAYER_SDK_VERSION is what selects @fhevm/sdk in the harness image.
  expect(from).not.toHaveProperty('RELAYER_SDK_VERSION');
});

test('pins host-contracts and the harness one tag back, where images actually exist', () => {
  // fhevm publishes images only for the components a tag touched. v0.14.0-10 has no
  // host-contracts or test-suite/e2e image, so pinning them there boots an unpullable ref.
  expect(to.HOST_VERSION).toBe(hostContractsTargetTag);
  expect(to.TEST_SUITE_VERSION).toBe(testSuiteTargetTag);
  expect(to.HOST_VERSION).not.toBe(targetTag);
  // Everything else does publish at the newest pre-release.
  expect(to.GATEWAY_VERSION).toBe(targetTag);
  expect(to.RELAYER_VERSION).toBe(targetTag);
  expect(to.COPROCESSOR_TFHE_WORKER_VERSION).toBe(targetTag);
  expect(to.LISTENER_CORE_VERSION).toBe(targetTag);
});

test('keeps every phase lock cumulative', () => {
  const ordered = [
    phaseVersions.baseline,
    phaseVersions.gatewayContracts,
    phaseVersions.hostContracts,
    phaseVersions.relayer,
    phaseVersions.kms,
    phaseVersions.listenerCore,
    phaseVersions.coprocessor,
  ];
  // Once a key reaches its target it never goes back. The PRSS bridge phase is excluded
  // because its CORE_VERSION is deliberately an intermediate, not a target.
  for (let index = 1; index < ordered.length; index += 1) {
    for (const [key, value] of Object.entries(ordered[index - 1])) {
      if (value === to[key as keyof typeof to]) {
        expect(ordered[index][key]).toBe(value);
      }
    }
  }
});

test('ends every phase on the target versions', () => {
  expect(phaseVersions.coprocessor).toEqual(to);
});

test('orders host contract upgrades so verification and limits land before the executor', () => {
  const position = (name: string) => hostContractUpgradeOrder.indexOf(name);
  expect(position('KMSVerifier')).toBeLessThan(position('FHEVMExecutor'));
  expect(position('HCULimit')).toBeLessThan(position('FHEVMExecutor'));
  expect(position('ACL')).toBe(hostContractUpgradeOrder.length - 1);
});

test('gates every phase on rollout-standard by default', () => {
  expect(resolveRolloutTestMode(undefined)).toBe('rollout-standard');
  for (const phase of phaseOrder) {
    expect(rolloutPhaseTestProfiles(phase, 'rollout-standard')).toEqual(['rollout-standard']);
  }
});

test('covers multi-chain isolation in heavy mode wherever contracts moved', () => {
  expect(rolloutPhaseTestProfiles('host-contracts', 'rollout-heavy')).toContain('multi-chain-isolation');
  expect(rolloutPhaseTestProfiles('final', 'rollout-heavy')).toContain('multi-chain-isolation');
});

test('rejects unsupported rollout test modes', () => {
  expect(() => resolveRolloutTestMode('standard')).toThrow('Unsupported ROLLOUT_TEST_PROFILE=standard');
});
