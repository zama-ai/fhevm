// Wait until every image built this run is pullable from GHCR.
// 404 is "not yet" (tag propagation). Transport/5xx stay inside registryClient.
//
//   await require('./ci/preview-env/scripts/wait-built-ghcr-tags.cjs')({
//     core, user, token, images: [{ repo, tag }],
//   })
//
// Baseline / lock-selected tags are different: a 404 means publishing never
// happened (failed retag, failed CGR build), not delay. assertLockGhcrTags
// fails immediately with the missing image:tag.

const { registryClient, sleep } = require('./ghcr-registry.cjs');

const DEFAULT_TIMEOUT_MS = 120_000;
const DEFAULT_INTERVAL_MS = 5_000;

const waitBuiltGhcrTags = async ({
  core,
  user,
  token,
  images,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  intervalMs = DEFAULT_INTERVAL_MS,
}) => {
  if (!images.length) {
    core.info('no images built this run; skipping GHCR wait');
    return;
  }
  if (!token) {
    throw new Error('GHCR_READ_TOKEN is empty; cannot wait for published tags');
  }

  const registry = registryClient({ core, user, token });
  const deadline = Date.now() + timeoutMs;
  let missing = images.map((image) => `${image.repo}:${image.tag}`);

  while (Date.now() < deadline) {
    const found = await Promise.all(images.map((image) => registry.manifestExists(image.repo, image.tag)));
    missing = images.filter((_, i) => !found[i]).map((image) => `${image.repo}:${image.tag}`);
    if (missing.length === 0) {
      core.info(`all ${images.length} built tag(s) present in GHCR`);
      return;
    }
    core.info(`GHCR not ready for ${missing.join(', ')}; retrying`);
    await sleep(intervalMs);
  }

  throw new Error(
    `built image tag(s) still missing from GHCR after ${timeoutMs}ms:\n` +
      missing.map((ref) => `  - ${ref}`).join('\n') +
      `\nThe docker-build job succeeded but the registry has not published the tag yet.`,
  );
};

/** Lock env keys for components this run did not rebuild. Empty / omitted keys
 * (optional unpublished images) are skipped. */
const selectBaselineGhcrImages = ({ lockEnv = {}, skipped = [] }) =>
  skipped
    .filter(({ envKey }) => typeof lockEnv[envKey] === 'string' && lockEnv[envKey].length)
    .map(({ envKey, repo }) => ({ repo, tag: lockEnv[envKey], envKey }));

const assertLockGhcrTags = async ({ core, user, token, images }) => {
  if (!images.length) {
    core.info('no baseline lock images to verify');
    return;
  }
  if (!token) {
    throw new Error('GHCR_READ_TOKEN is empty; cannot verify lock-selected tags');
  }

  const registry = registryClient({ core, user, token });
  const found = await Promise.all(images.map((image) => registry.manifestExists(image.repo, image.tag)));
  const missing = images.filter((_, i) => !found[i]).map((image) => `${image.repo}:${image.tag}`);
  if (missing.length === 0) {
    core.info(`all ${images.length} lock-selected baseline tag(s) present in GHCR`);
    return;
  }
  throw new Error(
    `lock-selected image(s) missing from GHCR:\n` +
      missing.map((ref) => `  - ${ref}`).join('\n') +
      `\nThis is not registry delay: the baseline tag is unavailable. Re-run the ` +
      `component's docker-build / retag job, or resolve to a permitted ancestor tag.`,
  );
};

module.exports = waitBuiltGhcrTags;
module.exports.waitBuiltGhcrTags = waitBuiltGhcrTags;
module.exports.assertLockGhcrTags = assertLockGhcrTags;
module.exports.selectBaselineGhcrImages = selectBaselineGhcrImages;
