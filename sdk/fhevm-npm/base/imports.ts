import { readFileSync } from 'node:fs';
import ts from 'typescript';

export function collectPackageImports(files: readonly string[]): ReadonlySet<string> {
  const imports = new Set<string>();
  for (const file of files) {
    for (const specifier of collectModuleSpecifiers(readFileSync(file, 'utf8'), file)) {
      const packageName = packageNameFromSpecifier(specifier);
      if (packageName !== undefined) imports.add(packageName);
    }
  }
  return imports;
}

export function collectModuleSpecifiers(source: string, fileName = 'source.ts'): ReadonlySet<string> {
  const sourceFile = ts.createSourceFile(fileName, source, ts.ScriptTarget.Latest, true, scriptKind(fileName));
  const specifiers = new Set<string>();

  function addLiteral(node: ts.Expression | ts.TypeNode | undefined): void {
    if (node !== undefined && (ts.isStringLiteral(node) || ts.isNoSubstitutionTemplateLiteral(node))) {
      specifiers.add(node.text);
    }
  }

  function visit(node: ts.Node): void {
    if (ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) {
      addLiteral(node.moduleSpecifier);
    } else if (ts.isImportEqualsDeclaration(node) && ts.isExternalModuleReference(node.moduleReference)) {
      addLiteral(node.moduleReference.expression);
    } else if (ts.isImportTypeNode(node) && ts.isLiteralTypeNode(node.argument)) {
      addLiteral(node.argument.literal);
    } else if (ts.isCallExpression(node) && isModuleLoadingCall(node.expression)) {
      addLiteral(node.arguments[0]);
    }
    ts.forEachChild(node, visit);
  }

  visit(sourceFile);
  return specifiers;
}

export function packageNameFromSpecifier(specifier: string): string | undefined {
  if (
    specifier.startsWith('.') ||
    specifier.startsWith('/') ||
    specifier.startsWith('#') ||
    /^[A-Za-z][A-Za-z0-9+.-]*:/.test(specifier)
  ) {
    return undefined;
  }

  const parts = specifier.split('/');
  if (specifier.startsWith('@')) {
    return parts.length >= 2 && parts[0] && parts[1] ? `${parts[0]}/${parts[1]}` : undefined;
  }
  return parts[0] || undefined;
}

function isModuleLoadingCall(expression: ts.LeftHandSideExpression): boolean {
  if (expression.kind === ts.SyntaxKind.ImportKeyword) return true;
  if (ts.isIdentifier(expression) && expression.text === 'require') return true;
  return (
    ts.isPropertyAccessExpression(expression) &&
    ts.isIdentifier(expression.expression) &&
    expression.expression.text === 'require' &&
    expression.name.text === 'resolve'
  );
}

function scriptKind(fileName: string): ts.ScriptKind {
  if (/\.tsx$/i.test(fileName)) return ts.ScriptKind.TSX;
  if (/\.jsx$/i.test(fileName)) return ts.ScriptKind.JSX;
  if (/\.(?:m|c)?js$/i.test(fileName)) return ts.ScriptKind.JS;
  return ts.ScriptKind.TS;
}
