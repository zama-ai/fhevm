'use client';

import { useEffect, useState } from 'react';

const initialDiagnostics = {
  error: undefined,
  initialized: false,
  logs: [],
  packageSpecifier: undefined,
  status: 'pending',
};

function errorMessage(err) {
  return err instanceof Error ? err.message : String(err);
}

export default function Page() {
  const [diagnostics, setDiagnostics] = useState(initialDiagnostics);

  useEffect(() => {
    let cancelled = false;

    async function run() {
      const t0 = performance.now();
      const logs = [];
      const log = (message) => {
        const elapsed = (performance.now() - t0).toFixed(0);
        logs.push(`[${elapsed}ms] ${message}`);
      };

      try {
        const packageSpecifier = '@fhevm/sdk/ethers';
        log(`Importing ${packageSpecifier}...`);
        const { hasFhevmRuntimeConfig, initFhevmEncryptRuntime, setFhevmRuntimeConfig } = await import(
          packageSpecifier
        );
        log(`[PASS] Imported ${packageSpecifier}`);

        if (!hasFhevmRuntimeConfig()) {
          log('Setting runtime config without locateFile...');
          setFhevmRuntimeConfig({
            singleThread: true,
            logger: {
              debug: (message) => log(`  [debug] ${message}`),
              error: (message, cause) => {
                log(`  [error] ${message}`);
                if (cause !== undefined) {
                  log(`  [error] ${errorMessage(cause)}`);
                }
              },
            },
          });
          log('[PASS] Runtime config set');
        } else {
          log('[PASS] Runtime config already set');
        }

        log('Initializing encrypt runtime...');
        await initFhevmEncryptRuntime();
        log('[PASS] Encrypt runtime initialized');

        if (!cancelled) {
          setDiagnostics({
            error: undefined,
            initialized: true,
            logs,
            packageSpecifier,
            status: 'pass',
          });
        }
      } catch (err) {
        log(`[FAIL] ${errorMessage(err)}`);
        if (!cancelled) {
          setDiagnostics({
            error: errorMessage(err),
            initialized: false,
            logs,
            packageSpecifier: '@fhevm/sdk/ethers',
            status: 'fail',
          });
        }
      }
    }

    run();

    return () => {
      cancelled = true;
    };
  }, []);

  return (
    <main>
      <h1>Packed SDK Next.js smoke test</h1>
      <p id="result" data-status={diagnostics.status}>
        {diagnostics.status}
      </p>
      <pre id="log">{JSON.stringify(diagnostics, null, 2)}</pre>
    </main>
  );
}
