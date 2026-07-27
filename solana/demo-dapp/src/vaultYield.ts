import type { VaultMetrics } from './batchTypes';
import { demoApiFetch } from './demoAuthorization';
import { parseVaultMetrics } from './demoApi';

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

export const readDemoVaultMetrics = async (): Promise<VaultMetrics> =>
  parseVaultMetrics(await responseJson(await fetch('/api/demo-vault-metrics')));

export const harvestDemoVault = async (): Promise<{
  readonly before: VaultMetrics;
  readonly after: VaultMetrics;
}> => {
  const body = (await responseJson(
    await demoApiFetch('/api/demo-harvest', {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: '{}',
    }),
  )) as { readonly before?: unknown; readonly after?: unknown };
  return { before: parseVaultMetrics(body.before), after: parseVaultMetrics(body.after) };
};
