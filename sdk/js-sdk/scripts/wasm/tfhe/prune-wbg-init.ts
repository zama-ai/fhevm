/*
 * Builds the pruned TFHE worker module from the raw npm `tfhe.js` file.
 *
 * The npm `tfhe.js` file is the full wasm-bindgen wrapper for the main thread:
 * it exports the whole public API, knows how to spawn Rayon workers, and carries
 * every helper needed by those APIs. A worker thread only needs a much smaller
 * shape: initialize the wasm module with `__wbg_init`, announce readiness, then
 * call `wbg_rayon_start_worker`.
 *
 * This script derives that worker module from the upstream generated wrapper
 * instead of maintaining a hand-written fork. That keeps the worker synchronized
 * with future wasm-bindgen output, removes unused public API glue, blocks nested
 * worker startup, and fails loudly when new or missing `__wbg_*` shims indicate
 * that the generated wrapper shape has changed.
 *
 * 1. Parse the input file and use `__wbg_init` as the default recursive root.
 * 2. Validate `__wbg_get_imports` and fail on missing or unknown wasm-bindgen import shims.
 * 3. Rewrite the `__wbg_startWorkers_*` shim to throw inside worker threads.
 * 4. Comment out SDK-controlled `module_or_path` URL and `fetch` logic in `__wbg_init`.
 * 5. Keep only top-level declarations recursively needed by the root functions.
 * 6. Add readable section separators and render the body into `tfhe-worker.template.mjs`.
 * 7. Write the worker module to the output file, or print it to stdout.
 */
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import ts from 'typescript';

const scriptDir = dirname(fileURLToPath(import.meta.url));
const workerTemplatePath = resolve(scriptDir, 'tfhe-worker.template.mjs');
const workerBodyPlaceholder = '/* __TFHE_WORKER_BODY__ */';

type ParsedArgs = {
  input: string;
  output?: string;
  roots: Set<string>;
  keptExports: Set<string>;
};

type DeclarationIndexes = {
  declarations: Map<string, ts.Statement>;
  imports: Map<string, ts.ImportDeclaration>;
  topLevelNames: Set<string>;
};

const expectedWbgImportNames = new Set<string>([
  '__wbg_BigInt_*',
  '__wbg_Error_*',
  '__wbg___wbindgen_bigint_get_as_i64_*',
  '__wbg___wbindgen_bit_and_*',
  '__wbg___wbindgen_debug_string_*',
  '__wbg___wbindgen_is_function_*',
  '__wbg___wbindgen_is_object_*',
  '__wbg___wbindgen_is_string_*',
  '__wbg___wbindgen_is_undefined_*',
  '__wbg___wbindgen_jsval_eq_*',
  '__wbg___wbindgen_lt_*',
  '__wbg___wbindgen_memory_*',
  '__wbg___wbindgen_module_*',
  '__wbg___wbindgen_neg_*',
  '__wbg___wbindgen_shr_*',
  '__wbg___wbindgen_string_get_*',
  '__wbg___wbindgen_throw_*',
  '__wbg_call_*',
  '__wbg_crypto_*',
  '__wbg_error_*',
  '__wbg_getRandomValues_*',
  '__wbg_instanceof_Window_*',
  '__wbg_length_*',
  '__wbg_msCrypto_*',
  '__wbg_new_*',
  '__wbg_new_with_length_*',
  '__wbg_node_*',
  '__wbg_process_*',
  '__wbg_prototypesetcall_*',
  '__wbg_randomFillSync_*',
  '__wbg_require_*',
  '__wbg_stack_*',
  '__wbg_startWorkers_*',
  '__wbg_static_accessor_GLOBAL_*',
  '__wbg_static_accessor_GLOBAL_THIS_*',
  '__wbg_static_accessor_SELF_*',
  '__wbg_static_accessor_WINDOW_*',
  '__wbg_subarray_*',
  '__wbg_toString_*',
  '__wbg_versions_*',
  '__wbindgen_cast_*',
  '__wbindgen_init_externref_table',
]);

const optionalWbgImportNames = new Set<string>([
  '__wbg___wbindgen_bit_or_*',
  '__wbg___wbindgen_shl_*',
  '__wbg_getTime_*',
  '__wbg_new_0_*',
  '__wbg_new_no_args_*',
]);

const allowedWbgImportNames = new Set([...expectedWbgImportNames, ...optionalWbgImportNames]);
const expectedWbgImportMinimumCounts = new Map<string, number>([
  ['__wbg_BigInt_*', 2],
  ['__wbg_toString_*', 2],
  ['__wbindgen_cast_*', 5],
]);

const usage = [
  'Usage:',
  '  node scripts/wasm/tfhe/prune-wbg-init.ts <input.js> [output.js] [--root <name>] [--keep-export <name>]',
  '',
  'Defaults:',
  '  --root __wbg_init',
  '',
  'Examples:',
  '  node scripts/wasm/tfhe/prune-wbg-init.ts src/wasm/tfhe/dev.local/tfhe.js src/wasm/tfhe/dev.local/tfhe.init.js',
  '  node scripts/wasm/tfhe/prune-wbg-init.ts src/wasm/tfhe/tfhe.v1.5.3.js src/wasm/tfhe/tfhe-worker.v1.5.3.mjs',
].join('\n');

function parseArgs(argv: string[]): ParsedArgs {
  const positional: string[] = [];
  const roots = new Set<string>(['__wbg_init']);
  const keptExports = new Set<string>();

  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i];

    if (arg === '--root') {
      const root = argv[++i];

      if (!root) {
        throw new Error(`Missing value for ${arg}\n\n${usage}`);
      }

      roots.add(root);
      continue;
    }

    if (arg === '--keep-export') {
      const exportedName = argv[++i];

      if (!exportedName) {
        throw new Error(`Missing value for ${arg}\n\n${usage}`);
      }

      keptExports.add(exportedName);
      continue;
    }

    if (arg === '--help' || arg === '-h') {
      console.log(usage);
      process.exit(0);
    }

    positional.push(arg);
  }

  if (positional.length === 0 || positional.length > 2) {
    throw new Error(usage);
  }

  return {
    input: positional[0],
    output: positional[1],
    roots,
    keptExports,
  };
}

function findRepoRoot(start: string): string | undefined {
  let current = resolve(start);

  while (true) {
    if (existsSync(resolve(current, '.git'))) {
      return current;
    }

    const parent = dirname(current);

    if (parent === current) {
      return undefined;
    }

    current = parent;
  }
}

function resolveInputPath(input: string): string {
  const cwdPath = resolve(process.cwd(), input);

  if (existsSync(cwdPath)) {
    return cwdPath;
  }

  const repoRoot = findRepoRoot(scriptDir);

  if (repoRoot) {
    const repoPath = resolve(repoRoot, input);

    if (existsSync(repoPath)) {
      return repoPath;
    }
  }

  return cwdPath;
}

function isIdentifierBindingName(name: ts.BindingName): name is ts.Identifier {
  return ts.isIdentifier(name);
}

function declaredNamesInStatement(statement: ts.Statement): string[] {
  if ((ts.isFunctionDeclaration(statement) || ts.isClassDeclaration(statement)) && statement.name) {
    return [statement.name.text];
  }

  if (!ts.isVariableStatement(statement)) {
    return [];
  }

  return statement.declarationList.declarations
    .map((declaration) => declaration.name)
    .filter(isIdentifierBindingName)
    .map((name) => name.text);
}

function importedNamesInStatement(statement: ts.Statement): string[] {
  if (!ts.isImportDeclaration(statement) || !statement.importClause) {
    return [];
  }

  const names: string[] = [];
  const { name, namedBindings } = statement.importClause;

  if (name) {
    names.push(name.text);
  }

  if (namedBindings && ts.isNamedImports(namedBindings)) {
    for (const element of namedBindings.elements) {
      names.push(element.name.text);
    }
  }

  return names;
}

function localNameOfExportSpecifier(specifier: ts.ExportSpecifier): string {
  return (specifier.propertyName || specifier.name).text;
}

function removeExportModifier(modifiers: readonly ts.ModifierLike[] | undefined): ts.ModifierLike[] | undefined {
  if (!modifiers) {
    return undefined;
  }

  const next = modifiers.filter((modifier) => modifier.kind !== ts.SyntaxKind.ExportKeyword);
  return next.length > 0 ? next : undefined;
}

function propertyNameText(name: ts.PropertyName): string | undefined {
  if (ts.isIdentifier(name) || ts.isStringLiteral(name) || ts.isNumericLiteral(name)) {
    return name.text;
  }

  return undefined;
}

function isStartWorkersShimName(name: string): boolean {
  return /^__wbg_startWorkers_[0-9a-f]+$/i.test(name);
}

function normalizeWbgImportName(name: string): string {
  return name.replace(/_[0-9a-f]{16}$/i, '_*');
}

function isWbgImportName(name: string): boolean {
  return name.startsWith('__wbg_') || name.startsWith('__wbindgen_');
}

function findWbgGetImportsFunctions(sourceFile: ts.SourceFile): ts.FunctionDeclaration[] {
  const functions: ts.FunctionDeclaration[] = [];

  function visit(node: ts.Node) {
    if (ts.isFunctionDeclaration(node) && node.name?.text === '__wbg_get_imports') {
      functions.push(node);
    }

    ts.forEachChild(node, visit);
  }

  visit(sourceFile);
  return functions;
}

function collectWbgImportShimNames(wbgGetImportsFunction: ts.FunctionDeclaration): string[] {
  const names: string[] = [];

  function visit(node: ts.Node) {
    if (ts.isPropertyAssignment(node)) {
      const name = propertyNameText(node.name);

      if (name && isWbgImportName(name)) {
        names.push(name);
      }
    }

    ts.forEachChild(node, visit);
  }

  visit(wbgGetImportsFunction);
  return names;
}

function formatList(items: Iterable<string>): string {
  return [...items].sort().join(', ');
}

function countByName(names: Iterable<string>): Map<string, number> {
  const counts = new Map<string, number>();

  for (const name of names) {
    counts.set(name, (counts.get(name) || 0) + 1);
  }

  return counts;
}

function validateWbgGetImports(sourceFile: ts.SourceFile): void {
  const wbgGetImportsFunctions = findWbgGetImportsFunctions(sourceFile);

  if (wbgGetImportsFunctions.length !== 1) {
    throw new Error(`Expected exactly one __wbg_get_imports function, found ${wbgGetImportsFunctions.length}.`);
  }

  const actualNames = collectWbgImportShimNames(wbgGetImportsFunctions[0]);
  const actualNormalizedNameList = actualNames.map(normalizeWbgImportName);
  const actualNormalizedNames = new Set(actualNormalizedNameList);
  const actualNormalizedNameCounts = countByName(actualNormalizedNameList);
  const missingNames = [...expectedWbgImportNames].filter((name) => !actualNormalizedNames.has(name));
  const undercountedNames = [...expectedWbgImportMinimumCounts]
    .filter(([name, minimumCount]) => (actualNormalizedNameCounts.get(name) || 0) < minimumCount)
    .map(
      ([name, minimumCount]) =>
        `${name} expected >= ${minimumCount}, found ${actualNormalizedNameCounts.get(name) || 0}`,
    );
  const unknownNames = [...actualNormalizedNames].filter((name) => !allowedWbgImportNames.has(name));

  if (actualNames.length === 0) {
    throw new Error('Expected __wbg_get_imports to contain wasm-bindgen import shims, found none.');
  }

  if (missingNames.length > 0) {
    throw new Error(`Missing expected __wbg_get_imports import shim(s): ${formatList(missingNames)}.`);
  }

  if (undercountedNames.length > 0) {
    throw new Error(
      `Missing expected __wbg_get_imports import shim overload(s): ${undercountedNames.sort().join(', ')}.`,
    );
  }

  if (unknownNames.length > 0) {
    throw new Error(`Unknown __wbg_get_imports import shim(s): ${formatList(unknownNames)}.`);
  }
}

function findStartWorkersShims(sourceFile: ts.SourceFile): ts.PropertyAssignment[] {
  const shims: ts.PropertyAssignment[] = [];

  function visit(node: ts.Node) {
    if (ts.isPropertyAssignment(node)) {
      const name = propertyNameText(node.name);

      if (name && isStartWorkersShimName(name)) {
        shims.push(node);
      }
    }

    ts.forEachChild(node, visit);
  }

  visit(sourceFile);
  return shims;
}

function createCommentOnlyStatement(comment: string): ts.Statement {
  const statement = ts.factory.createNotEmittedStatement(ts.factory.createIdentifier('__comment'));
  ts.addSyntheticLeadingComment(statement, ts.SyntaxKind.SingleLineCommentTrivia, ` ${comment}`, true);
  return statement;
}

function createCommentOnlyStatements(comments: string[]): ts.Statement[] {
  return comments.map(createCommentOnlyStatement);
}

function createUnsupportedStartWorkersBody(): ts.Block {
  const handleErrorStatement = ts.factory.createExpressionStatement(
    ts.factory.createCallExpression(ts.factory.createIdentifier('handleError'), undefined, [
      ts.factory.createFunctionExpression(
        undefined,
        undefined,
        undefined,
        undefined,
        [],
        undefined,
        ts.factory.createBlock(
          [
            ts.factory.createThrowStatement(
              ts.factory.createNewExpression(ts.factory.createIdentifier('Error'), undefined, [
                ts.factory.createStringLiteral('startWorkers not supported from a worker thread', true),
              ]),
            ),
          ],
          true,
        ),
      ),
    ]),
  );

  return ts.factory.createBlock(
    [
      handleErrorStatement,
      createCommentOnlyStatement('const ret = startWorkers(arg0, arg1, wbg_rayon_PoolBuilder.__wrap(arg2));'),
      createCommentOnlyStatement('return ret;'),
    ],
    true,
  );
}

function rewriteStartWorkersShim(sourceFile: ts.SourceFile): ts.SourceFile {
  const shims = findStartWorkersShims(sourceFile);

  if (shims.length !== 1) {
    throw new Error(`Expected exactly one __wbg_startWorkers_* import shim, found ${shims.length}.`);
  }

  const [shim] = shims;

  if (!ts.isFunctionExpression(shim.initializer)) {
    const name = propertyNameText(shim.name);
    throw new Error(`Expected ${name} to be a function expression.`);
  }

  const shimInitializer = shim.initializer;

  function visitor(node: ts.Node): ts.VisitResult<ts.Node> {
    if (node === shim) {
      return ts.factory.updatePropertyAssignment(
        shim,
        shim.name,
        ts.factory.updateFunctionExpression(
          shimInitializer,
          shimInitializer.modifiers,
          shimInitializer.asteriskToken,
          shimInitializer.name,
          shimInitializer.typeParameters,
          shimInitializer.parameters,
          shimInitializer.type,
          createUnsupportedStartWorkersBody(),
        ),
      );
    }

    return ts.visitEachChild(node, visitor, transformContext!);
  }

  let transformContext: ts.TransformationContext | undefined;
  const result = ts.transform(sourceFile, [
    (context) => {
      transformContext = context;
      return (node) => ts.visitNode(node, visitor) as ts.SourceFile;
    },
  ]);

  return result.transformed[0];
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

function createCommentedDefaultModulePathBlock(): ts.Statement[] {
  return createCommentOnlyStatements([
    '  if (module_or_path === undefined) {',
    "    module_or_path = new URL('tfhe_bg.wasm', import.meta.url);",
    '  }',
  ]);
}

function createCommentedFetchModulePathBlock(): ts.Statement[] {
  return createCommentOnlyStatements([
    '  if (',
    "    typeof module_or_path === 'string' ||",
    "    (typeof Request === 'function' && module_or_path instanceof Request) ||",
    "    (typeof URL === 'function' && module_or_path instanceof URL)",
    '  ) {',
    '    module_or_path = fetch(module_or_path);',
    '  }',
  ]);
}

function rewriteWbgInitFunction(sourceFile: ts.SourceFile): ts.SourceFile {
  let initFunctions = 0;
  let defaultModulePathBlocks = 0;
  let fetchModulePathBlocks = 0;

  function visitor(node: ts.Node): ts.VisitResult<ts.Node> {
    if (ts.isFunctionDeclaration(node) && node.name?.text === '__wbg_init') {
      initFunctions++;

      if (!node.body) {
        throw new Error('Expected __wbg_init to have a function body.');
      }

      const statements: ts.Statement[] = [];

      for (const statement of node.body.statements) {
        if (isModulePathUndefinedCheck(statement)) {
          defaultModulePathBlocks++;
          statements.push(...createCommentedDefaultModulePathBlock());
          continue;
        }

        if (isModulePathFetchCheck(statement)) {
          fetchModulePathBlocks++;
          statements.push(...createCommentedFetchModulePathBlock());
          continue;
        }

        statements.push(statement);
      }

      return ts.factory.updateFunctionDeclaration(
        node,
        node.modifiers,
        node.asteriskToken,
        node.name,
        node.typeParameters,
        node.parameters,
        node.type,
        ts.factory.updateBlock(node.body, statements),
      );
    }

    return ts.visitEachChild(node, visitor, transformContext!);
  }

  let transformContext: ts.TransformationContext | undefined;
  const result = ts.transform(sourceFile, [
    (context) => {
      transformContext = context;
      return (node) => ts.visitNode(node, visitor) as ts.SourceFile;
    },
  ]);

  if (initFunctions !== 1) {
    throw new Error(`Expected exactly one __wbg_init function, found ${initFunctions}.`);
  }

  if (defaultModulePathBlocks > 1) {
    throw new Error(
      `Expected at most one default module_or_path block in __wbg_init, found ${defaultModulePathBlocks}.`,
    );
  }

  if (fetchModulePathBlocks > 1) {
    throw new Error(`Expected at most one fetch module_or_path block in __wbg_init, found ${fetchModulePathBlocks}.`);
  }

  if (defaultModulePathBlocks !== fetchModulePathBlocks) {
    throw new Error(
      `Expected __wbg_init module_or_path rewrites to match, found ${defaultModulePathBlocks} default block(s) and ${fetchModulePathBlocks} fetch block(s).`,
    );
  }

  return result.transformed[0];
}

function shouldIgnoreIdentifier(node: ts.Identifier): boolean {
  const parent = node.parent;

  if (!parent) {
    return false;
  }

  if (ts.isPropertyAccessExpression(parent) && parent.name === node) {
    return true;
  }

  if (ts.isPropertyAssignment(parent) && parent.name === node) {
    return true;
  }

  if (ts.isMethodDeclaration(parent) && parent.name === node) {
    return true;
  }

  if (ts.isPropertyDeclaration(parent) && parent.name === node) {
    return true;
  }

  if (ts.isFunctionDeclaration(parent) && parent.name === node) {
    return true;
  }

  if (ts.isClassDeclaration(parent) && parent.name === node) {
    return true;
  }

  if (ts.isVariableDeclaration(parent) && parent.name === node) {
    return true;
  }

  if (ts.isParameter(parent) && parent.name === node) {
    return true;
  }

  if (ts.isImportSpecifier(parent) && parent.name === node) {
    return true;
  }

  if (ts.isImportClause(parent) && parent.name === node) {
    return true;
  }

  if (ts.isExportSpecifier(parent) && parent.name === node) {
    return true;
  }

  if (ts.isShorthandPropertyAssignment(parent) && parent.name === node) {
    return false;
  }

  if (ts.isBindingElement(parent) && parent.name === node) {
    return true;
  }

  return false;
}

function collectReferencedTopLevelNames(node: ts.Node, topLevelNames: Set<string>): Set<string> {
  const references = new Set<string>();

  function visit(current: ts.Node) {
    if (ts.isIdentifier(current) && !shouldIgnoreIdentifier(current) && topLevelNames.has(current.text)) {
      references.add(current.text);
    }

    ts.forEachChild(current, visit);
  }

  visit(node);
  return references;
}

function buildIndexes(sourceFile: ts.SourceFile): DeclarationIndexes {
  const declarations = new Map<string, ts.Statement>();
  const imports = new Map<string, ts.ImportDeclaration>();

  for (const statement of sourceFile.statements) {
    for (const name of declaredNamesInStatement(statement)) {
      declarations.set(name, statement);
    }

    if (ts.isImportDeclaration(statement)) {
      for (const name of importedNamesInStatement(statement)) {
        imports.set(name, statement);
      }
    }
  }

  const topLevelNames = new Set([...declarations.keys(), ...imports.keys()]);
  return { declarations, imports, topLevelNames };
}

function markReachable(roots: Set<string>, indexes: DeclarationIndexes): Set<string> {
  const needed = new Set<string>();
  const queue = [...roots];

  while (queue.length > 0) {
    const name = queue.shift()!;

    if (needed.has(name)) {
      continue;
    }

    needed.add(name);

    const declaration = indexes.declarations.get(name) || indexes.imports.get(name);

    if (!declaration) {
      continue;
    }

    for (const reference of collectReferencedTopLevelNames(declaration, indexes.topLevelNames)) {
      if (!needed.has(reference)) {
        queue.push(reference);
      }
    }
  }

  return needed;
}

function statementReferencesNeededName(
  statement: ts.Statement,
  needed: Set<string>,
  topLevelNames: Set<string>,
): boolean {
  for (const reference of collectReferencedTopLevelNames(statement, topLevelNames)) {
    if (needed.has(reference)) {
      return true;
    }
  }

  return false;
}

function updateImportDeclaration(
  statement: ts.ImportDeclaration,
  needed: Set<string>,
): ts.ImportDeclaration | undefined {
  const importClause = statement.importClause;

  if (!importClause) {
    return statement;
  }

  const defaultName = importClause.name && needed.has(importClause.name.text) ? importClause.name : undefined;
  let namedBindings: ts.NamedImportBindings | undefined;

  if (importClause.namedBindings) {
    if (ts.isNamedImports(importClause.namedBindings)) {
      const elements = importClause.namedBindings.elements.filter((element) => needed.has(element.name.text));

      if (elements.length > 0) {
        namedBindings = ts.factory.updateNamedImports(importClause.namedBindings, elements);
      }
    } else if (needed.has(importClause.namedBindings.name.text)) {
      namedBindings = importClause.namedBindings;
    }
  }

  if (!defaultName && !namedBindings) {
    return undefined;
  }

  return ts.factory.updateImportDeclaration(
    statement,
    statement.modifiers,
    ts.factory.updateImportClause(importClause, importClause.phaseModifier, defaultName, namedBindings),
    statement.moduleSpecifier,
    statement.attributes,
  );
}

function updateVariableStatement(
  statement: ts.VariableStatement,
  needed: Set<string>,
): ts.VariableStatement | undefined {
  const declarations = statement.declarationList.declarations.filter((declaration) => {
    return isIdentifierBindingName(declaration.name) && needed.has(declaration.name.text);
  });

  if (declarations.length === 0) {
    return undefined;
  }

  return ts.factory.updateVariableStatement(
    statement,
    removeExportModifier(statement.modifiers),
    ts.factory.updateVariableDeclarationList(statement.declarationList, declarations),
  );
}

function updateFunctionDeclaration(
  statement: ts.FunctionDeclaration,
  keptExports: Set<string>,
): ts.FunctionDeclaration {
  const name = statement.name?.text;
  const modifiers = name && keptExports.has(name) ? statement.modifiers : removeExportModifier(statement.modifiers);
  return ts.factory.updateFunctionDeclaration(
    statement,
    modifiers,
    statement.asteriskToken,
    statement.name,
    statement.typeParameters,
    statement.parameters,
    statement.type,
    statement.body,
  );
}

function updateClassDeclaration(statement: ts.ClassDeclaration, keptExports: Set<string>): ts.ClassDeclaration {
  const name = statement.name?.text;
  const modifiers = name && keptExports.has(name) ? statement.modifiers : removeExportModifier(statement.modifiers);
  return ts.factory.updateClassDeclaration(
    statement,
    modifiers,
    statement.name,
    statement.typeParameters,
    statement.heritageClauses,
    statement.members,
  );
}

function updateExportDeclaration(
  statement: ts.ExportDeclaration,
  needed: Set<string>,
  keptExports: Set<string>,
): ts.ExportDeclaration | undefined {
  if (!statement.exportClause || !ts.isNamedExports(statement.exportClause)) {
    return undefined;
  }

  const elements = statement.exportClause.elements.filter((element) => {
    const localName = localNameOfExportSpecifier(element);
    return needed.has(localName) && keptExports.has(localName);
  });

  if (elements.length === 0) {
    return undefined;
  }

  return ts.factory.updateExportDeclaration(
    statement,
    statement.modifiers,
    statement.isTypeOnly,
    ts.factory.updateNamedExports(statement.exportClause, elements),
    statement.moduleSpecifier,
    statement.attributes,
  );
}

function shouldKeepExportAssignment(
  statement: ts.ExportAssignment,
  needed: Set<string>,
  keptExports: Set<string>,
): boolean {
  return (
    ts.isIdentifier(statement.expression) &&
    needed.has(statement.expression.text) &&
    keptExports.has(statement.expression.text)
  );
}

function pruneSourceFile(
  sourceFile: ts.SourceFile,
  needed: Set<string>,
  keptExports: Set<string>,
  indexes: DeclarationIndexes,
): ts.SourceFile {
  const statements: ts.Statement[] = [];

  for (const statement of sourceFile.statements) {
    if (ts.isImportDeclaration(statement)) {
      const updated = updateImportDeclaration(statement, needed);

      if (updated) {
        statements.push(updated);
      }

      continue;
    }

    if (ts.isVariableStatement(statement)) {
      const updated = updateVariableStatement(statement, needed);

      if (updated) {
        statements.push(updated);
      }

      continue;
    }

    if (ts.isFunctionDeclaration(statement)) {
      if (statement.name && needed.has(statement.name.text)) {
        statements.push(updateFunctionDeclaration(statement, keptExports));
      }

      continue;
    }

    if (ts.isClassDeclaration(statement)) {
      if (statement.name && needed.has(statement.name.text)) {
        statements.push(updateClassDeclaration(statement, keptExports));
      }

      continue;
    }

    if (ts.isExportDeclaration(statement)) {
      const updated = updateExportDeclaration(statement, needed, keptExports);

      if (updated) {
        statements.push(updated);
      }

      continue;
    }

    if (ts.isExportAssignment(statement)) {
      if (shouldKeepExportAssignment(statement, needed, keptExports)) {
        statements.push(statement);
      }

      continue;
    }

    if (statementReferencesNeededName(statement, needed, indexes.topLevelNames)) {
      statements.push(statement);
    }
  }

  return ts.factory.updateSourceFile(sourceFile, statements);
}

function asciiSection(lines: string[]): string {
  return [
    '////////////////////////////////////////////////////////////////////////////////',
    ...lines.map((line) => `//${line.length > 0 ? ` ${line}` : ''}`),
    '////////////////////////////////////////////////////////////////////////////////',
  ].join('\n');
}

function statementNames(statement: ts.Statement): string[] {
  return [...declaredNamesInStatement(statement), ...importedNamesInStatement(statement)];
}

function sectionForStatement(statement: ts.Statement): string | undefined {
  const names = statementNames(statement);
  const primaryName = names[0];

  if (!primaryName) {
    return undefined;
  }

  if (primaryName === '__wbg_get_imports') {
    return asciiSection(['', 'Imports:', '__wbg_get_imports', '']);
  }

  if (primaryName === '__wbg_finalize_init') {
    return asciiSection(['Init:', '__wbg_finalize_init']);
  }

  if (primaryName === '__wbg_load') {
    return asciiSection(['Init:', '__wbg_load']);
  }

  if (primaryName === '__wbg_init') {
    return asciiSection(['Init:', '__wbg_init']);
  }

  if (primaryName === 'WASM_VECTOR_LEN') {
    return asciiSection([
      'WASM_VECTOR_LEN is a module-level variable that stores the byte length of',
      'the data just written into WASM memory. It acts as an out-parameter.',
    ]);
  }

  if (names.includes('wasm')) {
    return asciiSection(['WASM module state']);
  }

  return asciiSection([primaryName]);
}

function formatWorkerBody(sourceFile: ts.SourceFile, printer: ts.Printer): string {
  const chunks: string[] = [];

  for (const statement of sourceFile.statements) {
    const section = sectionForStatement(statement);

    if (section) {
      chunks.push(section);
    }

    chunks.push(printer.printNode(ts.EmitHint.Unspecified, statement, sourceFile));
  }

  return chunks.join('\n\n');
}

function readWorkerTemplate(): string {
  const template = readFileSync(workerTemplatePath, 'utf8');

  if (!template.includes(workerBodyPlaceholder)) {
    throw new Error(`Missing ${workerBodyPlaceholder} placeholder in ${workerTemplatePath}.`);
  }

  return template;
}

function formatWorkerModule(sourceFile: ts.SourceFile, printer: ts.Printer): string {
  const body = formatWorkerBody(sourceFile, printer);
  const template = readWorkerTemplate();

  return `${template.replace(workerBodyPlaceholder, body)}\n`;
}

const { input, output, roots, keptExports } = parseArgs(process.argv.slice(2));
const inputPath = resolveInputPath(input);
const sourceText = readFileSync(inputPath, 'utf8');
const parsedSourceFile = ts.createSourceFile(inputPath, sourceText, ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
validateWbgGetImports(parsedSourceFile);
const sourceFile = rewriteWbgInitFunction(rewriteStartWorkersShim(parsedSourceFile));
const indexes = buildIndexes(sourceFile);
const needed = markReachable(roots, indexes);
const pruned = pruneSourceFile(sourceFile, needed, keptExports, indexes);
const printer = ts.createPrinter({ newLine: ts.NewLineKind.LineFeed });
const result = formatWorkerModule(pruned, printer);

if (output) {
  writeFileSync(resolve(process.cwd(), output), result);
} else {
  process.stdout.write(result);
}

const removedDeclarations = [...indexes.declarations.keys()].filter((name) => !needed.has(name));
console.error(`Kept ${needed.size} top-level names; removed ${removedDeclarations.length} top-level declarations.`);
