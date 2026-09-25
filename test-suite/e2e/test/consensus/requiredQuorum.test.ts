import { expect } from 'chai';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import ts from 'typescript';
import { requireQuorumConfiguration } from './helpers';

// Execute actual hooks/tails without importing Hardhat or contacting a stack.
function callbackStatements(file: string, hook: 'before' | 'it'): ts.NodeArray<ts.Statement> {
  const source = ts.createSourceFile(file, readFileSync(join(__dirname, file), 'utf8'), ts.ScriptTarget.ES2022, true);
  let body: ts.Block | undefined;
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && node.expression.getText(source) === hook) {
      const callback = node.arguments.find((argument) => ts.isFunctionExpression(argument));
      if (callback && ts.isFunctionExpression(callback)) body = callback.body;
    }
    ts.forEachChild(node, visit);
  };
  visit(source);
  if (!body) throw new Error(`${file} has no ${hook} hook`);
  return body.statements;
}

function execute(statements: readonly ts.Statement[], bindings: Record<string, unknown>): Promise<void> {
  const code = ts.transpileModule(`async function run() { ${statements.map((statement) => statement.getText()).join('\n')} }`, {
    compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.CommonJS },
  }).outputText;
  return new Function(...Object.keys(bindings), `${code}; return run.call({skip(){throw Error('unexpected skip')}});`)(...Object.values(bindings));
}

const address = `0x${'ab'.repeat(20)}`;
const configuration = { GATEWAY_RPC_URL: 'http://gateway:8546', GATEWAY_CONFIG_ADDRESS: address, CIPHERTEXT_COMMITS_ADDRESS: address };
const suites = ['reorgConsensus.ts'];

describe('Required quorum is an executed suite gate', () => {
  for (const file of suites) {
    for (const missing of Object.keys(configuration)) {
      it(`${file} rejects missing ${missing} before fixture access`, async () => {
        let accesses = 0;
        const forbidden = () => { accesses++; throw new Error('fixture access before configuration validation'); };
        const fields = { ...configuration, [missing]: '' };
        let failure: unknown;
        try {
          await execute(callbackStatements(file, 'before'), {
            ...fields, ENABLE: true, ENABLE_REORG_CONSENSUS: true, VICTIM: 1, COPROCESSOR_COUNT: 3,
            process: { env: { ...fields } }, requireQuorumConfiguration,
            getCoprocessorDbUrls: forbidden, require: forbidden,
          });
        } catch (error) { failure = error; }
        expect(failure).to.be.instanceOf(Error);
        expect((failure as Error).message).to.contain('requires gateway');
        expect(accesses).to.eq(0);
      });
    }
  }

  for (const file of suites) {
    it(`${file} cannot publish completion while required quorum is pending or rejected`, async () => {
      const statements = callbackStatements(file, 'it');
      const start = statements.findIndex((statement) => statement.getText().includes('readGatewayMembership('));
      expect(start).to.be.at.least(0);
      const receipts: unknown[] = [];
      const messages: unknown[] = [];
      let calls = 0;
      let reject!: (reason: Error) => void;
      const pending = new Promise<never>((_, fail) => { reject = fail; });
      const run = execute(statements.slice(start), {
        ...configuration, BOUNDARY: 'before-commit', distinctHandles: ['handle'], firstHandle: 'handle', MARKER: 'COMPLETE',
        process: { env: {} }, readGatewayMembership: async () => ({ txSenders: ['sender'], threshold: 2 }),
        assertQuorumOutcome: async (input: { mode: string }) => { expect(input.mode).to.eq('required'); calls++; return pending; },
        emitAssertions: (...args: unknown[]) => receipts.push(args), console: { info: (...args: unknown[]) => messages.push(args) },
      });
      await new Promise((resolve) => setImmediate(resolve));
      expect(calls).to.eq(1);
      expect(receipts).to.have.length(0);
      expect(messages).to.have.length(0);
      reject(new Error('quorum never formed'));
      let failure: unknown;
      try { await run; } catch (error) { failure = error; }
      expect((failure as Error).message).to.eq('quorum never formed');
      expect(receipts).to.have.length(0);
      expect(messages).to.have.length(0);
    });
  }
});
