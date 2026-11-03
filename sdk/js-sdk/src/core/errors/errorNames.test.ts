import { readdirSync } from 'node:fs';
import { describe, it, expect } from 'vitest';
import { ErrorBase } from '../base/errors/ErrorBase.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/errors/errorNames.test.ts
//
// Every SDK error must report its own class name in `err.name`, since apps
// match on `err.name` (see docs/error-handling.md). The error classes are
// discovered from the error modules rather than listed by hand, so a new
// error class is covered as soon as it is added.
////////////////////////////////////////////////////////////////////////////////

type ErrorClass = (abstract new (...args: never[]) => ErrorBase) & { readonly name: string };
type ConcreteErrorClass = new (params: Record<string, unknown>, options: Record<string, unknown>) => ErrorBase;

const ERROR_DIRS = ['./', '../base/errors/'] as const;

// Abstract bases cannot be thrown as-is; they are tested through a subclass.
const ABSTRACT_ERROR_CLASSES = new Set([
  'ContractErrorBase',
  'Erc1271VerificationError',
  'FhevmErrorBase',
  'RelayerErrorBase',
  'RelayerFetchErrorBase',
  'RelayerRequestErrorBase',
  'RelayerResponseErrorBase',
]);

// One params object accepted by every error constructor (each ignores the
// fields it does not use).
const PARAMS: Record<string, unknown> = {
  message: 'test message',
  // relayer
  url: 'https://relayer.test',
  operation: 'INPUT_PROOF',
  fetchMethod: 'POST',
  retryCount: 1,
  elapsed: 1,
  status: 500,
  timeoutMs: 1,
  bodyJson: '{}',
  cause: new Error('test cause'),
  relayerApiError: { label: 'internal_server_error', message: 'test api message' },
  state: {
    aborted: false,
    canceled: false,
    failed: false,
    fetching: false,
    running: false,
    succeeded: false,
    terminated: false,
    timeout: false,
  },
  // contracts and signers
  contractAddress: '0x0000000000000000000000000000000000000001',
  contractName: 'ACL',
  handles: ['0x' + '00'.repeat(32)],
  userAddress: '0x0000000000000000000000000000000000000002',
  unknownAddress: '0x0000000000000000000000000000000000000003',
  duplicateAddress: '0x0000000000000000000000000000000000000004',
  magicValue: '0xffffffff',
  detail: 'test detail',
  type: 'kms',
  numberOfHandles: 257,
  // base errors
  subject: 'test subject',
  property: 'test property',
  expectedType: 'string',
  expected: '0x00',
  actual: '0x01',
};

// Keyed by the class itself, so two classes sharing a name are both kept.
async function discoverErrorClasses(): Promise<Map<ErrorClass, string>> {
  const classes = new Map<ErrorClass, string>();
  for (const dir of ERROR_DIRS) {
    const files = readdirSync(new URL(dir, import.meta.url)).filter(
      (f) => f.endsWith('.ts') && !f.endsWith('.test.ts'),
    );
    for (const file of files) {
      const mod = (await import(`${dir}${file}`)) as Record<string, unknown>;
      for (const value of Object.values(mod)) {
        const errorClass = value as unknown as ErrorClass;
        if (typeof value === 'function' && value.prototype instanceof ErrorBase && !classes.has(errorClass)) {
          classes.set(errorClass, `${dir}${file}`);
        }
      }
    }
  }
  return classes;
}

const errorClassFiles = await discoverErrorClasses();
const errorClasses = [...errorClassFiles.keys()].sort((a, b) => a.name.localeCompare(b.name));
const concreteErrorClasses = errorClasses.filter((c) => !ABSTRACT_ERROR_CLASSES.has(c.name));
const abstractErrorClasses = errorClasses.filter((c) => ABSTRACT_ERROR_CLASSES.has(c.name));

describe('error names', () => {
  //////////////////////////////////////////////////////////////////////////////

  it('discovers the error classes, including every listed abstract base', () => {
    const names = errorClasses.map((c) => c.name);
    expect(names).toEqual(expect.arrayContaining(['AclUserDecryptionError', 'FhevmConfigError', 'RelayerAbortError']));
    expect(abstractErrorClasses.map((c) => c.name).sort()).toEqual([...ABSTRACT_ERROR_CLASSES].sort());
  });

  it('gives every error class a distinct name', () => {
    const filesByName = new Map<string, string[]>();
    for (const [errorClass, file] of errorClassFiles) {
      filesByName.set(errorClass.name, [...(filesByName.get(errorClass.name) ?? []), file]);
    }
    const clashes = [...filesByName]
      .filter(([, files]) => files.length > 1)
      .map(([name, files]) => `${name}: ${files.join(', ')}`);
    expect(clashes).toEqual([]);
  });

  //////////////////////////////////////////////////////////////////////////////

  describe.each(concreteErrorClasses.map((c) => [c.name, c] as const))('%s', (name, ErrorClass) => {
    const err = new (ErrorClass as unknown as ConcreteErrorClass)(PARAMS, {});

    it('reports its own class name', () => {
      expect(err.name).toBe(name);
    });

    it('is an instance of its class, its parent class, ErrorBase and Error', () => {
      expect(err).toBeInstanceOf(ErrorClass);
      expect(err).toBeInstanceOf(Object.getPrototypeOf(ErrorClass) as ErrorClass);
      expect(err).toBeInstanceOf(ErrorBase);
      expect(err).toBeInstanceOf(Error);
    });

    it('prefixes toString() with its class name', () => {
      expect(err.toString().startsWith(`${name}: `)).toBe(true);
    });
  });

  //////////////////////////////////////////////////////////////////////////////

  describe.each(abstractErrorClasses.map((c) => [c.name, c] as const))('subclass of %s', (_name, Base) => {
    class CustomError extends (Base as unknown as ConcreteErrorClass) {
      constructor() {
        super({ ...PARAMS, name: 'CustomError' }, {});
      }
    }

    it('reports the name the subclass passes', () => {
      const err = new CustomError();
      expect(err.name).toBe('CustomError');
      expect(err).toBeInstanceOf(CustomError);
      expect(err).toBeInstanceOf(Base);
    });
  });
});
