// Fake worker recording how it was invoked (argv, environment, stdin job), then exiting 0.
import { mkdirSync, writeFileSync } from 'node:fs';
import path from 'node:path';

const chunks: Buffer[] = [];
for await (const chunk of process.stdin) chunks.push(chunk as Buffer);
const stdin = Buffer.concat(chunks).toString('utf8');
const job = JSON.parse(stdin) as { artifactsDir: string };
mkdirSync(job.artifactsDir, { recursive: true });
writeFileSync(
  path.join(job.artifactsDir, 'invocation.json'),
  JSON.stringify({ argv: process.argv, env: process.env, stdin }),
);
process.exit(0);
