// Shared GHCR read client. Used by resolve-tags.cjs (preview) and
// wait-built-ghcr-tags.cjs (orchestrated e2e). Tokens are per-repository.
//
// A transient error must never read as "tag missing" — that would silently
// downgrade a deploy — so only 404 is an answer; anything else retries and
// eventually throws.

const GHCR_HOST = 'ghcr.io';
const GHCR_OWNER = 'zama-ai';

const MANIFEST_ACCEPT = [
  'application/vnd.oci.image.index.v1+json',
  'application/vnd.docker.distribution.manifest.list.v2+json',
  'application/vnd.oci.image.manifest.v1+json',
  'application/vnd.docker.distribution.manifest.v2+json',
].join(', ');

const RETRYABLE_STATUS = new Set([429, 500, 502, 503, 504]);

const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

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

module.exports = { GHCR_HOST, GHCR_OWNER, registryClient, sleep };
