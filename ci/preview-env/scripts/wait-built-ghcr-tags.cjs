// Wait until every image built this run is pullable from GHCR.
// 404 is "not yet" (tag propagation). Transport/5xx stay inside registryClient.
//
//   await require('./ci/preview-env/scripts/wait-built-ghcr-tags.cjs')({
//     core, user, token, images: [{ repo, tag }],
//   })

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

module.exports = waitBuiltGhcrTags;
