export type DemoVaultMetrics = {
  readonly totalAssets: bigint;
  readonly totalShares: bigint;
};

const parseMetrics = (value: unknown): DemoVaultMetrics => {
  if (typeof value !== 'object' || value === null) throw new Error('vault metrics must be an object');
  const raw = value as Record<string, unknown>;
  if (typeof raw.totalAssets !== 'string' || typeof raw.totalShares !== 'string') {
    throw new Error('vault metrics totals must be strings');
  }
  return { totalAssets: BigInt(raw.totalAssets), totalShares: BigInt(raw.totalShares) };
};

const responseJson = async (response: Response): Promise<unknown> => {
  const body = await response.json().catch(() => null);
  if (!response.ok) {
    throw new Error(
      typeof body === 'object' && body !== null && typeof (body as { error?: unknown }).error === 'string'
        ? (body as { error: string }).error
        : `vault yield request failed with HTTP ${response.status}`,
    );
  }
  return body;
};

export const readDemoVaultMetrics = async (): Promise<DemoVaultMetrics> =>
  parseMetrics(await responseJson(await fetch('/api/demo-vault-metrics')));

export const harvestDemoVault = async (): Promise<{
  readonly before: DemoVaultMetrics;
  readonly after: DemoVaultMetrics;
}> => {
  const body = (await responseJson(
    await fetch('/api/demo-harvest', {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: '{}',
    }),
  )) as { readonly before?: unknown; readonly after?: unknown };
  return { before: parseMetrics(body.before), after: parseMetrics(body.after) };
};
