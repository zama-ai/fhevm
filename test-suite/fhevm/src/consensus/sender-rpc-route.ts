/** The same bounded retry policy is used with old and fixed sender images. */
export function senderRoute(command: unknown, url?: string): { original: string; command: string[] } {
  if (!Array.isArray(command) || command.some((arg) => typeof arg !== 'string'))
    throw new Error('invalid sender command');
  const overrides: Record<string, string> = {
    '--gateway-url': '',
    '--add-ciphertexts-max-retries': '3',
    '--database-polling-interval-secs': '1',
    '--send-txn-sync-timeout-secs': '4',
    '--add-ciphertexts-batch-limit': '20',
  };
  const kept: string[] = [],
    seen = new Set<string>();
  let original = '';
  for (let index = 0; index < command.length; index++) {
    const arg = command[index] as string,
      flag = arg.split('=')[0]!;
    if (!(flag in overrides)) {
      kept.push(arg);
      continue;
    }
    if (seen.has(flag)) throw new Error(`ambiguous sender argument ${flag}`);
    seen.add(flag);
    const value = arg.includes('=') ? arg.slice(flag.length + 1) : command[++index];
    if (!value || value.startsWith('--')) throw new Error(`missing sender argument ${flag}`);
    if (flag === '--gateway-url') original = value;
  }
  if (!original || new URL(original).protocol !== 'http:') throw new Error('explicit HTTP sender route required');
  if (url && new URL(url).protocol !== 'http:') throw new Error('invalid proxy route');
  overrides['--gateway-url'] = url ?? original;
  return { original, command: [...kept, ...Object.entries(overrides).map(([key, value]) => `${key}=${value}`)] };
}
