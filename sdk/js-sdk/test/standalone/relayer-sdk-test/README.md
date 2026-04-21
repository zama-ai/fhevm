# Zama relayer-sdk Web Test Harness App

This project is a test harness for the web version of the Zama [relayer-sdk](https://github.com/zama-ai/relayer-sdk/tree/main) package.
It is a simple app which provides multiple routes to trigger the different functions provided by the `relayer-sdk` API.

## Table of Contents

- [Tech Stack](#tech-stack)
- [Setup](#setup)
- [Running the Web App Locally](#running-the-web-app-locally)
- [Running Tests](#running-tests)
  - [Running Playwright Tests Locally](#running-playwright-tests-locally)
  - [Running BrowserStack Tests Locally](#running-browserstack-tests-locally)
  - [Running BrowserStack Tests in CI](#running-browserstack-tests-in-ci)
- [Hardcoded Handles vs Fresh Handles](#hardcoded-handles-vs-fresh-handles)
- [User Decryption Tests](#user-decryption-tests)
- [Contributing](#contributing)
  - [Updating the SDK Version](#updating-the-sdk-version)
  - [Adding New Routes](#adding-new-routes)
  - [Adding New Tests](#adding-new-tests)
- [Project Structure](#project-structure)

## Tech Stack

This project uses [Bun](https://bun.sh/) as its JavaScript runtime.

### Web App

- [Vite](https://vite.dev/) - Build tool and dev server
- [React](https://react.dev/) - UI framework
- [TanStack Router](https://tanstack.com/router/latest) - File-based routing

### Automated E2E Tests

- [Playwright](https://playwright.dev/) - E2E testing framework
- [BrowserStack](https://www.browserstack.com/) - Cloud testing platform

E2E tests reproduce navigation through the web app under real browser conditions.
Playwright allows writing such tests with a simple API, reproducing real conditions of using the underlying `relayer-sdk` within a web app.

BrowserStack is a Cloud Testing Platform which allows running E2E tests on a variety of OS/browser combinations (Windows, macOS, Android, iOS) using [BrowserStack Automate](https://www.browserstack.com/automate).
The browsers/engines supported by BrowserStack are available on this [page](https://www.browserstack.com/list-of-browsers-and-platforms/playwright).

### Linter/Formatter

ESLint and Prettier are configured for code quality and formatting.

## Setup

### Prerequisites

- [Bun](https://bun.sh/) installed on your machine
- A BrowserStack account (for running BrowserStack tests)
- A funded Ethereum Sepolia account private key (for user decryption tests)
  - Funds on Ethereum Sepolia are only required for authorization of freshly generated handles (outside of CI tests).

### Installation

Install the project dependencies:

```bash
bun install
```

### Environment Variables

Copy the `.env.example` file to `.env`:

```bash
cp .env.example .env
```

Fill in the following variables:

| Variable                  | Description                                           | Required For               |
| ------------------------- | ----------------------------------------------------- | -------------------------- |
| `BROWSERSTACK_USERNAME`   | Your BrowserStack username                            | BrowserStack tests         |
| `BROWSERSTACK_ACCESS_KEY` | Your BrowserStack access key                          | BrowserStack tests         |
| `VITE_PRIVATE_KEY`        | Private key of an account funded with Sepolia ETH    | User decryption tests      |
| `VITE_MNEMONIC`           | Mnemonic phrase (takes precedence over private key)  | User decryption tests      |
| `VITE_SEPOLIA_RPC_URL`    | Sepolia RPC URL (optional, uses default if not set)  | On-chain interactions      |

You can provide either `VITE_PRIVATE_KEY` or `VITE_MNEMONIC` for user decryption tests. If both are set, `VITE_MNEMONIC` takes precedence.

## Running the Web App Locally

Start the development server:

```bash
bun vite dev
```

The app will be available at `http://localhost:5173` (or another port if 5173 is in use).

Use the `--cors` flag to enable CORS if needed:

```bash
bun vite dev --cors
```

To specify a port:

```bash
bun vite dev --port 3000
```

## Running Tests

### Running Playwright Tests Locally

Playwright tests run against a local dev server. The test configuration automatically starts the dev server on port 3000.

First, install Playwright browser engines (required once after `bun install`):

```bash
bun playwright install --with-deps
```

Run all E2E tests:

```bash
bun playwright:all
```

Run specific test suites:

```bash
bun playwright:init           # SDK initialization tests
bun playwright:zkproof        # ZK proof tests
bun playwright:encrypt        # Encryption tests
bun playwright:verify-input   # Input verification tests
bun playwright:public-decrypt # Public decryption tests
bun playwright:user-decrypt   # User decryption tests (hardcoded handles)
bun playwright:user-decrypt-multi  # Multi user decryption tests (hardcoded handles)
```

### Running BrowserStack Tests Locally

To run tests on BrowserStack from your local machine, ensure your `.env` file contains valid `BROWSERSTACK_USERNAME` and `BROWSERSTACK_ACCESS_KEY` values.

Run all tests on BrowserStack:

```bash
bun bstack:all:desktop  # Run all tests on desktop browsers
bun bstack:all:mobile   # Run all tests on mobile browsers
```

Run specific test suites on BrowserStack:

```bash
# Desktop browsers (Windows, macOS)
bun bstack:init:desktop
bun bstack:zkproof:desktop
bun bstack:encrypt:desktop
bun bstack:verify-input:desktop
bun bstack:public-decrypt:desktop
bun bstack:user-decrypt:desktop
bun bstack:user-decrypt-multi:desktop

# Mobile browsers (Android, iOS)
bun bstack:init:mobile
bun bstack:zkproof:mobile
bun bstack:encrypt:mobile
bun bstack:verify-input:mobile
bun bstack:public-decrypt:mobile
bun bstack:user-decrypt:mobile
bun bstack:user-decrypt-multi:mobile
```

BrowserStack configuration files:
- `browserstack-desktop.yml` - Desktop browser configurations (Windows, macOS)
- `browserstack-mobile.yml` - Mobile browser configurations (Android, iOS)

To monitor running tests and view results, go to the [BrowserStack Automate Dashboard](https://automate.browserstack.com/).

### Running BrowserStack Tests in CI

BrowserStack tests run automatically in CI via the `.github/workflows/02-tests-browserstack.yml` workflow.

The available OS/browser combinations and test suites are defined in `.github/scripts/build-browserstack-matrix.cjs`. This script builds the test matrix based on the trigger type and input parameters.

#### Automatic Triggers

Tests run automatically on:
- **Push to `main`**: When changes are made to `src/**` (runs `bstack:init` and `bstack:verify-input` on Windows 11 Chrome)
- **Pull requests to `main`**: When changes are made to `src/**` (runs `bstack:init` and `bstack:verify-input` on Windows 11 Chrome)
- **Release published**: Runs all tests on all supported platforms

#### Manual Trigger (workflow_dispatch)

You can manually trigger the workflow from GitHub Actions with customizable options:

| Input      | Description                                                                                              | Default       |
| ---------- | -------------------------------------------------------------------------------------------------------- | ------------- |
| `network`  | Network to target: `devnet`, `testnet`, or `both`                                                        | `both`        |
| `oses`     | Comma-separated OS list (see below), or `all`                                                            | `windows-11`  |
| `browsers` | Comma-separated browsers: `chrome`, `edge`, or `all`                                                     | `chrome`      |
| `tests`    | Comma-separated tests (see below), or `all`                                                              | `bstack:init` |

**Available OS options:**
- `windows-11`, `windows-10`
- `macos-tahoe`, `macos-sequoia`
- `android-15-s25-ultra`

**Available test suites:**
- `bstack:init`, `bstack:zkproof`, `bstack:encrypt`, `bstack:verify-input`
- `bstack:public-decrypt`, `bstack:user-decrypt`, `bstack:user-decrypt-multi`

Example: To run encryption tests on Windows 11 Chrome targeting testnet only, select:
- network: `testnet`
- oses: `windows-11`
- browsers: `chrome`
- tests: `bstack:encrypt`

## Hardcoded Handles vs Fresh Handles

The project has two types of decryption routes:

### Hardcoded Handles (CI-friendly)

These routes use pre-generated handles stored in:
- `src/utils/handles.ts` (for the web app)
- `tests/handles.ts` (for Playwright tests)

Routes:
- `/public-decrypt` - Public decryption with hardcoded handles
- `/user-decrypt` - User decryption with hardcoded handles
- `/user-decrypt-multi` - Multi-user decryption with hardcoded handles

These are the routes tested in CI because the handles are pre-computed and don't require on-chain transactions.

### Fresh Handles (Not in CI)

These routes generate new handles at runtime by:
1. Creating encrypted inputs
2. Generating ZK proofs
3. Verifying on-chain
4. Making handles decryptable
5. Decrypting

Routes:
- `/public-decrypt-fresh-handles` - Public decryption with fresh handles
- `/user-decrypt-fresh-handles` - User decryption with fresh handles
- `/user-decrypt-multi-fresh-handles` - Multi-user decryption with fresh handles

Tests exist for these routes but are **not included in CI** because they:
- Require on-chain transactions (cost gas)
- Take significantly longer to execute
- May be flaky due to network conditions

To run fresh handles tests locally:

```bash
bun playwright test tests/user-decrypt-fresh-handles.spec.ts
bun playwright test tests/user-decrypt-multi-fresh-handles.spec.ts
```

## User Decryption Tests

User decryption tests (with hardcoded handles) have a specific requirement: **the handles must be decryptable by the user whose private key is in your `.env` file**.

### In CI

The CI uses a dedicated private key (stored as a GitHub secret) whose corresponding address matches the hardcoded handles in the repository. Tests run successfully because the user address in the hardcoded handles matches the CI's private key.

### Locally

If you want to run user decryption tests locally:

1. **Option A**: Use the same private key as CI (if you have access to it)

2. **Option B**: Generate new handles for your address
   - Run the fresh handles routes to create handles for your address
   - Update the hardcoded values in **both** files:
     - `src/utils/handles.ts`
     - `tests/handles.ts`
   - Ensure the `user` field in the handles matches your wallet address

The hardcoded user address in the repository is `0xE2f899d22b00854D8B4d2704D9429be9C411c433`. If your private key doesn't correspond to this address, user decryption tests will fail.

## Contributing

### Updating the SDK Version

When a new version of `@zama-fhe/relayer-sdk` is released:

1. Update the package version in `package.json`:

```json
{
  "dependencies": {
    "@zama-fhe/relayer-sdk": "X.Y.Z"
  }
}
```

2. Update the CDN script version in `index.html`:

```html
<script
  src="https://cdn.zama.org/relayer-sdk-js/X.Y.Z/relayer-sdk-js.umd.cjs"
  type="text/javascript"
  defer
></script>
```

3. Install updated dependencies:

```bash
bun install
```

4. Test the changes locally before committing.

### Adding New Routes

Routes are file-based using TanStack Router.

1. Create a new route file in `src/routes/`:

```tsx
// src/routes/my-new-route.tsx
import { createFileRoute } from "@tanstack/react-router"

export const Route = createFileRoute("/my-new-route")({
  component: MyNewRoute,
})

function MyNewRoute() {
  return <div>My new route</div>
}
```

2. Create the corresponding component in `src/components/` if needed.

3. Add CSS styles in a component-specific `*.css` file if needed.

### Adding New Tests

Tests are located in the `tests/` folder and follow the `*.spec.ts` naming convention.

1. Create a new test file:

```typescript
// tests/my-new-test.spec.ts
import { expect, test } from "@playwright/test"

test("my new test", async ({ page }) => {
  await page.goto("http://localhost:3000/my-new-route")
  // Add assertions
})
```

2. Add npm scripts to `package.json` for the new test:

```json
{
  "scripts": {
    "playwright:my-new-test": "playwright test tests/my-new-test.spec.ts",
    "bstack:my-new-test": "browserstack-node-sdk playwright test tests/my-new-test.spec.ts",
    "bstack:my-new-test:desktop": "BROWSERSTACK_CONFIG_FILE=browserstack-desktop.yml browserstack-node-sdk playwright test tests/my-new-test.spec.ts",
    "bstack:my-new-test:mobile": "BROWSERSTACK_CONFIG_FILE=browserstack-mobile.yml browserstack-node-sdk playwright test tests/my-new-test.spec.ts"
  }
}
```

3. If the test should be available in CI:
   - Add it to the `testsAllowed` array in `.github/scripts/build-browserstack-matrix.cjs`
   - Update the workflow's test input options in `.github/workflows/02-tests-browserstack.yml`

See the [Playwright docs](https://playwright.dev/docs/intro) for more information on writing tests.

## Project Structure

```
.
├── src/
│   ├── components/        # React components (UI + logic)
│   ├── routes/            # TanStack Router file-based routes
│   ├── hooks/             # Custom React hooks
│   └── utils/             # Shared utility functions
│       └── handles.ts     # Hardcoded decryption handles (web app)
├── tests/
│   ├── *.spec.ts          # Playwright E2E tests
│   ├── handles.ts         # Hardcoded decryption handles (tests)
│   └── utils.ts           # Test utilities
├── .github/
│   ├── scripts/
│   │   └── build-browserstack-matrix.cjs  # CI test matrix configuration
│   └── workflows/         # CI/CD workflows
├── browserstack-desktop.yml  # BrowserStack desktop config (local)
├── browserstack-mobile.yml   # BrowserStack mobile config (local)
├── playwright.config.ts      # Playwright configuration
└── index.html                # Entry HTML with CDN script
```
