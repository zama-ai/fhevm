/** Replace just the test-owned replay window; keep every deployment argument. */
export function artifactReplayRoute(command: unknown, mode: string): string[] {
  if (!Array.isArray(command) || command.some((arg) => typeof arg !== 'string'))
    throw new Error('invalid poller command');
  if (!['overlap', 'cursor'].includes(mode)) throw new Error('unknown recovery mode');
  const kept: string[] = [];
  let seen = false;
  for (let i = 0; i < command.length; i++) {
    const arg = command[i] as string;
    if (arg === '--replay-window' || arg.startsWith('--replay-window=')) {
      if (seen) throw new Error('ambiguous replay window');
      seen = true;
      const value = arg === '--replay-window' ? command[++i] : arg.slice('--replay-window='.length);
      if (!/^\d+$/.test(value ?? '')) throw new Error('invalid existing replay window');
    } else kept.push(arg);
  }
  return [...kept, `--replay-window=${mode === 'overlap' ? 512 : 1}`];
}
