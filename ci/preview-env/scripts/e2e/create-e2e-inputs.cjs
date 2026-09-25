// Shared image selection and registry checks for CPU and GPU E2E orchestration.
const mappings = [
  [
    'coprocessor-docker-build',
    'db_migration_build_result',
    'coprocessor-db-migration-version',
    'fhevm/coprocessor/db-migration',
    'COPROCESSOR_DB_MIGRATION_VERSION',
  ],
  [
    'coprocessor-docker-build',
    'gw_listener_build_result',
    'coprocessor-gw-listener-version',
    'fhevm/coprocessor/gw-listener',
    'COPROCESSOR_GW_LISTENER_VERSION',
  ],
  [
    'coprocessor-docker-build',
    'host_listener_build_result',
    'coprocessor-host-listener-version',
    'fhevm/coprocessor/host-listener',
    'COPROCESSOR_HOST_LISTENER_VERSION',
  ],
  [
    'coprocessor-docker-build',
    'sns_worker_build_result',
    'coprocessor-sns-worker-version',
    'fhevm/coprocessor/sns-worker',
    'COPROCESSOR_SNS_WORKER_VERSION',
  ],
  [
    'coprocessor-docker-build',
    'tfhe_worker_build_result',
    'coprocessor-tfhe-worker-version',
    'fhevm/coprocessor/tfhe-worker',
    'COPROCESSOR_TFHE_WORKER_VERSION',
  ],
  [
    'coprocessor-docker-build',
    'tx_sender_build_result',
    'coprocessor-tx-sender-version',
    'fhevm/coprocessor/tx-sender',
    'COPROCESSOR_TX_SENDER_VERSION',
  ],
  [
    'coprocessor-docker-build',
    'zkproof_worker_build_result',
    'coprocessor-zkproof-worker-version',
    'fhevm/coprocessor/zkproof-worker',
    'COPROCESSOR_ZKPROOF_WORKER_VERSION',
  ],
  [
    'coprocessor-docker-build',
    'consensus_detector_build_result',
    'coprocessor-consensus-detector-version',
    'fhevm/coprocessor/consensus-detector',
    'COPROCESSOR_CONSENSUS_DETECTOR_VERSION',
  ],
  [
    'coprocessor-docker-build',
    'upgrade_controller_build_result',
    'coprocessor-upgrade-controller-version',
    'fhevm/coprocessor/upgrade-controller',
    'COPROCESSOR_UPGRADE_CONTROLLER_VERSION',
  ],
  [
    'listener-docker-build',
    'build_result',
    'listener-core-version',
    'fhevm/listener/listener-core',
    'LISTENER_CORE_VERSION',
  ],
  [
    'kms-connector-docker-build',
    'db_migration_build_result',
    'connector-db-migration-version',
    'fhevm/kms-connector/db-migration',
    'CONNECTOR_DB_MIGRATION_VERSION',
  ],
  [
    'kms-connector-docker-build',
    'gw_listener_build_result',
    'connector-gw-listener-version',
    'fhevm/kms-connector/gw-listener',
    'CONNECTOR_GW_LISTENER_VERSION',
  ],
  [
    'kms-connector-docker-build',
    'kms_worker_build_result',
    'connector-kms-worker-version',
    'fhevm/kms-connector/kms-worker',
    'CONNECTOR_KMS_WORKER_VERSION',
  ],
  [
    'kms-connector-docker-build',
    'tx_sender_build_result',
    'connector-tx-sender-version',
    'fhevm/kms-connector/tx-sender',
    'CONNECTOR_TX_SENDER_VERSION',
  ],
  [
    'kms-connector-docker-build',
    'endpoint_build_result',
    'connector-endpoint-version',
    'fhevm/kms-connector/endpoint',
    'CONNECTOR_ENDPOINT_VERSION',
  ],
  [
    'kms-connector-docker-build',
    'proxy_build_result',
    'connector-proxy-version',
    'fhevm/kms-connector/proxy',
    'CONNECTOR_PROXY_VERSION',
  ],
  ['gateway-contracts-docker-build', 'build_result', 'gateway-version', 'fhevm/gateway-contracts', 'GATEWAY_VERSION'],
  ['host-contracts-docker-build', 'build_result', 'host-version', 'fhevm/host-contracts', 'HOST_VERSION'],
  [
    'relayer-docker-build',
    'relayer_migrate_build_result',
    'relayer-migrate-version',
    'fhevm/relayer-migrate',
    'RELAYER_MIGRATE_VERSION',
  ],
  ['relayer-docker-build', 'relayer_build_result', 'relayer-version', 'fhevm/relayer', 'RELAYER_VERSION'],
  ['test-suite-docker-build', 'build_result', 'test-suite-version', 'fhevm/test-suite/e2e', 'TEST_SUITE_VERSION'],
];

const GPU_WORKERS = new Set([
  'coprocessor-tfhe-worker-version',
  'coprocessor-sns-worker-version',
  'coprocessor-zkproof-worker-version',
]);

// A supplied GPU tag replaces only the three GPU worker results. All other
// services, including the connector endpoint, use the same fail-closed policy.
function selectImages({ buildResults, headTag, gpuWorkerTag }) {
  if (gpuWorkerTag !== undefined && !gpuWorkerTag) {
    throw new Error('the GPU worker build produced no tag; nothing to test against');
  }
  const requiredJobs = new Set(mappings.map(([job]) => job));
  if (gpuWorkerTag !== undefined) requiredJobs.add('gpu-worker-build');
  for (const job of requiredJobs) {
    const result = buildResults[job]?.result ?? 'missing';
    if (result !== 'success') {
      throw new Error(`Required build job did not succeed: ${job}=${result}`);
    }
  }
  const outputs = { 'kms-core-version': '' };
  const built = [];
  const skipped = [];
  for (const [job, resultKey, outputName, repo, envKey] of mappings) {
    if (gpuWorkerTag !== undefined && GPU_WORKERS.has(outputName)) {
      outputs[outputName] = gpuWorkerTag;
      built.push({ repo, tag: gpuWorkerTag });
      continue;
    }
    const result =
      buildResults[job] && buildResults[job].outputs && buildResults[job].outputs[resultKey]
        ? buildResults[job].outputs[resultKey]
        : 'missing';
    if (result === 'success') {
      outputs[outputName] = headTag;
      built.push({ repo, tag: headTag });
    } else if (result === 'skipped') {
      outputs[outputName] = '';
      skipped.push({ envKey, repo });
    } else {
      throw new Error(`Required repo-owned build output failed: ${job}.${resultKey}=${result}`);
    }
  }

  outputs['connector-versions'] = JSON.stringify(
    Object.fromEntries(
      ['db-migration', 'gw-listener', 'kms-worker', 'tx-sender', 'endpoint', 'proxy'].map((name) => [
        name,
        outputs[`connector-${name}-version`],
      ]),
    ),
  );
  return { outputs, built, skipped };
}

module.exports = async function createE2eInputs({ core, env = process.env, gpuWorkerTag }) {
  const fs = require('fs');
  const path = require('path');
  const waitBuiltGhcrTags = require('../resolve/wait-built-ghcr-tags.cjs');
  const { assertLockGhcrTags, selectBaselineGhcrImages } = waitBuiltGhcrTags;
  const headTag = env.NEW_COMMIT_HASH.slice(0, 7);
  const buildResults = JSON.parse(env.DOCKER_BUILD_RESULTS);

  const lockDir = env.LOCK_DIR;
  const lockFile = fs.readdirSync(lockDir).find((name) => name.endsWith('.json'));
  if (!lockFile) {
    throw new Error(`Could not locate downloaded baseline lock in ${lockDir}`);
  }
  const lock = JSON.parse(fs.readFileSync(path.join(lockDir, lockFile), 'utf8'));
  if (Array.isArray(lock.sources)) {
    for (const source of lock.sources) {
      core.info(`[baseline-lock] ${source}`);
    }
    const unverified = lock.sources.filter(
      (source) => source.includes('published-image-check=skipped') || source.includes('unverified:'),
    );
    if (unverified.length) {
      throw new Error(
        `baseline lock is unverified; refusing to start e2e:\n` +
          unverified.map((source) => `  - ${source}`).join('\n'),
      );
    }
  }

  const { outputs, built, skipped } = selectImages({ buildResults, headTag, gpuWorkerTag });

  const baseline = selectBaselineGhcrImages({ lockEnv: lock.env || {}, skipped });
  await assertLockGhcrTags({
    core,
    user: env.GHCR_USER,
    token: env.GHCR_READ_TOKEN,
    images: baseline,
  });
  await waitBuiltGhcrTags({
    core,
    user: env.GHCR_USER,
    token: env.GHCR_READ_TOKEN,
    images: built,
  });
  for (const [name, value] of Object.entries(outputs)) core.setOutput(name, value);
};
module.exports.selectImages = selectImages;
