/**
 * Compiles the engine's own test contracts (`contracts/*.sol`) with solc-js into Hardhat-format
 * artifacts (`dist/contracts/<File>.sol/<Name>.json`), the files `this.engine.contracts` reads. Run
 * by `npm run build`; nothing is compiled at run time. The contracts take the protocol addresses as a
 * constructor argument, so the artifacts serve every profile.
 *
 * Same compiler settings as the e2e suite (solc 0.8.24, optimizer 800 runs, cancun, no metadata hash).
 */
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';

const require = createRequire(import.meta.url);

interface Solc {
  version(): string;
  compile(
    input: string,
    callbacks: { import: (importPath: string) => { contents: string } | { error: string } },
  ): string;
}

interface SolcError {
  severity: 'error' | 'warning' | 'info';
  formattedMessage: string;
}

interface SolcOutput {
  errors?: SolcError[];
  contracts?: Record<
    string,
    Record<string, { abi: unknown[]; evm: { bytecode: { object: string }; deployedBytecode: { object: string } } }>
  >;
}

export const SOLC_SETTINGS = {
  optimizer: { enabled: true, runs: 800 },
  evmVersion: 'cancun',
  metadata: { bytecodeHash: 'none' },
  outputSelection: { '*': { '*': ['abi', 'evm.bytecode.object', 'evm.deployedBytecode.object'] } },
} as const;

export interface BuildResult {
  compiler: string;
  /** Contract names written, sorted. */
  contracts: string[];
  warnings: string[];
}

/**
 * Resolves a library import (`@fhevm/solidity/lib/FHE.sol`, `encrypted-types/...`) in the
 * `node_modules` folders from `root` upwards, the way Hardhat does.
 */
function importResolver(root: string): (importPath: string) => { contents: string } | { error: string } {
  return (importPath) => {
    for (let dir = root; ; dir = path.dirname(dir)) {
      const candidate = path.join(dir, 'node_modules', importPath);
      if (existsSync(candidate)) return { contents: readFileSync(candidate, 'utf8') };
      if (path.dirname(dir) === dir) return { error: `not found in any node_modules from ${root}: ${importPath}` };
    }
  };
}

/** Compiles every `.sol` file of `sourcesDir` and writes the deployable contracts to `outDir` (replaced). */
export function buildContracts(sourcesDir: string, outDir: string, packageRoot: string): BuildResult {
  const solc = require('solc') as Solc;
  const files = readdirSync(sourcesDir)
    .filter((file) => file.endsWith('.sol'))
    .sort();
  if (files.length === 0) throw new Error(`no .sol file in ${sourcesDir}`);
  const sources = Object.fromEntries(
    files.map((file) => [`contracts/${file}`, { content: readFileSync(path.join(sourcesDir, file), 'utf8') }]),
  );
  const input = { language: 'Solidity', sources, settings: SOLC_SETTINGS };
  const output = JSON.parse(solc.compile(JSON.stringify(input), { import: importResolver(packageRoot) })) as SolcOutput;

  const problems = output.errors ?? [];
  const errors = problems.filter((problem) => problem.severity === 'error');
  if (errors.length > 0) {
    throw new Error(`solc ${solc.version()} failed:\n${errors.map((error) => error.formattedMessage).join('\n')}`);
  }

  rmSync(outDir, { recursive: true, force: true });
  const written: string[] = [];
  for (const [sourceName, contracts] of Object.entries(output.contracts ?? {})) {
    // Only the engine's own sources: the libraries they import are not artifacts of the engine.
    if (!sourceName.startsWith('contracts/')) continue;
    for (const [contractName, contract] of Object.entries(contracts)) {
      // Abstract contracts and interfaces cannot be deployed.
      if (contract.evm.bytecode.object === '') continue;
      const dir = path.join(outDir, path.basename(sourceName));
      mkdirSync(dir, { recursive: true });
      const artifact = {
        _format: 'hh-sol-artifact-1',
        contractName,
        sourceName,
        abi: contract.abi,
        bytecode: `0x${contract.evm.bytecode.object}`,
        deployedBytecode: `0x${contract.evm.deployedBytecode.object}`,
        linkReferences: {},
        deployedLinkReferences: {},
      };
      writeFileSync(path.join(dir, `${contractName}.json`), `${JSON.stringify(artifact, null, 2)}\n`);
      written.push(contractName);
    }
  }
  return {
    compiler: solc.version(),
    contracts: written.sort(),
    warnings: problems.filter((problem) => problem.severity === 'warning').map((problem) => problem.formattedMessage),
  };
}
