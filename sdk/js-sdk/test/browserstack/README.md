# BrowserStack Tests

These tests run the SDK in real browsers on real Android and iOS phones and on Windows and macOS desktops, using [BrowserStack](https://www.browserstack.com/). They check that the SDK's WASM modules (TFHE, TKMS) load and work outside Node, and that the full encrypt and decrypt flows work against Sepolia.

The tests are driven by Playwright through `browserstack-node-sdk`. A Vite dev server runs on your machine (port `3333`). BrowserStack Local opens a tunnel so the remote browser can reach it at `bs-local.com:3333`. Each test opens an HTML page, the page runs the SDK, and the spec waits for the page to report `pass` or `fail`.

## What the tests do

There are two groups of suites.

### Suites in this directory

Each suite has a spec in `specs/`, an HTML page in `pages/`, and an in-page script in `scripts/`. All of them target **Sepolia** (`sepolia` from `src/core/chains`) through `https://ethereum-sepolia-rpc.publicnode.com`.

| npm script | What it does | Needs `MNEMONIC` | Sends a transaction |
| --- | --- | --- | --- |
| `bstack:encrypt` | Creates an encrypt client, runs `init()` (loads TFHE WASM and fetches the global FHE key), and encrypts one value of each type (`bool`, `uint8` to `uint256`, `address`) with `encryptValues`. Checks that it gets back 8 handles and an input proof. Uses dummy contract and user addresses. | No | No |
| `bstack:verify-input` | Encrypts a `uint8` and submits it to `FHETest.addEuint8()`. The contract checks the ZK input proof on-chain. Passes if the transaction succeeds. | Yes | Yes (pays gas) |
| `bstack:user-decrypt` | Makes sure the wallet owns an `ebool` handle on `FHETest`, and calls `initFheTest()` if it doesn't. Then it creates a decrypt client, generates a transport keypair (TKMS WASM), signs an EIP-712 decryption permit, and decrypts the handle through the relayer. The result must be `true`. | Yes | Only on the first run for a wallet |

The `FHETest` address comes from `test/chains/chain-defaults.json` (`sepolia.fheTestAddress`). The page fetches that file from the dev server.

### Browser smoke suites run on BrowserStack

These scripts run the existing `test/browser-smoke` suite (`smoke-wasm`, `smoke-base64`, `smoke-cdn`, `smoke-coexistence`, `smoke-csp-block`) on BrowserStack instead of local Playwright browsers. They use `test/browser-smoke/playwright.config.ts`, an HTTPS dev server (`test/browser-smoke/server.ts`), and the fixtures in `test/browser-smoke/fixtures.ts`.

| npm script | What it runs |
| --- | --- |
| `bstack:smoke-wasm` | Only `smoke-wasm.spec.ts`: the client initializes with WASM loaded from URLs. |
| `bstack:browser:mobile` | The whole smoke suite, on the devices in `browserstack-mobile.yml`. |
| `bstack:browser:desktop` | The whole smoke suite, on the desktops in `browserstack-desktop.yml`. |

## Prerequisites

1. **A BrowserStack account** with Automate access. Your username and access key are on the Automate dashboard under *Access Key*.
2. **Dependencies installed** in `sdk/js-sdk` with `npm install`, then `npm run bstack:install`. The second command installs this directory's own `package.json` (see [Dependencies](#dependencies)). `browserstack-node-sdk` also downloads and starts the BrowserStack Local binary for you.
3. **A funded Sepolia wallet**, for `verify-input` and `user-decrypt` only. The tests derive the signer from a BIP-39 mnemonic (first account, default path). The wallet needs Sepolia ETH for gas.

## Dependencies

`test/browserstack` is its own npm package, like `test/browser-next`. Its `package.json` installs only `browserstack-node-sdk`, into `test/browserstack/node_modules`. `browserstack-node-sdk` uses BrowserStack's own license, and some of its dependencies use licenses that aren't on the allowlist. `npm run licenses:check` only covers `sdk/js-sdk`'s dependencies, so these packages don't fail it.

Don't add `@playwright/test` to this package. `browserstack-node-sdk` and the specs must load the same copy of Playwright, otherwise the run fails with "Playwright Test did not expect test() to be called here". Without a local copy, both resolve the one in `sdk/js-sdk/node_modules`.

The `bstack:*` scripts are in `sdk/js-sdk/package.json` and run from `sdk/js-sdk`.

## Setup

Export your BrowserStack credentials:

```bash
export BROWSERSTACK_USERNAME=<username>
export BROWSERSTACK_ACCESS_KEY=<access-key>
```

Provide the mnemonic. The in-page scripts read `import.meta.env.MNEMONIC`. Vite exposes any variable whose name starts with `MNEMONIC`, taken from either:

- `test/.env` (`vite.config.ts` sets `envDir` to `test/`), or
- your shell: `export MNEMONIC="word1 word2 ..."`.

`browserstack-mobile.yml` also points at `test/.env` (`envFile`). `test/.env` is git-ignored. Never commit a mnemonic.

## Running

Run all commands from `sdk/js-sdk`:

```bash
npm run bstack:encrypt          # no MNEMONIC needed
npm run bstack:verify-input     # needs MNEMONIC
npm run bstack:user-decrypt     # needs MNEMONIC

npm run bstack:smoke-wasm
npm run bstack:browser:mobile
npm run bstack:browser:desktop
```

### Choosing devices

The platforms are listed in two config files.

`browserstack-mobile.yml`:

- Samsung Galaxy S25 Ultra (Android 15) / Chrome
- Google Pixel 9 (Android 15) / Chrome
- iPhone 16 (iOS 18) / Safari
- iPhone 14 (iOS 16) / Safari

`browserstack-desktop.yml`:

- Windows 11 / Chrome
- macOS Tahoe / Chrome

`bstack:browser:mobile` and `bstack:browser:desktop` always use their own file. The other `bstack:*` scripts use `browserstack-mobile.yml` unless `BROWSERSTACK_CONFIG_FILE` is set.

The test runs once on every platform in the file. To target a single platform, or to run on desktop, copy the YAML, keep one entry under `platforms:`, and point `BROWSERSTACK_CONFIG_FILE` at the copy:

```bash
BROWSERSTACK_CONFIG_FILE=test/browserstack/my-iphone.yml npm run bstack:encrypt
BROWSERSTACK_CONFIG_FILE=test/browserstack/browserstack-desktop.yml npm run bstack:encrypt
```

You can pass extra Playwright arguments after `--`, for example `npm run bstack:encrypt -- --grep encrypt`.

### Debugging the pages locally

The pages are plain HTML and work in any local browser. Start the dev server and open a page:

```bash
npx vite --config test/browserstack/vite.config.ts
# then open http://localhost:3333/test/browserstack/pages/encrypt.html
```

Each page writes a step-by-step log to its `#log` element and to the console. When it finishes it adds `<div id="result" data-status="pass|fail">`, which is the element the spec waits for.

## Results

- **BrowserStack Automate dashboard.** Builds are grouped by `projectName` and `buildName` from the YAML. Each session has a video, network logs and console logs.
- **Terminal.** When a test fails, the spec prints the page's full `#log` content.
- **Session status.** Both config files set `testContextOptions.skipSessionStatus: true`. The BrowserStack SDK would otherwise try to mark the session after the page is already closed, and that fails. Instead, every spec (in this directory and in `test/browser-smoke`) imports `test` from `test/browser-smoke/fixtures.ts`, which marks pass or fail itself while the page is still open.

The SDK writes `browserstackSetupConfig.json`, `playwright-browserstack-sdk.config.ts` and `log/` into `sdk/js-sdk` on every run. After a failed run it can also write `browserstack.err`. These files are git-ignored. `browserstackSetupConfig.json` contains your credentials in plain text.

## Gotchas

These explain the configuration choices. Read them before you change any of it.

- **Use `bs-local.com`, not `localhost`.** BrowserStack Local always routes `bs-local.com` back to your machine. `localhost` doesn't work on iOS Safari, and a LAN IP isn't reliably tunneled on real devices. Vite binds on all interfaces and allows `bs-local.com` as a host.
- **COOP/COEP headers.** The dev servers send `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp` so the page is cross-origin isolated, which the WASM multithreading needs.
- **HTTPS over HTTP/1.1 for the smoke suite.** A secure context is needed for `crypto.subtle` and cross-origin isolation, so `server.ts` serves HTTPS with a self-signed certificate. It uses `node:https` because Vite's built-in HTTPS uses HTTP/2, which fails through the tunnel on Android Chrome (`ERR_HTTP2_PROTOCOL_ERROR`).
- **`forceLocal: false` on mobile.** With `forceLocal: true`, all traffic, including relayer calls, goes through the tunnel and your machine. On real mobile devices that path was unreliable ("Failed to fetch"), so `browserstack-mobile.yml` turns it off. `browserstack-desktop.yml` keeps `forceLocal: true`.
- **One browser per test, one worker.** A real-device session allows only one browser context, so `fixtures.ts` launches a new browser for every test. The smoke config uses `workers: 1` so connections don't queue up for a device and time out.
- **Long timeouts.** WASM loading, fetching the FHE key and waiting for Sepolia confirmations are slow on phones. Tests allow 5 minutes, and `verify-input` allows 10.
