// Pick each component's image tag (freshly-built short SHA vs pinned version),
// derive the preview namespace and chart/ref versions, and write the
// deployment-plan job summary. Invoked from the resolve-tags github-script step:
//   await require('./ci/preview-env/scripts/resolve-tags.cjs')({ core, context })
// Env: NEEDS, EVENT_NAME, INPUTS, ACTOR (step) + the pinned *_VERSION/*_CHART (workflow).
module.exports = async ({ core, context }) => {
  const needs = JSON.parse(process.env.NEEDS);
  const isDispatch = process.env.EVENT_NAME === 'workflow_dispatch';
  const inputs = JSON.parse(process.env.INPUTS);
  const env = process.env;

  // PR head SHA on pull_request; picked branch tip on dispatch.
  const shortSha = isDispatch
    ? context.sha.substring(0, 7)
    : context.payload.pull_request.head.sha.substring(0, 7);

  // 'success' = this run built+pushed shortSha; else fall back to the pin.
  const pick = (result, pinnedVersion) => (result === 'success' ? shortSha : pinnedVersion);

  // Pinned version: env on pull_request, matching input on dispatch.
  const pinned = (envVar, dispatchInput) => (isDispatch ? dispatchInput : envVar);
  const hostContractsPin = pinned(env.HOST_CONTRACTS_VERSION, inputs.host_contracts_version);
  const gatewayContractsPin = pinned(env.GATEWAY_CONTRACTS_VERSION, inputs.gateway_contracts_version);
  const kmsConnectorPin = pinned(env.KMS_CONNECTOR_VERSION, inputs.kms_connector_version);
  const coprocessorPin = pinned(env.COPROCESSOR_VERSION, inputs.coprocessor_version);
  const relayerPin = pinned(env.RELAYER_VERSION, inputs.relayer_version);
  const testSuitePin = pinned(env.TEST_SUITE_VERSION, inputs.test_suite_version);

  const tags = {
    host_contracts: pick(needs['build-host-contracts'].outputs.build_result, hostContractsPin),
    gateway_contracts: pick(needs['build-gateway-contracts'].outputs.build_result, gatewayContractsPin),
    kms_connector_db_migration: pick(needs['build-kms-connector'].outputs.db_migration_build_result, kmsConnectorPin),
    kms_connector_gw_listener: pick(needs['build-kms-connector'].outputs.gw_listener_build_result, kmsConnectorPin),
    kms_connector_kms_worker: pick(needs['build-kms-connector'].outputs.kms_worker_build_result, kmsConnectorPin),
    kms_connector_tx_sender: pick(needs['build-kms-connector'].outputs.tx_sender_build_result, kmsConnectorPin),
    coprocessor_db_migration: pick(needs['build-coprocessor'].outputs.db_migration_build_result, coprocessorPin),
    coprocessor_gw_listener: pick(needs['build-coprocessor'].outputs.gw_listener_build_result, coprocessorPin),
    coprocessor_host_listener: pick(needs['build-coprocessor'].outputs.host_listener_build_result, coprocessorPin),
    coprocessor_sns_worker: pick(needs['build-coprocessor'].outputs.sns_worker_build_result, coprocessorPin),
    coprocessor_tfhe_worker: pick(needs['build-coprocessor'].outputs.tfhe_worker_build_result, coprocessorPin),
    coprocessor_tx_sender: pick(needs['build-coprocessor'].outputs.tx_sender_build_result, coprocessorPin),
    coprocessor_zkproof_worker: pick(needs['build-coprocessor'].outputs.zkproof_worker_build_result, coprocessorPin),
    relayer_migrate: pick(needs['build-relayer'].outputs.relayer_migrate_build_result, relayerPin),
    relayer: pick(needs['build-relayer'].outputs.relayer_build_result, relayerPin),
    test_suite: pick(needs['build-test-suite'].outputs.build_result, testSuitePin),
  };

  // Actor segment is the PR AUTHOR (pull_request.user.login), not github.actor,
  // so it matches pr-preview-destroy.yml (which runs on `closed`). Keep in sync.
  const namespace = isDispatch
    ? `fhevm-ci-${env.ACTOR}-${inputs.namespace_suffix || context.runId}`
    : `fhevm-ci-${context.payload.pull_request.user.login}-${context.payload.pull_request.number}`;

  // Chart/ref versions: env on pull_request, matching input on dispatch.
  const chartVersions = {
    contracts_chart_version: isDispatch ? inputs.contracts_chart_version : env.CONTRACTS_CHART_VERSION,
    coprocessor_chart_version: isDispatch ? inputs.coprocessor_chart_version : env.COPROCESSOR_CHART_VERSION,
    coprocessor_infra_chart_version: isDispatch ? inputs.coprocessor_infra_chart_version : env.COPROCESSOR_INFRA_CHART_VERSION,
    kms_connector_chart_version: isDispatch ? inputs.kms_connector_chart_version : env.KMS_CONNECTOR_CHART_VERSION,
    // Backs postgres-*/relayer-migrate/relayer/test-suite.
    common_chart_version: isDispatch ? inputs.common_chart_version : env.COMMON_CHART_VERSION,
    relayer_sdk_version: isDispatch ? inputs.relayer_sdk_version : env.RELAYER_SDK_VERSION,
    redis_chart_version: isDispatch ? inputs.redis_chart_version : env.REDIS_CHART_VERSION,
    listener_chart_version: isDispatch ? inputs.listener_chart_version : env.LISTENER_CHART_VERSION,
    listener_version: isDispatch ? inputs.listener_version : env.LISTENER_VERSION,
    kms_repo_ref: isDispatch ? inputs.kms_repo_ref : env.KMS_REPO_REF,
    kms_core_tag: isDispatch ? inputs.kms_core_version : env.KMS_CORE_TAG,
    nb_kms_core: isDispatch ? inputs.nb_kms_core : env.NB_KMS_CORE,
    nb_coprocessor: isDispatch ? inputs.nb_coprocessor : env.NB_COPROCESSOR,
  };

  core.info(`Resolved tags: ${JSON.stringify(tags, null, 2)}`);
  core.info(`Resolved namespace: ${namespace}`);
  core.info(`Resolved chart versions: ${JSON.stringify(chartVersions, null, 2)}`);
  core.setOutput('short_sha', shortSha);
  core.setOutput('tags_json', JSON.stringify(tags));
  core.setOutput('namespace', namespace);
  for (const [key, value] of Object.entries(chartVersions)) {
    core.setOutput(key, value);
  }

  // Deployment plan summary.
  await core.summary
    .addHeading(
      isDispatch
        ? `fhevm e2e preview - deployment plan (manual dispatch, ${namespace}, ${shortSha})`
        : `fhevm e2e preview - deployment plan (PR #${context.payload.pull_request.number}, ${shortSha})`,
    )
    .addHeading('Helm charts', 3)
    .addTable([
      [{ data: 'Component', header: true }, { data: 'Chart', header: true }, { data: 'Version', header: true }],
      ['anvil-node (host + gateway chains)', env.ANVIL_NODE_CHART, env.ANVIL_NODE_CHART_VERSION],
      ['contracts (host/gateway/keygen)', env.CONTRACTS_CHART, chartVersions.contracts_chart_version],
      ['coprocessor', env.COPROCESSOR_CHART, chartVersions.coprocessor_chart_version],
      ['kms-connector', env.KMS_CONNECTOR_CHART, chartVersions.kms_connector_chart_version],
      ['coprocessor-infra (Crossplane S3)', env.COPROCESSOR_INFRA_CHART, chartVersions.coprocessor_infra_chart_version],
      ['coprocessor-redis (per-party broker)', env.REDIS_CHART, chartVersions.redis_chart_version],
      ['listener (per-party host-chain producer)', env.LISTENER_CHART, `${chartVersions.listener_chart_version} (image ${chartVersions.listener_version})`],
      ['postgres-* / relayer-migrate / relayer / test-suite (common chart)', env.COMMON_CHART, chartVersions.common_chart_version],
    ])
    .addHeading('Dedicated KMS (zama-ai/kms)', 3)
    .addTable([
      [{ data: 'Component', header: true }, { data: 'Ref / tag', header: true }],
      ['kms repo ref (deploy scripts + tkms-infra/kms-core charts)', chartVersions.kms_repo_ref],
      ['kms-core image tag', chartVersions.kms_core_tag],
      ['number of parties (kms-core / kms-connector / Postgres, 1:1)', chartVersions.nb_kms_core],
      ['number of coprocessor parties (coprocessor / coprocessor-infra / Postgres, 1:1)', chartVersions.nb_coprocessor],
    ])
    .addHeading('Images', 3)
    .addRaw(
      `Short SHA \`${shortSha}\` for components built this run (build_images=\`${needs['check-labels'].outputs.build_images}\`) ` +
        `and actually changed, otherwise the pinned version shown per component below.\n\n`,
    )
    .addTable([
      [{ data: 'Component', header: true }, { data: 'Tag', header: true }],
      ...Object.entries(tags).map(([component, tag]) => [component, tag]),
    ])
    .write();
};
