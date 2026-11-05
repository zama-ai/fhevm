export type HostReportOriginal = { chain: string; block: string; original: string; blockHash: string; bucket: string; key: string };

export function hostReportOriginals(value: unknown): HostReportOriginal[] {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("invalid host report journal");
  return Object.entries(value).map(([key, item]) => {
    const row = item as HostReportOriginal;
    const digits = !!row && /^[0-9]+$/.test(row.chain) && /^[0-9]+$/.test(row.block);
    // `key` is the attempt-scoped object key the divergent report was published
    // at (state_hash.rs:state_hash_key); restoration must target exactly that
    // object, and only within this row's own (chain, block) scope.
    const scopedKey = digits && new RegExp(
      `^state_hash/proposal=[0-9a-f]+/attempt=[0-9]+/chain=${row.chain}/block=${row.block}\\.bin$`,
    ).test(row.key);
    if (!row || row.block !== key || !digits ||
        !/^[0-9a-f]{64}$/.test(row.original) || !/^0x[0-9a-f]{64}$/.test(row.blockHash) || row.bucket !== "coproc-1" ||
        !scopedKey) {
      throw new Error("unscoped or damaged host report recovery entry");
    }
    return row;
  });
}
