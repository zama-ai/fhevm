/*
 * Builds the SDK-flavored TFHE wrapper from the raw npm `tfhe.js` file.
 *
 * 1. Infer the TFHE version from `--version`, `package.json`, or the `vX.Y.Z` output directory, then resolve output paths.
 * 2. Run `prune-wbg-init.ts` to generate `tfhe-worker.mjs`.
 * 3. Patch raw `tfhe.js` so wasm loading and worker startup are SDK-controlled.
 * 4. Render `tfhe.js` from `tfhe-js.template.js` with SDK imports, exports, and download metadata.
 * 5. Render `startWorkers.js` from `startWorkers.template.js`, embedding `tfhe-worker.mjs` as base64.
 * 6. Render `tfhe.d.ts` from `tfhe-dts.template.d.ts` with SDK helper declarations when it exists.
 * 7. Render `type-check.test.ts` from `type-check.test.template.ts` when `tfhe.d.ts` exists.
 *
 * Template placeholders:
 * - `__TFHE_START_WORKERS_IMPORT__`
 * - `__TFHE_VERSION_JSON__`
 * - `__TFHE_DOWNLOAD_FILES_JSON__`
 * - `/* __TFHE_JS_BODY__ *\/`
 * - `/* __TFHE_DTS_BODY__ *\/`
 * - `__TFHE_TYPE_CHECK_BANNER__`
 * - `__TFHE_JS_IMPORT__`
 * - `__TFHE_API_IMPORT__`
 * - `__CORE_TYPES_FHEVM_RUNTIME_IMPORT__`
 * - `__TFHE_WORKER_URL_SHA256_JSON__`
 * - `__TFHE_WORKER_BASE64_JSON__`
 * - `__TFHE_WORKER_BASE64_SHA256__`
 */
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { basename, dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import ts from 'typescript';

const scriptDir = dirname(fileURLToPath(import.meta.url));
const pruneScriptPath = resolve(scriptDir, 'prune-wbg-init.ts');
const startWorkersTemplatePath = resolve(scriptDir, 'startWorkers.template.js');
const tfheJsTemplatePath = resolve(scriptDir, 'tfhe-js.template.js');
const tfheDtsTemplatePath = resolve(scriptDir, 'tfhe-dts.template.d.ts');
const typeCheckTemplatePath = resolve(scriptDir, 'type-check.test.template.ts');
const typeCheckOutputFileName = 'type-check.test.ts';
const startWorkersImportDefault = './startWorkers.js';
const startWorkersImportPlaceholder = '__TFHE_START_WORKERS_IMPORT__';
const tfheVersionJsonPlaceholder = '__TFHE_VERSION_JSON__';
const tfheDownloadFilesJsonPlaceholder = '__TFHE_DOWNLOAD_FILES_JSON__';
const tfheJsBodyPlaceholder = '/* __TFHE_JS_BODY__ */';
const tfheDtsBodyPlaceholder = '/* __TFHE_DTS_BODY__ */';
const typeCheckBannerPlaceholder = '__TFHE_TYPE_CHECK_BANNER__';
const typeCheckTfheJsImportPlaceholder = '__TFHE_JS_IMPORT__';
const typeCheckTfheApiImportPlaceholder = '__TFHE_API_IMPORT__';
const typeCheckCoreTypesImportPlaceholder = '__CORE_TYPES_FHEVM_RUNTIME_IMPORT__';
const typeCheckTfheJsImportDefault = './tfhe.js';
const typeCheckTfheApiImportDefault = '../TfheApi.js';
const typeCheckCoreTypesImportDefault = '../../../core/types/coreFhevmRuntime.js';
const workerUrlSha256JsonPlaceholder = '__TFHE_WORKER_URL_SHA256_JSON__';
const workerBase64JsonPlaceholder = '__TFHE_WORKER_BASE64_JSON__';
const workerBase64Sha256Placeholder = '__TFHE_WORKER_BASE64_SHA256__';

type DownloadFileInfo = {
  filename: string;
  sha256: string;
};

type ParsedArgs = {
  input: string;
  outputDir?: string;
  version?: string;
  startWorkersImport: string;
  patchTypes: boolean;
};

const usage = [
  'Usage:',
  '  node scripts/wasm/tfhe/build-tfhe.ts <input-tfhe.js> [output-dir] [--version <x.y.z>]',
  '',
  'Outputs:',
  '  <output-dir>/tfhe.js',
  '  <output-dir>/tfhe-worker.mjs',
  '  <output-dir>/startWorkers.js',
  '  <output-dir>/type-check.test.ts  when <output-dir>/tfhe.d.ts exists',
  '',
  'Defaults:',
  `  output-dir             dirname(<input-tfhe.js>)`,
  `  --start-workers-import ${startWorkersImportDefault}`,
  '  --patch-types          enabled when <output-dir>/tfhe.d.ts exists',
].join('\n');

function parseArgs(argv: string[]): ParsedArgs {
  const positional: string[] = [];
  let version: string | undefined;
  let startWorkersImport = startWorkersImportDefault;
  let patchTypes = true;

  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i];

    if (arg === '--version') {
      version = argv[++i];
      if (!version) {
        throw new Error(`Missing value for ${arg}\n\n${usage}`);
      }
      continue;
    }

    if (arg === '--start-workers-import') {
      startWorkersImport = argv[++i];
      if (!startWorkersImport) {
        throw new Error(`Missing value for ${arg}\n\n${usage}`);
      }
      continue;
    }

    if (arg === '--no-patch-types') {
      patchTypes = false;
      continue;
    }

    if (arg === '--help' || arg === '-h') {
      console.log(usage);
      process.exit(0);
    }

    if (arg.startsWith('--')) {
      throw new Error(`Unknown option: ${arg}\n\n${usage}`);
    }

    positional.push(arg);
  }

  if (positional.length === 0 || positional.length > 2) {
    throw new Error(usage);
  }

  return {
    input: positional[0],
    outputDir: positional[1],
    version,
    startWorkersImport,
    patchTypes,
  };
}

function fail(message: string): never {
  throw new Error(message);
}

function renderTemplateText(template: string, label: string, replacements: ReadonlyMap<string, string>): string {
  for (const [placeholder, replacement] of replacements) {
    if (!template.includes(placeholder)) {
      fail(`Missing ${placeholder} in ${label}.`);
    }

    template = template.replaceAll(placeholder, replacement);
  }

  return template.trimEnd();
}

function renderTemplate(path: string, replacements: ReadonlyMap<string, string>): string {
  return renderTemplateText(readFileSync(path, 'utf8'), path, replacements);
}

function escapeSingleQuotedStringContent(value: string): string {
  return value
    .replace(/\\/g, '\\\\')
    .replace(/'/g, "\\'")
    .replace(/\r/g, '\\r')
    .replace(/\n/g, '\\n')
    .replace(/\u2028/g, '\\u2028')
    .replace(/\u2029/g, '\\u2029');
}

function parseJs(path: string, text: string): ts.SourceFile {
  return ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
}

function readJsonFile(path: string): unknown {
  return JSON.parse(readFileSync(path, 'utf8'));
}

function renderDownloadFiles(downloadFiles: readonly DownloadFileInfo[]): string {
  const entries = downloadFiles
    .map((file) => {
      return [
        '      {',
        `        filename: ${JSON.stringify(file.filename)},`,
        `        sha256: ${JSON.stringify(file.sha256)},`,
        '      }',
      ].join('\n');
    })
    .join(',\n');

  return `[\n${entries}\n    ]`;
}

function sha256File(path: string): string {
  return createHash('sha256').update(readFileSync(path)).digest('hex');
}

function inferVersion(inputPath: string, outputDir: string, explicitVersion?: string): string {
  if (explicitVersion) {
    return explicitVersion.replace(/^v/, '');
  }

  for (const dir of [outputDir, dirname(inputPath)]) {
    const packagePath = resolve(dir, 'package.json');

    if (existsSync(packagePath)) {
      const json = readJsonFile(packagePath);

      if (typeof json === 'object' && json && 'version' in json && typeof json.version === 'string') {
        return json.version.replace(/^v/, '');
      }
    }
  }

  const directoryName = basename(outputDir);
  const match = /^v(.+)$/.exec(directoryName);

  if (match) {
    return match[1];
  }

  fail('Unable to infer TFHE version. Pass --version <x.y.z>.');
}

function importDeclarationHasLocalName(statement: ts.ImportDeclaration, localName: string): boolean {
  const importClause = statement.importClause;

  if (!importClause) {
    return false;
  }

  if (importClause.name?.text === localName) {
    return true;
  }

  const namedBindings = importClause.namedBindings;

  return (
    !!namedBindings &&
    ts.isNamedImports(namedBindings) &&
    namedBindings.elements.some((element) => element.name.text === localName)
  );
}

function isModulePathUndefinedCheck(statement: ts.Statement): boolean {
  if (!ts.isIfStatement(statement) || !ts.isBinaryExpression(statement.expression)) {
    return false;
  }

  const { left, operatorToken, right } = statement.expression;

  return (
    ts.isIdentifier(left) &&
    left.text === 'module_or_path' &&
    operatorToken.kind === ts.SyntaxKind.EqualsEqualsEqualsToken &&
    ((ts.isIdentifier(right) && right.text === 'undefined') || right.kind === ts.SyntaxKind.UndefinedKeyword)
  );
}

function containsModulePathFetchAssignment(node: ts.Node): boolean {
  let found = false;

  function visit(current: ts.Node) {
    if (found) {
      return;
    }

    if (
      ts.isBinaryExpression(current) &&
      current.operatorToken.kind === ts.SyntaxKind.EqualsToken &&
      ts.isIdentifier(current.left) &&
      current.left.text === 'module_or_path' &&
      ts.isCallExpression(current.right) &&
      ts.isIdentifier(current.right.expression) &&
      current.right.expression.text === 'fetch'
    ) {
      found = true;
      return;
    }

    ts.forEachChild(current, visit);
  }

  visit(node);
  return found;
}

function isModulePathFetchCheck(statement: ts.Statement): boolean {
  return ts.isIfStatement(statement) && containsModulePathFetchAssignment(statement.thenStatement);
}

function isRawWasmBindgenDefaultExport(statement: ts.Statement): statement is ts.ExportDeclaration {
  if (!ts.isExportDeclaration(statement) || !statement.exportClause || !ts.isNamedExports(statement.exportClause)) {
    return false;
  }

  return statement.exportClause.elements.some((element) => {
    return element.propertyName?.text === '__wbg_init' && element.name.text === 'default';
  });
}

type Replacement = {
  start: number;
  end: number;
  text: string;
};

function applyReplacements(text: string, replacements: Replacement[]): string {
  const sorted = [...replacements].sort((a, b) => b.start - a.start);
  let next = text;

  for (const replacement of sorted) {
    next = `${next.slice(0, replacement.start)}${replacement.text}${next.slice(replacement.end)}`;
  }

  return next;
}

function commentOutStatement(sourceText: string, sourceFile: ts.SourceFile, statement: ts.Statement): Replacement {
  const statementStart = statement.getStart(sourceFile);
  const start = sourceText.lastIndexOf('\n', statementStart - 1) + 1;
  const end = statement.getEnd();
  const statementText = sourceText
    .slice(start, end)
    .replace(/[ \t]+$/gm, '')
    .trimEnd();
  const lines = statementText.split(/\r?\n/);
  const indents = lines.filter((line) => line.trim().length > 0).map((line) => /^ */.exec(line)?.[0].length || 0);
  const indent = Math.min(...indents);
  const prefix = ' '.repeat(indent);
  const commented = lines
    .map((line) => {
      const content = line.slice(indent);
      return `${prefix}// ${content}`;
    })
    .join('\n');

  return { start, end, text: commented };
}

function stripLeadingSelfTypesComment(sourceText: string): string {
  return sourceText.replace(/^\s*\/\*\s*@ts-self-types=["'][^"']+["']\s*\*\/\s*/, '');
}

function renderTfheJs(
  body: string,
  version: string,
  startWorkersImport: string,
  downloadFiles: readonly DownloadFileInfo[],
): string {
  return renderTemplate(
    tfheJsTemplatePath,
    new Map([
      [startWorkersImportPlaceholder, escapeSingleQuotedStringContent(startWorkersImport)],
      [tfheVersionJsonPlaceholder, JSON.stringify(version)],
      [tfheDownloadFilesJsonPlaceholder, renderDownloadFiles(downloadFiles)],
      [tfheJsBodyPlaceholder, body],
    ]),
  );
}

function stripGeneratedDtsHelpers(sourceText: string): string {
  return sourceText.replace(/\n+\/{80}\n\nexport function getWasmInfo\(\):[\s\S]*$/g, '').trim();
}

function renderTfheDts(body: string): string {
  return renderTemplate(tfheDtsTemplatePath, new Map([[tfheDtsBodyPlaceholder, body]]));
}

function renderTypeCheckTest(outputDir: string): string {
  const banner = `sdk/js-sdk/src/wasm/tfhe/${basename(outputDir)}/${typeCheckOutputFileName}`;

  return renderTemplate(
    typeCheckTemplatePath,
    new Map([
      [typeCheckBannerPlaceholder, banner],
      [typeCheckTfheJsImportPlaceholder, typeCheckTfheJsImportDefault],
      [typeCheckTfheApiImportPlaceholder, typeCheckTfheApiImportDefault],
      [typeCheckCoreTypesImportPlaceholder, typeCheckCoreTypesImportDefault],
    ]),
  );
}

function patchTfheJs(
  sourceText: string,
  inputPath: string,
  version: string,
  startWorkersImport: string,
  downloadFiles: readonly DownloadFileInfo[],
): string {
  const sourceFile = parseJs(inputPath, sourceText);
  const replacements: Replacement[] = [];
  const startWorkersImports = sourceFile.statements.filter((statement): statement is ts.ImportDeclaration => {
    return ts.isImportDeclaration(statement) && importDeclarationHasLocalName(statement, 'startWorkers');
  });

  if (startWorkersImports.length !== 1) {
    fail(`Expected exactly one import containing startWorkers, found ${startWorkersImports.length}.`);
  }

  replacements.push({
    start: startWorkersImports[0].getStart(sourceFile),
    end: startWorkersImports[0].getEnd(),
    text: '',
  });

  const initFunction = sourceFile.statements.find((statement): statement is ts.FunctionDeclaration => {
    return ts.isFunctionDeclaration(statement) && statement.name?.text === '__wbg_init';
  });

  if (!initFunction?.body) {
    fail('Expected exactly one __wbg_init function with a body.');
  }

  const defaultModulePathBlocks = initFunction.body.statements.filter(isModulePathUndefinedCheck);
  const fetchModulePathBlocks = initFunction.body.statements.filter(isModulePathFetchCheck);

  if (defaultModulePathBlocks.length !== 1) {
    fail(`Expected one default module_or_path block in __wbg_init, found ${defaultModulePathBlocks.length}.`);
  }

  if (fetchModulePathBlocks.length !== 1) {
    fail(`Expected one fetch module_or_path block in __wbg_init, found ${fetchModulePathBlocks.length}.`);
  }

  replacements.push(commentOutStatement(sourceText, sourceFile, defaultModulePathBlocks[0]));
  replacements.push(commentOutStatement(sourceText, sourceFile, fetchModulePathBlocks[0]));

  const rawDefaultExports = sourceFile.statements.filter(isRawWasmBindgenDefaultExport);

  if (rawDefaultExports.length !== 1) {
    fail(`Expected exactly one raw wasm-bindgen default export, found ${rawDefaultExports.length}.`);
  }

  replacements.push({
    start: rawDefaultExports[0].getStart(sourceFile),
    end: rawDefaultExports[0].getEnd(),
    text: '',
  });

  const body = stripLeadingSelfTypesComment(applyReplacements(sourceText, replacements)).trim();
  return renderTfheJs(body, version, startWorkersImport, downloadFiles);
}

function runPruner(inputPath: string, workerOutputPath: string): void {
  const result = spawnSync(process.execPath, [pruneScriptPath, inputPath, workerOutputPath], {
    stdio: 'inherit',
  });

  if (result.error) {
    throw result.error;
  }

  if (result.status !== 0) {
    fail(`prune-wbg-init.ts failed with exit code ${result.status ?? 'unknown'}.`);
  }
}

function generateStartWorkers(workerOutputPath: string): {
  code: string;
  workerSha256: string;
  base64Sha256: string;
  base64Length: number;
} {
  const template = readFileSync(startWorkersTemplatePath, 'utf8');

  if (!template.includes(workerUrlSha256JsonPlaceholder)) {
    fail(`Missing ${workerUrlSha256JsonPlaceholder} in ${startWorkersTemplatePath}.`);
  }

  if (!template.includes(workerBase64JsonPlaceholder)) {
    fail(`Missing ${workerBase64JsonPlaceholder} in ${startWorkersTemplatePath}.`);
  }

  if (!template.includes(workerBase64Sha256Placeholder)) {
    fail(`Missing ${workerBase64Sha256Placeholder} in ${startWorkersTemplatePath}.`);
  }

  const workerCode = readFileSync(workerOutputPath, 'utf8');
  const base64 = Buffer.from(workerCode, 'utf8').toString('base64');
  const workerSha256 = sha256File(workerOutputPath);
  const base64Sha256 = createHash('sha256').update(base64).digest('hex');
  const code = template
    .replaceAll(workerUrlSha256JsonPlaceholder, JSON.stringify(workerSha256))
    .replace(workerBase64JsonPlaceholder, JSON.stringify(base64))
    .replaceAll(workerBase64Sha256Placeholder, base64Sha256);

  return { code, workerSha256, base64Sha256, base64Length: base64.length };
}

function collectDownloadFiles(outputDir: string, workerOutputPath: string): DownloadFileInfo[] {
  const wasmPath = resolve(outputDir, 'tfhe_bg.wasm');

  if (!existsSync(wasmPath)) {
    fail(`Expected downloadable wasm file: ${wasmPath}`);
  }

  if (!existsSync(workerOutputPath)) {
    fail(`Expected downloadable worker file: ${workerOutputPath}`);
  }

  return [
    {
      filename: basename(wasmPath),
      sha256: sha256File(wasmPath),
    },
    {
      filename: basename(workerOutputPath),
      sha256: sha256File(workerOutputPath),
    },
  ];
}

function patchDtsIfPresent(outputDir: string, patchTypes: boolean): boolean {
  if (!patchTypes) {
    return false;
  }

  const dtsPath = resolve(outputDir, 'tfhe.d.ts');

  if (!existsSync(dtsPath)) {
    return false;
  }

  const source = readFileSync(dtsPath, 'utf8');
  const body = stripGeneratedDtsHelpers(source);
  writeFileSync(dtsPath, `${renderTfheDts(body)}\n`);
  return true;
}

const args = parseArgs(process.argv.slice(2));
const inputPath = resolve(process.cwd(), args.input);

if (!existsSync(inputPath)) {
  fail(`Input file does not exist: ${inputPath}`);
}

const outputDir = resolve(process.cwd(), args.outputDir || dirname(inputPath));
const version = inferVersion(inputPath, outputDir, args.version);
const tfheOutputPath = resolve(outputDir, 'tfhe.js');
const workerOutputPath = resolve(outputDir, 'tfhe-worker.mjs');
const startWorkersOutputPath = resolve(outputDir, 'startWorkers.js');
const typeCheckOutputPath = resolve(outputDir, typeCheckOutputFileName);

mkdirSync(outputDir, { recursive: true });

const sourceText = readFileSync(inputPath, 'utf8');
runPruner(inputPath, workerOutputPath);

const downloadFiles = collectDownloadFiles(outputDir, workerOutputPath);
const startWorkers = generateStartWorkers(workerOutputPath);
const patchedTfhe = patchTfheJs(sourceText, inputPath, version, args.startWorkersImport, downloadFiles);

writeFileSync(tfheOutputPath, patchedTfhe.endsWith('\n') ? patchedTfhe : `${patchedTfhe}\n`);
writeFileSync(startWorkersOutputPath, startWorkers.code.endsWith('\n') ? startWorkers.code : `${startWorkers.code}\n`);
const patchedTypes = patchDtsIfPresent(outputDir, args.patchTypes);

if (patchedTypes) {
  const typeCheckTest = renderTypeCheckTest(outputDir);
  writeFileSync(typeCheckOutputPath, typeCheckTest.endsWith('\n') ? typeCheckTest : `${typeCheckTest}\n`);
}

console.log(`Wrote ${tfheOutputPath}`);
console.log(`Wrote ${workerOutputPath}`);
console.log(`Wrote ${startWorkersOutputPath}`);
console.log(`Worker file SHA-256: ${startWorkers.workerSha256}`);
console.log(`Embedded worker base64: ${startWorkers.base64Length} chars`);
console.log(`Embedded worker base64 SHA-256: ${startWorkers.base64Sha256}`);

if (patchedTypes) {
  console.log(`Patched ${resolve(outputDir, 'tfhe.d.ts')}`);
  console.log(`Wrote ${typeCheckOutputPath}`);
}
