export const formatUsdc = (baseUnits: bigint): string =>
  new Intl.NumberFormat('en-US', { maximumFractionDigits: 6 }).format(Number(baseUnits) / 1_000_000);
