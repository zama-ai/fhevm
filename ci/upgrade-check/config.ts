export type PackageName = "host-contracts" | "gateway-contracts";

export const PACKAGE_CONFIG: Record<PackageName, { extraDeps?: string }> = {
  "host-contracts": { extraDeps: "forge soldeer install" },
  "gateway-contracts": {},
};
