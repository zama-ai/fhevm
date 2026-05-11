import {
    type PlaywrightTestConfig,
    defineConfig,
    devices,
} from "@playwright/test"
/**
 * Read environment variables from file.
 * https://github.com/motdotla/dotenv
 */
import dotenv from "dotenv"
import path from "path"
import { fileURLToPath } from "url"

// Import the necessary function

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)
dotenv.config({ path: path.resolve(__dirname, ".env") })

/**
 * See https://playwright.dev/docs/test-configuration.
 */
const defaultConfig: PlaywrightTestConfig = {
    timeout: 240_000,
    expect: { timeout: 60_000 },
    testDir: "./tests",
    /* Run tests in files in parallel */
    fullyParallel: false,
    /* Fail the build on CI if you accidentally left test.only in the source code. */
    forbidOnly: !!process.env.CI,
    /* Retry on CI only */
    retries: process.env.CI ? 2 : 0,
    /* Opt out of parallel tests on CI. */
    workers: process.env.CI ? 1 : 1,
    /* Reporter to use. See https://playwright.dev/docs/test-reporters */
    reporter: "blob",
    /* Configure projects for major browsers */
    projects: [
        {
            name: "chromium",
            use: { ...devices["Desktop Chrome"], channel: "chrome" },
        },
    ],

    /* Run your local dev server before starting the tests */
    webServer: {
        command: "bun vite dev --port 3000",
        url: "http://localhost:3000",
        // reuseExistingServer: !process.env.CI,
        reuseExistingServer: false,
        // ignoreHTTPSErrors: true,
    },
}

export default defineConfig(defaultConfig)
