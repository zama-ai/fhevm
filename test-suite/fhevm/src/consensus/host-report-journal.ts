export type HostReportOriginal = { chain: string; block: string; original: string; blockHash: string; bucket: string };

export function hostReportOriginals(value: unknown): HostReportOriginal[] {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("invalid host report journal");
  return Object.entries(value).map(([key, item]) => {
    const row = item as HostReportOriginal;
    if (!row || row.block !== key || !/^[0-9]+$/.test(row.chain) || !/^[0-9]+$/.test(row.block) ||
        !/^[0-9a-f]{64}$/.test(row.original) || !/^0x[0-9a-f]{64}$/.test(row.blockHash) || row.bucket !== "coproc-1") {
      throw new Error("unscoped or damaged host report recovery entry");
    }
    return row;
  });
}
