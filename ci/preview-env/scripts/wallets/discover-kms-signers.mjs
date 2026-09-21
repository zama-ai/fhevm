// Resolve each KMS party's on-chain signer (VerfAddress) from the shared S3
// public vault (PUB-p<i>/ prefix) and print [{party,signer}] to stdout.
// Resolve the newest VerfAddress OBJECT dynamically rather than a hardcoded
// handle: the handle drifts across kms-core versions and a stale one yields a
// signer that no longer matches the running node, which reverts every
// prepKeygenResponse (KmsSignerDoesNotMatchTxSender). FALLBACK_HANDLE is used
// only when anonymous bucket listing is disabled.
// Env: NB_KMS_CORE, S3_ENDPOINT, FALLBACK_HANDLE.
const n = parseInt(process.env.NB_KMS_CORE, 10);
const base = process.env.S3_ENDPOINT.replace(/\/+$/, '');
const fallback = process.env.FALLBACK_HANDLE;

// Newest object under PUB-p<i>/VerfAddress/, or the fallback handle.
async function resolveHandle(i) {
  const url = `${base}?list-type=2&prefix=PUB-p${i}/VerfAddress/`;
  try {
    const res = await fetch(url);
    if (!res.ok) return fallback;
    const xml = await res.text();
    const items = [...xml.matchAll(/<Contents>([\s\S]*?)<\/Contents>/g)]
      .map((m) => {
        const key = (m[1].match(/<Key>([^<]+)<\/Key>/) || [])[1];
        const lm = (m[1].match(/<LastModified>([^<]+)<\/LastModified>/) || [])[1];
        return { key, lm: lm ? Date.parse(lm) : 0 };
      })
      .filter((o) => o.key && /\/VerfAddress\/[^/]+$/.test(o.key));
    if (!items.length) return fallback;
    items.sort((a, b) => b.lm - a.lm);
    return items[0].key.split('/').pop();
  } catch (e) {
    return fallback;
  }
}

(async () => {
  const out = [];
  for (let i = 1; i <= n; i++) {
    const handle = await resolveHandle(i);
    const url = `${base}/PUB-p${i}/VerfAddress/${handle}`;
    const res = await fetch(url);
    if (!res.ok) {
      console.error(`::error::failed to fetch signer for party ${i} at ${url} (HTTP ${res.status})`);
      process.exit(1);
    }
    const signer = (await res.text()).trim();
    if (!/^0x[0-9a-fA-F]{40}$/.test(signer)) {
      console.error(`::error::party ${i} VerfAddress at ${url} is not a valid address: "${signer}"`);
      process.exit(1);
    }
    console.error(`Party ${i}: handle=${handle} signer=${signer}`);
    out.push({ party: i, signer });
  }
  // N parties must map to N distinct signers; a duplicate means a stale/shared
  // identity that would scramble the signer<->tx-sender pairing on-chain.
  const uniq = new Set(out.map((o) => o.signer.toLowerCase()));
  if (uniq.size !== out.length) {
    console.error(`::error::discovered signers are not distinct (stale/shared identity in the public vault): ${JSON.stringify(out)}`);
    process.exit(1);
  }
  process.stdout.write(JSON.stringify(out));
})();
