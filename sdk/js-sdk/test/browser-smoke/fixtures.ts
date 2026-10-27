import {
  test as base,
  type Browser,
  type BrowserContext,
  type BrowserContextOptions,
  type PlaywrightTestArgs,
  type PlaywrightTestOptions,
  type PlaywrightWorkerArgs,
  type PlaywrightWorkerOptions,
  type TestInfo,
  type TestType,
} from '@playwright/test';

/**
 * `_combinedContextOptions` is the internal fixture Playwright's own
 * `context` fixture passes to `newContext()`: every context option from the
 * config (`baseURL`, `ignoreHTTPSErrors`, `viewport`, `contextOptions`, ...)
 * merged into one object. It isn't in the public types, so declare it here.
 */
const baseTest = base as unknown as TestType<
  PlaywrightTestArgs & PlaywrightTestOptions & { _combinedContextOptions: BrowserContextOptions },
  PlaywrightWorkerArgs & PlaywrightWorkerOptions
>;

/**
 * BrowserStack real devices allow exactly one browser context per session.
 * Playwright's built-in `browser` fixture is worker-scoped (and its scope
 * can't be overridden), so a worker that runs more than one spec file
 * reuses the same session and its second `newContext()` call is rejected
 * with "Only one browser context is allowed". Overriding `context` (already
 * test-scoped) to launch its own dedicated browser bypasses the shared
 * `browser` fixture, giving every test its own fresh BrowserStack session
 * regardless of how Playwright distributes spec files across workers.
 *
 * The fixture has its own timeout: launching waits for a real device to be
 * allocated, which can take minutes. By default a fixture shares the test's
 * timeout, so a slow allocation would eat the test body's budget and kill
 * the page mid-check even when the SDK itself succeeded.
 */
export const test = baseTest.extend<{}, {}>({
  context: [
    async (
      { playwright, browserName, connectOptions, launchOptions, headless, channel, _combinedContextOptions },
      use,
      testInfo,
    ) => {
      const browser = await launchWhenTunnelReady((): Promise<Browser> => {
        if (connectOptions) {
          return playwright[browserName].connect(connectOptions.wsEndpoint, {
            ...connectOptions,
            headers: {
              'x-playwright-launch-options': JSON.stringify({ ...launchOptions, headless, channel }),
              ...connectOptions.headers,
            },
          });
        }
        // No arguments, like Playwright's own `browser` fixture: launch() picks up
        // `launchOptions`, `headless` and `channel` from the config by itself.
        return playwright[browserName].launch();
      });
      // Close the browser even if newContext() or the test throws: on a real
      // device the session is otherwise held until BrowserStack's idleTimeout,
      // blocking the retry under parallelsPerPlatform: 1.
      try {
        const context = await browser.newContext(_combinedContextOptions);
        await use(context);
        await markBrowserStackSessionStatus(context, testInfo);
        await context.close();
      } finally {
        await browser.close();
      }
    },
    { scope: 'test', timeout: 600_000 },
  ],
});

/**
 * BrowserStack Local reports "[SUCCESS]" locally a few seconds before the
 * BrowserStack hub sees the tunnel. A connect in that window is rejected
 * instantly with "local testing through BrowserStack is not connected", and
 * since it fails in milliseconds, Playwright's retry hits the same window.
 * Wait and retry that specific error; rethrow anything else.
 */
const TUNNEL_NOT_CONNECTED = 'local testing through BrowserStack is not connected';
const TUNNEL_WAIT_MS = 120_000;
const TUNNEL_RETRY_DELAY_MS = 5_000;

async function launchWhenTunnelReady<T>(launch: () => Promise<T>): Promise<T> {
  const deadline = Date.now() + TUNNEL_WAIT_MS;
  for (;;) {
    try {
      return await launch();
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      if (!message.includes(TUNNEL_NOT_CONNECTED) || Date.now() + TUNNEL_RETRY_DELAY_MS > deadline) {
        throw err;
      }
      console.log(`BrowserStack Local tunnel not registered yet, retrying in ${TUNNEL_RETRY_DELAY_MS / 1000}s...`);
      await new Promise((r) => setTimeout(r, TUNNEL_RETRY_DELAY_MS));
    }
  }
}

/**
 * The BrowserStack SDK marks a session passed/failed in its own after-test
 * hook, which runs after this fixture has already closed the page — it then
 * logs "page closed and no hashed_id — session will remain UNMARKED" (hence
 * `skipSessionStatus: true` in the browserstack-*.yml configs). Mark it here
 * instead, while the page is still open. Outside BrowserStack the executor
 * string is just an argument to a no-op function, so this is harmless.
 */
async function markBrowserStackSessionStatus(context: BrowserContext, testInfo: TestInfo): Promise<void> {
  const page = context.pages().find((p) => !p.isClosed());
  if (page === undefined) {
    return;
  }
  const passed = testInfo.status === testInfo.expectedStatus;
  const reason = passed ? '' : (testInfo.error?.message ?? `status: ${testInfo.status}`).slice(0, 255);
  const command = {
    action: 'setSessionStatus',
    arguments: { status: passed ? 'passed' : 'failed', reason },
  };
  try {
    await page.evaluate(() => {}, `browserstack_executor: ${JSON.stringify(command)}`);
  } catch {
    // Best effort: never fail a test because its status couldn't be reported.
  }
}

export { expect } from '@playwright/test';
