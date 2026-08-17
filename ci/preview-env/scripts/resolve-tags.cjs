// Resolve the tag of every image the deploy job installs, derive the preview
// namespace, and write the deployment-plan job summary. Invoked from the
// resolve-tags github-script step:
//   await require('./ci/preview-env/scripts/resolve-tags.cjs')({ core, context, github })
// Env: NEEDS, EVENT_NAME, INPUTS, ACTOR, MAX_IMAGE_COMMIT_COUNT, GHCR_USER,
// GHCR_READ_TOKEN.
//
// Per image: built this run -> this run's short SHA; else a non-empty
// <component>_version dispatch input; else the short SHA of the newest ancestor
// of the change-detection base commit with an image published in GHCR. Every
// main/release/* push tags every image with its short SHA (built or re-tagged,
// see re-tag-docker-image.yml), so that is normally the base commit itself;
// existence is still verified so a pruned tag fails here instead of as
// ImagePullBackOff ~40min into the deploy. There are deliberately no fallback
// pins - anything unresolvable is a hard failure, not a silent downgrade.
// Charts and external pins are wired in preview-env-deploy.yml, not here.

// CI pushes to ghcr.io/zama-ai/*; the cluster pulls the same artifacts through
// the hub.zama.org/ghcr proxy-cache, so GHCR is where existence is checked.
const GHCR_HOST = 'ghcr.io';
const GHCR_OWNER = 'zama-ai';

// `key` is what the deploy job's `helm --set-string` calls read out of
// tags_json, `repo` must match the `image-name:` in the matching
// *-docker-build.yml, and `<component>_version` is the dispatch override input.
const IMAGES = [
  { key: 'host_contracts', repo: 'fhevm/host-contracts', job: 'build-host-contracts', output: 'build_result', component: 'host_contracts', label: 'host-contracts' },
  { key: 'gateway_contracts', repo: 'fhevm/gateway-contracts', job: 'build-gateway-contracts', output: 'build_result', component: 'gateway_contracts', label: 'gateway-contracts' },
  { key: 'kms_connector_db_migration', repo: 'fhevm/kms-connector/db-migration', job: 'build-kms-connector', output: 'db_migration_build_result', component: 'kms_connector', label: 'kms-connector/db-migration' },
  { key: 'kms_connector_gw_listener', repo: 'fhevm/kms-connector/gw-listener', job: 'build-kms-connector', output: 'gw_listener_build_result', component: 'kms_connector', label: 'kms-connector/gw-listener' },
  { key: 'kms_connector_kms_worker', repo: 'fhevm/kms-connector/kms-worker', job: 'build-kms-connector', output: 'kms_worker_build_result', component: 'kms_connector', label: 'kms-connector/kms-worker' },
  { key: 'kms_connector_tx_sender', repo: 'fhevm/kms-connector/tx-sender', job: 'build-kms-connector', output: 'tx_sender_build_result', component: 'kms_connector', label: 'kms-connector/tx-sender' },
  { key: 'coprocessor_db_migration', repo: 'fhevm/coprocessor/db-migration', job: 'build-coprocessor', output: 'db_migration_build_result', component: 'coprocessor', label: 'coprocessor/db-migration' },
  { key: 'coprocessor_gw_listener', repo: 'fhevm/coprocessor/gw-listener', job: 'build-coprocessor', output: 'gw_listener_build_result', component: 'coprocessor', label: 'coprocessor/gw-listener' },
  { key: 'coprocessor_host_listener', repo: 'fhevm/coprocessor/host-listener', job: 'build-coprocessor', output: 'host_listener_build_result', component: 'coprocessor', label: 'coprocessor/host-listener' },
  { key: 'coprocessor_sns_worker', repo: 'fhevm/coprocessor/sns-worker', job: 'build-coprocessor', output: 'sns_worker_build_result', component: 'coprocessor', label: 'coprocessor/sns-worker' },
  { key: 'coprocessor_tfhe_worker', repo: 'fhevm/coprocessor/tfhe-worker', job: 'build-coprocessor', output: 'tfhe_worker_build_result', component: 'coprocessor', label: 'coprocessor/tfhe-worker' },
  { key: 'coprocessor_tx_sender', repo: 'fhevm/coprocessor/tx-sender', job: 'build-coprocessor', output: 'tx_sender_build_result', component: 'coprocessor', label: 'coprocessor/tx-sender' },
  { key: 'coprocessor_zkproof_worker', repo: 'fhevm/coprocessor/zkproof-worker', job: 'build-coprocessor', output: 'zkproof_worker_build_result', component: 'coprocessor', label: 'coprocessor/zkproof-worker' },
  { key: 'listener', repo: 'fhevm/listener/listener-core', job: 'build-listener', output: 'build_result', component: 'listener', label: 'listener/listener-core' },
  { key: 'relayer_migrate', repo: 'fhevm/relayer-migrate', job: 'build-relayer', output: 'relayer_migrate_build_result', component: 'relayer', label: 'relayer/migrate' },
  { key: 'relayer', repo: 'fhevm/relayer', job: 'build-relayer', output: 'relayer_build_result', component: 'relayer', label: 'relayer' },
  { key: 'test_suite', repo: 'fhevm/test-suite/e2e', job: 'build-test-suite', output: 'build_result', component: 'test_suite', label: 'test-suite' },
];

// Accept indexes as well as manifests, or the registry can 404 a multi-arch
// tag that exists.
const MANIFEST_ACCEPT = [
  'application/vnd.oci.image.index.v1+json',
  'application/vnd.docker.distribution.manifest.list.v2+json',
  'application/vnd.oci.image.manifest.v1+json',
  'application/vnd.docker.distribution.manifest.v2+json',
].join(', ');

const RETRYABLE_STATUS = new Set([429, 500, 502, 503, 504]);

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

// GHCR read client (tokens are per-repository, so cache one each). A transient
// error must never read as "tag missing" - that would silently downgrade the
// deploy to older artifacts - so only 404 is an answer; anything else retries
// and eventually throws.
const registryClient = ({ core, user, token }) => {
  const bearers = new Map();

  const request = async (label, doFetch) => {
    let last = '';
    for (let attempt = 1; attempt <= 4; attempt += 1) {
      let response;
      try {
        response = await doFetch();
      } catch (error) {
        last = error.message;
        if (attempt === 4) break;
        await sleep(500 * 2 ** (attempt - 1));
        continue;
      }
      if (response.ok || response.status === 404) return response;
      last = `HTTP ${response.status}`;
      if (!RETRYABLE_STATUS.has(response.status)) break;
      core.info(`${label}: ${last}, retrying (attempt ${attempt}/4)`);
      await sleep(500 * 2 ** (attempt - 1));
    }
    throw new Error(`${label} failed: ${last}`);
  };

  const bearerFor = async (repo) => {
    if (bearers.has(repo)) return bearers.get(repo);
    const scope = encodeURIComponent(`repository:${GHCR_OWNER}/${repo}:pull`);
    const response = await request(`ghcr token for ${repo}`, () =>
      fetch(`https://${GHCR_HOST}/token?service=${GHCR_HOST}&scope=${scope}`, {
        headers: { authorization: `Basic ${Buffer.from(`${user}:${token}`).toString('base64')}` },
      }),
    );
    if (!response.ok) throw new Error(`ghcr token for ${repo} failed: HTTP ${response.status}`);
    const body = await response.json();
    if (!body.token) throw new Error(`ghcr token for ${repo} returned no token`);
    bearers.set(repo, body.token);
    return body.token;
  };

  // GET, not HEAD: HEAD on /manifests/ is optional in the registry spec, and a
  // 405 would be indistinguishable from a real failure. Manifests are a few KB.
  const manifestExists = async (repo, tag) => {
    const bearer = await bearerFor(repo);
    const response = await request(`manifest ${repo}:${tag}`, () =>
      fetch(`https://${GHCR_HOST}/v2/${GHCR_OWNER}/${repo}/manifests/${tag}`, {
        headers: { authorization: `Bearer ${bearer}`, accept: MANIFEST_ACCEPT },
      }),
    );
    return response.status !== 404;
  };

  return { manifestExists };
};

/** Newest-first SHAs reachable from `sha`, capped at `max`. */
const listAncestors = async ({ github, owner, repo, sha, max }) => {
  const perPage = Math.min(100, max);
  const shas = [];
  for (let page = 1; shas.length < max; page += 1) {
    const { data } = await github.rest.repos.listCommits({ owner, repo, sha, per_page: perPage, page });
    shas.push(...data.map((commit) => commit.sha));
    if (data.length < perPage) break;
  }
  return shas.slice(0, max);
};

module.exports = async ({ core, context, github }) => {
  const needs = JSON.parse(process.env.NEEDS);
  const isDispatch = process.env.EVENT_NAME === 'workflow_dispatch';
  const inputs = JSON.parse(process.env.INPUTS);
  const { owner, repo } = context.repo;
  const short = (sha) => sha.substring(0, 7);

  // PR head SHA on pull_request; picked branch tip on dispatch.
  const shortSha = isDispatch ? short(context.sha) : short(context.payload.pull_request.head.sha);

  // Empty dispatch input = "resolve it"; pull_request runs never override.
  const override = (name) => (isDispatch ? String(inputs[name] ?? '').trim() : '');

  const registry = registryClient({ core, user: process.env.GHCR_USER, token: process.env.GHCR_READ_TOKEN });

  // 'success' -> freshly built+pushed; ''/undefined/'skipped' -> not built this
  // run. Anything else is FATAL: a failed build must not fall back to an older
  // image and deploy stale code under a green check.
  const wasBuilt = (image) => {
    const result = needs[image.job]?.outputs?.[image.output];
    if (result === 'success') return true;
    if (result === undefined || result === '' || result === 'skipped') return false;
    throw new Error(`build for '${image.label}' did not succeed (result='${result}'); refusing to fall back to an older image`);
  };

  // The change-detection base: what "unchanged" is measured against, and so the
  // commit whose images are the right ones to deploy. Only main/release/*
  // commits are published, so anything else resolves to its merge-base with main.
  const resolveBase = async () => {
    const mergeBaseWith = async (head) => {
      const { data } = await github.rest.repos.compareCommits({ owner, repo, base: 'main', head });
      return data.merge_base_commit.sha;
    };
    if (isDispatch) {
      return { sha: await mergeBaseWith(context.sha), why: `merge-base of ${shortSha} with main` };
    }
    const pr = context.payload.pull_request;
    if (pr.base.ref === 'main' || pr.base.ref.startsWith('release/')) {
      return { sha: pr.base.sha, why: `PR base (${pr.base.ref})` };
    }
    const sha = await mergeBaseWith(pr.base.sha);
    core.warning(
      `This PR targets '${pr.base.ref}', which publishes no images. Resolving from its merge-base ` +
        `with main (${short(sha)}) instead, so images built by the parent PR are NOT included. ` +
        `Retarget at main once the parent merges.`,
    );
    return { sha, why: `merge-base of ${pr.base.ref} with main` };
  };

  const { sha: baseSha, why: baseWhy } = await resolveBase();

  // source: 'built' | 'dispatch-override' | 'base-sha' | 'unresolved'
  const decisions = new Map();
  for (const image of IMAGES) {
    const value = override(`${image.component}_version`);
    if (wasBuilt(image)) {
      decisions.set(image.key, { tag: shortSha, source: 'built', detail: `built this run (${shortSha})` });
    } else if (value) {
      decisions.set(image.key, { tag: value, source: 'dispatch-override', detail: 'explicit dispatch input' });
    }
  }

  // Fail fast in case of bad override tags
  const overridden = IMAGES.filter((image) => decisions.get(image.key)?.source === 'dispatch-override');
  const overrideExists = await Promise.all(overridden.map((image) => registry.manifestExists(image.repo, decisions.get(image.key).tag)));
  overridden.forEach((image, i) => {
    if (overrideExists[i]) return;
    const { tag } = decisions.get(image.key);
    decisions.set(image.key, { tag: '', source: 'unresolved', detail: `dispatch override '${tag}' not found in GHCR (${image.repo})` });
  });

  let pending = IMAGES.filter((image) => !decisions.has(image.key));
  let searched = 0;
  if (pending.length > 0) {
    const maxCommits = Number(process.env.MAX_IMAGE_COMMIT_COUNT || 50);
    const ancestors = await listAncestors({ github, owner, repo, sha: baseSha, max: maxCommits });
    searched = ancestors.length;
    core.info(`Resolving ${pending.length} image(s) from ${baseWhy} ${short(baseSha)} (${searched} candidate commits)`);

    // Commits outer / images inner: the common case (everything published at
    // the base commit) costs one round of parallel lookups.
    for (const [distance, commit] of ancestors.entries()) {
      if (pending.length === 0) break;
      const tag = short(commit);
      const found = await Promise.all(pending.map((image) => registry.manifestExists(image.repo, tag)));
      const behind = [];
      pending = pending.filter((image, i) => {
        if (!found[i]) return true;
        decisions.set(image.key, {
          tag,
          source: 'base-sha',
          detail: distance === 0 ? `${baseWhy} ${tag}` : `${tag}, ${distance} commit(s) behind base`,
        });
        if (distance > 0) behind.push(image.label);
        return false;
      });
      // One warning per commit, not per image: a pruned base commit affects
      // every image at once.
      if (behind.length > 0) {
        core.warning(
          `No image at base commit ${short(baseSha)} for: ${behind.join(', ')}. Using ${tag} ` +
            `(${distance} commit(s) behind base) - likely registry retention; the deployed code ` +
            `may be older than the base commit.`,
        );
      }
    }
  }
  for (const image of pending) {
    decisions.set(image.key, { tag: '', source: 'unresolved', detail: `no published image in the last ${searched} commits from ${short(baseSha)}` });
  }

  const tags = Object.fromEntries(IMAGES.map((image) => [image.key, decisions.get(image.key).tag]));

  // Actor segment is the PR AUTHOR (not github.actor) so it matches what
  // preview-env-destroy.yml derives on `closed` - keep the two in sync. k8s
  // namespaces must be RFC-1123 labels, but GitHub logins are case-preserving
  // and bot logins contain brackets (`dependabot[bot]`), hence the sanitizing.
  const sanitizeNs = (s) =>
    String(s)
      .toLowerCase()
      .replace(/[^a-z0-9-]/g, '-')
      .replace(/-+/g, '-')
      .replace(/^-+|-+$/g, '');
  const namespace = isDispatch
    ? sanitizeNs(`fhevm-ci-${process.env.ACTOR}-${inputs.namespace_suffix || context.runId}`)
    : sanitizeNs(`fhevm-ci-${context.payload.pull_request.user.login}-${context.payload.pull_request.number}`);

  // AWS limits namespace length to 63 chars. If longer, the enclave nodegroup
  // silently times out after 20 minutes, so fail fast here with margin.
  // Format: kms-party-<namespace> + two 6-char Crossplane suffixes
  const NAMESPACE_MAX = 40;
  if (namespace.length > NAMESPACE_MAX) {
    throw new Error(
      `namespace '${namespace}' is ${namespace.length} chars, max is ${NAMESPACE_MAX}: the derived EKS ` +
        `nodegroup name (kms-party-<namespace> plus two Crossplane suffixes) would exceed AWS's 63-char ` +
        `limit and the KMS deploy would time out with no readable error. ` +
        (isDispatch
          ? `Pass a shorter namespace_suffix.`
          : `The PR author's login is too long; deploy via workflow_dispatch with a short namespace_suffix instead.`),
    );
  }

  // Summary first, so a failed resolution still shows exactly what it resolved
  // and what it couldn't.
  await core.summary
    .addHeading(
      isDispatch
        ? `fhevm e2e preview - deployment plan (manual dispatch, ${namespace}, ${shortSha})`
        : `fhevm e2e preview - deployment plan (PR #${context.payload.pull_request.number}, ${shortSha})`,
    )
    .addRaw(
      `Base commit: ${baseWhy} \`${short(baseSha)}\`. Images built this run are tagged \`${shortSha}\`; ` +
        `the rest resolve from the base commit (table below). Helm charts install from this run's ` +
        `checkout unless overridden (see preview-env-deploy.yml).\n\n`,
    )
    .addHeading('Images', 3)
    .addTable([
      [{ data: 'Component', header: true }, { data: 'Tag', header: true }, { data: 'Source', header: true }, { data: 'Resolved from', header: true }],
      ...IMAGES.map((image) => {
        const decision = decisions.get(image.key);
        return [image.key, decision.tag || '-', decision.source, decision.detail];
      }),
    ])
    .write();

  const unresolved = IMAGES.filter((i) => decisions.get(i.key).source === 'unresolved');
  if (unresolved.length > 0) {
    throw new Error(
      `could not resolve ${unresolved.length} image(s) from ${baseWhy} ${short(baseSha)}:\n` +
        unresolved.map((i) => `  - ${i.label} (${decisions.get(i.key).detail})`).join('\n') +
        `\nThere are deliberately no fallback pins. Fix the dispatch override tag if one is listed ` +
        `above; otherwise the registry likely pruned these tags - raise MAX_IMAGE_COMMIT_COUNT, ` +
        `rebase onto a newer base commit, or pass an explicit version via workflow_dispatch.`,
    );
  }

  core.info(`Resolved tags: ${JSON.stringify(tags, null, 2)}`);
  core.info(`Resolved namespace: ${namespace}`);
  core.setOutput('tags_json', JSON.stringify(tags));
  core.setOutput('namespace', namespace);
};
