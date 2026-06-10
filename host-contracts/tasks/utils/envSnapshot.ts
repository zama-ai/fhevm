type EnvForKeys<K extends string> = Record<K, string>;
type EnvSnapshotForKeys<K extends string> = Partial<EnvForKeys<K>>;

export function makeEnvHelpers<K extends string>(keys: readonly K[]) {
  return {
    apply: (env: EnvForKeys<K>): void => {
      for (const key of keys) {
        process.env[key] = env[key];
      }
    },
    snapshot: (): EnvSnapshotForKeys<K> => {
      const snapshot: EnvSnapshotForKeys<K> = {};
      for (const key of keys) {
        const value = process.env[key];
        if (value !== undefined) {
          snapshot[key] = value;
        }
      }
      return snapshot;
    },
    restore: (snapshot: EnvSnapshotForKeys<K>): void => {
      for (const key of keys) {
        const value = snapshot[key];
        if (value === undefined) {
          delete process.env[key];
        } else {
          process.env[key] = value;
        }
      }
    },
  };
}
