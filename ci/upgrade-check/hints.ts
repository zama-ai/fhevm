export interface UpgradeReportHint {
  defaults?: Record<string, string>;
}

export interface UpgradeConstraint {
  contracts: string[];
  message: string;
}

export const CONTRACT_HINTS: Record<string, Record<string, UpgradeReportHint>> = {
  "host-contracts": {
    HCULimit: {},
  },
  "gateway-contracts": {},
};

export const PACKAGE_CONSTRAINTS: Record<string, UpgradeConstraint[]> = {
  "host-contracts": [
    {
      contracts: ["HCULimit", "FHEVMExecutor"],
      message: "HCULimit and FHEVMExecutor both changed. Check whether they must be upgraded atomically or back-to-back.",
    },
  ],
  "gateway-contracts": [],
};
