/*
 * Builds the SDK-flavored TKMS wrapper from the raw npm `tkms/kms_lib.js` file.
 *
 * 1. Infer the TKMS version from `--version`, `package.json`, or the `vX.Y.Z` output directory.
 * 2. Patch raw `kms_lib.js` so wasm loading is SDK-controlled.
 * 3. Back-port the optional `threshold` arg for versions older than 0.13.20-0.
 * 4. Render `kms_lib.js` and `kms_lib.d.ts` from templates.
 * 5. Render `type-check.test.ts` from a template when `kms_lib.d.ts` exists.
 *
 * Template placeholders:
 * - `__KMS_VERSION__`
 * - `__KMS_DOWNLOAD_FILES_JSON__`
 * - `/* __KMS_JS_BODY__ *\/`
 * - `/* __KMS_DTS_BODY__ *\/`
 * - `__KMS_TYPE_CHECK_BANNER__`
 * - `__KMS_LIB_IMPORT__`
 * - `__KMS_API_IMPORT__`
 */
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { basename, dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

import ts from 'typescript';

const scriptDir = dirname(fileURLToPath(import.meta.url));
const kmsJsTemplatePath = resolve(scriptDir, 'kms-js.template.js');
const kmsDtsTemplatePath = resolve(scriptDir, 'kms-dts.template.d.ts');
const typeCheckTemplatePath = resolve(scriptDir, 'type-check.test.template.ts');
const typeCheckOutputFileName = 'type-check.test.ts';
const kmsVersionPlaceholder = '__KMS_VERSION__';
const kmsDownloadFilesJsonPlaceholder = '__KMS_DOWNLOAD_FILES_JSON__';
const kmsJsBodyPlaceholder = '/* __KMS_JS_BODY__ */';
const kmsDtsBodyPlaceholder = '/* __KMS_DTS_BODY__ */';
const typeCheckBannerPlaceholder = '__KMS_TYPE_CHECK_BANNER__';
const typeCheckKmsLibImportPlaceholder = '__KMS_LIB_IMPORT__';
const typeCheckKmsApiImportPlaceholder = '__KMS_API_IMPORT__';
const typeCheckKmsLibImportDefault = './kms_lib.js';
const typeCheckKmsApiImportDefault = '../KmsLibApi.js';

type ParsedArgs = {
  input: string;
  outputDir?: string;
  version?: string;
  patchTypes: boolean;
};

type Replacement = {
  start: number;
  end: number;
  text: string;
};

type DownloadFileInfo = {
  filename: string;
  sha256: string;
};

const usage = [
  'Usage:',
  '  node scripts/wasm/kms/build-kms.ts <input-kms_lib.js> [output-dir] [--version <x.y.z>]',
  '',
  'Outputs:',
  '  <output-dir>/kms_lib.js',
  '  <output-dir>/kms_lib.d.ts       when <output-dir>/kms_lib.d.ts exists',
  '  <output-dir>/type-check.test.ts when <output-dir>/kms_lib.d.ts exists',
  '',
  'Defaults:',
  `  output-dir    dirname(<input-kms_lib.js>)`,
  '  --patch-types enabled when <output-dir>/kms_lib.d.ts exists',
].join('\n');

function parseArgs(argv: string[]): ParsedArgs {
  const positional: string[] = [];
  let version: string | undefined;
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

function parseJs(path: string, text: string): ts.SourceFile {
  return ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
}

function parseTs(path: string, text: string): ts.SourceFile {
  return ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
}

function readJsonFile(path: string): unknown {
  return JSON.parse(readFileSync(path, 'utf8'));
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

  fail('Unable to infer TKMS version. Pass --version <x.y.z>.');
}

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

function escapeSingleQuotedStringContent(value: string): string {
  return value
    .replace(/\\/g, '\\\\')
    .replace(/'/g, "\\'")
    .replace(/\r/g, '\\r')
    .replace(/\n/g, '\\n')
    .replace(/\u2028/g, '\\u2028')
    .replace(/\u2029/g, '\\u2029');
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

function stripGeneratedDtsHelpers(sourceText: string): string {
  return sourceText.replace(/\n+\/{80}\n\nexport function getWasmInfo\(\):[\s\S]*$/g, '').trim();
}

function getIdentifierText(name: ts.BindingName | undefined): string | undefined {
  return name && ts.isIdentifier(name) ? name.text : undefined;
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

function isProcessUserDecryptionFromJs(
  statement: ts.Statement,
): statement is ts.FunctionDeclaration & { name: ts.Identifier } {
  return ts.isFunctionDeclaration(statement) && statement.name?.text === 'process_user_decryption_resp_from_js';
}

function isGeneratedGetWasmInfo(
  statement: ts.Statement,
): statement is ts.FunctionDeclaration & { name: ts.Identifier } {
  return ts.isFunctionDeclaration(statement) && statement.name?.text === 'getWasmInfo';
}

function addJsThresholdArgIfMissing(
  sourceText: string,
  sourceFile: ts.SourceFile,
  fn: ts.FunctionDeclaration,
): Replacement[] {
  const parameterNames = fn.parameters.map((parameter) => getIdentifierText(parameter.name));

  if (parameterNames.includes('threshold')) {
    return [];
  }

  const verifyParameter = fn.parameters.find((parameter) => getIdentifierText(parameter.name) === 'verify');

  if (!verifyParameter) {
    fail('Expected process_user_decryption_resp_from_js to have a verify parameter.');
  }

  const commentRanges = ts.getLeadingCommentRanges(sourceText, fn.pos) ?? [];
  const jsDoc = [...commentRanges].reverse().find((range) => sourceText.slice(range.pos, range.end).startsWith('/**'));
  const replacements: Replacement[] = [
    {
      start: verifyParameter.getStart(sourceFile),
      end: verifyParameter.getStart(sourceFile),
      text: 'threshold, ',
    },
  ];

  if (jsDoc) {
    const commentText = sourceText.slice(jsDoc.pos, jsDoc.end);

    if (!commentText.includes('@param {number | null | undefined} threshold')) {
      const verifyParamIndex = commentText.lastIndexOf(' * @param {boolean} verify');

      if (verifyParamIndex < 0) {
        fail('Expected process_user_decryption_resp_from_js JSDoc to document the verify parameter.');
      }

      replacements.push({
        start: jsDoc.pos + verifyParamIndex,
        end: jsDoc.pos + verifyParamIndex,
        text: ' * @param {number | null | undefined} threshold\n',
      });
    }
  }

  return replacements;
}

function addDtsThresholdArgIfMissing(
  sourceText: string,
  sourceFile: ts.SourceFile,
  fn: ts.FunctionDeclaration,
): Replacement[] {
  const parameterNames = fn.parameters.map((parameter) => getIdentifierText(parameter.name));

  if (parameterNames.includes('threshold')) {
    return [];
  }

  const verifyParameter = fn.parameters.find((parameter) => getIdentifierText(parameter.name) === 'verify');

  if (!verifyParameter) {
    fail('Expected process_user_decryption_resp_from_js declaration to have a verify parameter.');
  }

  const insertAt = verifyParameter.getStart(sourceFile);
  const lineStart = sourceText.lastIndexOf('\n', insertAt - 1) + 1;
  const indent = sourceText.slice(lineStart, insertAt);

  return [
    {
      start: insertAt,
      end: insertAt,
      text: `${indent}// Not in original version. Make it compatible with v0.13.20\n${indent}threshold: number | null | undefined,\n${indent}`,
    },
  ];
}

function renderKmsJs(body: string, version: string, downloadFiles: readonly DownloadFileInfo[]): string {
  return renderTemplate(
    kmsJsTemplatePath,
    new Map([
      [kmsVersionPlaceholder, escapeSingleQuotedStringContent(version)],
      [kmsDownloadFilesJsonPlaceholder, renderDownloadFiles(downloadFiles)],
      [kmsJsBodyPlaceholder, body],
    ]),
  );
}

function renderKmsDts(body: string): string {
  return renderTemplate(kmsDtsTemplatePath, new Map([[kmsDtsBodyPlaceholder, body]]));
}

function renderTypeCheckTest(outputDir: string): string {
  const banner = `sdk/js-sdk/src/wasm/tkms/${basename(outputDir)}/${typeCheckOutputFileName}`;

  return renderTemplate(
    typeCheckTemplatePath,
    new Map([
      [typeCheckBannerPlaceholder, banner],
      [typeCheckKmsLibImportPlaceholder, typeCheckKmsLibImportDefault],
      [typeCheckKmsApiImportPlaceholder, typeCheckKmsApiImportDefault],
    ]),
  );
}

function patchKmsJs(
  sourceText: string,
  inputPath: string,
  version: string,
  downloadFiles: readonly DownloadFileInfo[],
): string {
  const sourceFile = parseJs(inputPath, sourceText);
  const replacements: Replacement[] = [];
  const initFunction = sourceFile.statements.find((statement): statement is ts.FunctionDeclaration => {
    return ts.isFunctionDeclaration(statement) && statement.name?.text === '__wbg_init';
  });

  if (!initFunction?.body) {
    fail('Expected exactly one __wbg_init function with a body.');
  }

  const defaultModulePathBlocks = initFunction.body.statements.filter(isModulePathUndefinedCheck);
  const fetchModulePathBlocks = initFunction.body.statements.filter(isModulePathFetchCheck);

  if (defaultModulePathBlocks.length === 1) {
    replacements.push(commentOutStatement(sourceText, sourceFile, defaultModulePathBlocks[0]));
  } else if (!sourceText.includes(`// if (module_or_path === undefined)`)) {
    fail(`Expected one default module_or_path block in __wbg_init, found ${defaultModulePathBlocks.length}.`);
  }

  if (fetchModulePathBlocks.length === 1) {
    replacements.push(commentOutStatement(sourceText, sourceFile, fetchModulePathBlocks[0]));
  } else if (!sourceText.includes(`// if (typeof module_or_path === 'string'`)) {
    fail(`Expected one fetch module_or_path block in __wbg_init, found ${fetchModulePathBlocks.length}.`);
  }

  const rawDefaultExports = sourceFile.statements.filter(isRawWasmBindgenDefaultExport);

  if (rawDefaultExports.length !== 1) {
    fail(`Expected exactly one raw wasm-bindgen default export, found ${rawDefaultExports.length}.`);
  }

  replacements.push({
    start: rawDefaultExports[0].getStart(sourceFile),
    end: rawDefaultExports[0].getEnd(),
    text: '',
  });

  const generatedGetWasmInfo = sourceFile.statements.filter(isGeneratedGetWasmInfo);

  if (generatedGetWasmInfo.length > 1) {
    fail(`Expected at most one generated getWasmInfo function, found ${generatedGetWasmInfo.length}.`);
  }

  if (generatedGetWasmInfo.length === 1) {
    replacements.push({
      start: generatedGetWasmInfo[0].getFullStart(),
      end: generatedGetWasmInfo[0].getEnd(),
      text: '',
    });
  }

  const decryptionFunction = sourceFile.statements.find(isProcessUserDecryptionFromJs);

  if (!decryptionFunction) {
    fail('Expected process_user_decryption_resp_from_js function in kms_lib.js.');
  }

  replacements.push(...addJsThresholdArgIfMissing(sourceText, sourceFile, decryptionFunction));

  const body = stripLeadingSelfTypesComment(applyReplacements(sourceText, replacements)).trim();
  return renderKmsJs(body, version, downloadFiles);
}

function collectDownloadFiles(outputDir: string): DownloadFileInfo[] {
  const wasmPath = resolve(outputDir, 'kms_lib_bg.wasm');

  if (!existsSync(wasmPath)) {
    fail(`Expected downloadable wasm file: ${wasmPath}`);
  }

  return [
    {
      filename: basename(wasmPath),
      sha256: sha256File(wasmPath),
    },
  ];
}

function patchDtsIfPresent(outputDir: string, patchTypes: boolean): boolean {
  if (!patchTypes) {
    return false;
  }

  const dtsPath = resolve(outputDir, 'kms_lib.d.ts');

  if (!existsSync(dtsPath)) {
    return false;
  }

  const sourceText = stripGeneratedDtsHelpers(readFileSync(dtsPath, 'utf8'));
  const sourceFile = parseTs(dtsPath, sourceText);
  const decryptionFunction = sourceFile.statements.find(isProcessUserDecryptionFromJs);

  if (!decryptionFunction) {
    fail('Expected process_user_decryption_resp_from_js declaration in kms_lib.d.ts.');
  }

  const patchedBody = applyReplacements(
    sourceText,
    addDtsThresholdArgIfMissing(sourceText, sourceFile, decryptionFunction),
  );
  writeFileSync(dtsPath, `${renderKmsDts(patchedBody.trim())}\n`);
  return true;
}

const args = parseArgs(process.argv.slice(2));
const inputPath = resolve(process.cwd(), args.input);

if (!existsSync(inputPath)) {
  fail(`Input file does not exist: ${inputPath}`);
}

const outputDir = resolve(process.cwd(), args.outputDir || dirname(inputPath));
const version = inferVersion(inputPath, outputDir, args.version);
const kmsOutputPath = resolve(outputDir, 'kms_lib.js');
const typeCheckOutputPath = resolve(outputDir, typeCheckOutputFileName);

mkdirSync(outputDir, { recursive: true });

const sourceText = readFileSync(inputPath, 'utf8');
const downloadFiles = collectDownloadFiles(outputDir);
const patchedKms = patchKmsJs(sourceText, inputPath, version, downloadFiles);

writeFileSync(kmsOutputPath, patchedKms.endsWith('\n') ? patchedKms : `${patchedKms}\n`);
const patchedTypes = patchDtsIfPresent(outputDir, args.patchTypes);

if (patchedTypes) {
  const typeCheckTest = renderTypeCheckTest(outputDir);
  writeFileSync(typeCheckOutputPath, typeCheckTest.endsWith('\n') ? typeCheckTest : `${typeCheckTest}\n`);
}

console.log(`Wrote ${kmsOutputPath}`);

if (patchedTypes) {
  console.log(`Patched ${resolve(outputDir, 'kms_lib.d.ts')}`);
  console.log(`Wrote ${typeCheckOutputPath}`);
}
