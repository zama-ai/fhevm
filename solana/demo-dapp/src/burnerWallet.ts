// The demo wallet: an Ed25519 keypair generated in this browser and kept in its local storage, so a
// reload finds the same funded wallet. Nothing about it leaves the page except signatures; it is a
// burner for toy mints, and the storage key names it as such.

export const BURNER_WALLET_STORAGE_KEY = 'fhevm-demo-burner-wallet';

type KeyStorage = Pick<Storage, 'getItem' | 'setItem'>;

const parseStored = (value: string | null): Uint8Array | undefined => {
  if (value === null) return undefined;
  try {
    const bytes = JSON.parse(value) as unknown;
    if (
      Array.isArray(bytes) &&
      bytes.length === 64 &&
      bytes.every((byte) => Number.isInteger(byte) && byte >= 0 && byte <= 255)
    ) {
      return Uint8Array.from(bytes as number[]);
    }
  } catch {
    // A corrupt entry is replaced below.
  }
  return undefined;
};

/** A fresh 64-byte secret key in the layout `@solana/kit` reads: the seed, then the public key. */
const generateBurnerSecretKey = async (): Promise<Uint8Array> => {
  const subtle = crypto.subtle;
  const keyPair = (await subtle.generateKey('Ed25519', true, ['sign', 'verify'])) as CryptoKeyPair;
  const [pkcs8, publicKey] = await Promise.all([
    subtle.exportKey('pkcs8', keyPair.privateKey),
    subtle.exportKey('raw', keyPair.publicKey),
  ]);
  const secretKey = new Uint8Array(64);
  // PKCS#8 wraps the 32-byte seed as the trailing OCTET STRING.
  secretKey.set(new Uint8Array(pkcs8).slice(-32), 0);
  secretKey.set(new Uint8Array(publicKey), 32);
  return secretKey;
};

/** The stored demo wallet, or a new one persisted for the next reload. */
export const loadOrCreateBurnerSecretKey = async (storage: KeyStorage): Promise<Uint8Array> => {
  const stored = parseStored(storage.getItem(BURNER_WALLET_STORAGE_KEY));
  if (stored !== undefined) return stored;
  const secretKey = await generateBurnerSecretKey();
  storage.setItem(BURNER_WALLET_STORAGE_KEY, JSON.stringify(Array.from(secretKey)));
  return secretKey;
};
