// A framed message for the terminal. `stderr` writes bypass hardhat's console capture, which is why
// the diagnostics default there.

import { PLUGIN_ID } from './constants.js';

export type LogOutput = 'stderr' | 'stdout' | 'console';

export function logBox(title: string, body: string, out: LogOutput): void {
  const heading = `${PLUGIN_ID}: ${title}`;
  const lines = body.split('\n');
  const width = Math.max(heading.length, ...lines.map((line) => line.length));
  const frame = (line: string): string => ` ║  ${line.padEnd(width)}  ║`;
  const text = [
    ` ╔${'═'.repeat(width + 4)}╗`,
    frame(heading),
    frame(''),
    ...lines.map(frame),
    ` ╚${'═'.repeat(width + 4)}╝`,
  ].join('\n');
  write(text, out);
}

function write(text: string, out: LogOutput): void {
  if (out === 'stderr') process.stderr.write(`${text}\n`);
  else if (out === 'stdout') process.stdout.write(`${text}\n`);
  else console.log(text);
}
