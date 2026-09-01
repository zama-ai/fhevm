const BYTES32_HEX = /^0x[0-9a-fA-F]{64}$/;

export function optionalFheEncryptionKeyTrust(config) {
  if (config?.contracts?.kmsGeneration !== undefined) {
    return undefined;
  }

  const trust = config?.fheEncryptionKeyTrust;
  if (trust === undefined) {
    return undefined;
  }
  if (!BYTES32_HEX.test(trust.publicKeyDigest ?? '') || !BYTES32_HEX.test(trust.crsDigest ?? '')) {
    throw new Error('Gateway config has invalid FHE encryption-key digest pins.');
  }
  return trust;
}

export async function fetchOptionalFheEncryptionKeyTrust(origin, slot) {
  const response = await fetch(`${origin}/gw/${slot}/config`, { cache: 'no-store' });
  if (!response.ok) {
    throw new Error(`GET /gw/${slot}/config -> ${response.status}`);
  }
  return optionalFheEncryptionKeyTrust(await response.json());
}
