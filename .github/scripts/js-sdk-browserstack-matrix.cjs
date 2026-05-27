#!/usr/bin/env node

const fs = require("fs");

const event = process.env.GITHUB_EVENT_NAME;
const input = (name, fallback = "") =>
    process.env[`INPUT_${name.toUpperCase()}`] || fallback;

const platformCatalog = [
    {
        id: "windows-11-chrome",
        label: "Windows 11 - Chrome",
        configTemplate: "browserstack-desktop.yml",
        os: "Windows",
        osKey: "windows-11",
        osVersion: "11",
        browser: "chrome",
        browserVersion: "latest",
    },
    {
        id: "windows-11-edge",
        label: "Windows 11 - Edge",
        configTemplate: "browserstack-desktop.yml",
        os: "Windows",
        osKey: "windows-11",
        osVersion: "11",
        browser: "edge",
        browserVersion: "latest",
    },
    {
        id: "windows-10-chrome",
        label: "Windows 10 - Chrome",
        configTemplate: "browserstack-desktop.yml",
        os: "Windows",
        osKey: "windows-10",
        osVersion: "10",
        browser: "chrome",
        browserVersion: "latest",
    },
    {
        id: "windows-10-edge",
        label: "Windows 10 - Edge",
        configTemplate: "browserstack-desktop.yml",
        os: "Windows",
        osKey: "windows-10",
        osVersion: "10",
        browser: "edge",
        browserVersion: "latest",
    },
    {
        id: "macos-tahoe-chrome",
        label: "macOS Tahoe - Chrome",
        configTemplate: "browserstack-desktop.yml",
        os: "OS X",
        osKey: "macos-tahoe",
        osVersion: "Tahoe",
        browser: "chrome",
        browserVersion: "latest",
    },
    {
        id: "macos-tahoe-edge",
        label: "macOS Tahoe - Edge",
        configTemplate: "browserstack-desktop.yml",
        os: "OS X",
        osKey: "macos-tahoe",
        osVersion: "Tahoe",
        browser: "edge",
        browserVersion: "latest",
    },
    {
        id: "macos-sequoia-chrome",
        label: "macOS Sequoia - Chrome",
        configTemplate: "browserstack-desktop.yml",
        os: "OS X",
        osKey: "macos-sequoia",
        osVersion: "Sequoia",
        browser: "chrome",
        browserVersion: "latest",
    },
    {
        id: "macos-sequoia-edge",
        label: "macOS Sequoia - Edge",
        configTemplate: "browserstack-desktop.yml",
        os: "OS X",
        osKey: "macos-sequoia",
        osVersion: "Sequoia",
        browser: "edge",
        browserVersion: "latest",
    },
    {
        id: "android-15-s25-ultra-chrome",
        label: "Android 15 (Galaxy S25 Ultra) - Chrome",
        configTemplate: "browserstack-mobile.yml",
        deviceName: "Samsung Galaxy S25 Ultra",
        osKey: "android-15-s25-ultra",
        osVersion: "15.0",
        browser: "chrome",
        browserVersion: "latest",
    },
];

const testsAllowed = [
    "bstack:smoke-wasm",
    "bstack:encrypt",
    "bstack:public-decrypt",
    "bstack:user-decrypt",
    "bstack:verify-input",
];

const osAllowed = [...new Set(platformCatalog.map((p) => p.osKey))];
const browserAllowed = [...new Set(platformCatalog.map((p) => p.browser))];

const normalizeList = (raw, allowed, { allowAll = false } = {}) => {
    if (!raw) return [];
    const lower = raw.trim().toLowerCase();
    if (allowAll && lower === "all") return allowed;

    return lower
        .split(/[,\n]/)
        .map((v) => v.trim())
        .filter(Boolean)
        .map((v) =>
            v
                .replace(/[^a-z0-9:]+/g, "-")
                .replace(/-+/g, "-")
                .replace(/^-|-$/g, ""),
        )
        .filter((v) => allowed.includes(v));
};

const requireMatchOrFail = (raw, normalized, label, allowed) => {
    if (!raw) return;
    if (normalized.length) return;
    console.error(
        `No valid ${label} parsed from "${raw}". Allowed values: ${allowed.join(", ")}`,
    );
    process.exit(1);
};

const defaultPlatformId = "windows-11-chrome";
const defaultPlatform = platformCatalog.find((p) => p.id === defaultPlatformId);
const defaultOsKey = defaultPlatform?.osKey || "windows-11";
const defaultBrowser = defaultPlatform?.browser || "chrome";

let platforms;
if (event === "workflow_dispatch") {
    const rawOs = input("oses");
    const rawBrowsers = input("browsers");
    const osSelection = normalizeList(rawOs, osAllowed, { allowAll: true });
    const browserSelection = normalizeList(rawBrowsers, browserAllowed, {
        allowAll: true,
    });

    requireMatchOrFail(rawOs, osSelection, "OS choices", osAllowed);
    requireMatchOrFail(
        rawBrowsers,
        browserSelection,
        "browser choices",
        browserAllowed,
    );

    const osSet = new Set(osSelection.length ? osSelection : [defaultOsKey]);
    const browserSet = new Set(
        browserSelection.length ? browserSelection : [defaultBrowser],
    );

    platforms = platformCatalog.filter(
        (p) => osSet.has(p.osKey) && browserSet.has(p.browser),
    );

    if (!platforms.length) {
        platforms = [defaultPlatform];
    }
} else {
    platforms = [defaultPlatform];
}

let tests;
if (event === "workflow_dispatch") {
    const rawTests = input("tests");
    const parsedTests =
        normalizeList(rawTests, testsAllowed, { allowAll: true }) || [];
    requireMatchOrFail(rawTests, parsedTests, "tests", testsAllowed);
    tests =
        (parsedTests.length && parsedTests) ||
        (rawTests && rawTests.trim().toLowerCase() === "all"
            ? testsAllowed
            : ["bstack:smoke-wasm"]);
} else {
    tests = testsAllowed;
}

const networkInput = input("network", "testnet").toLowerCase();
const network = ["devnet", "testnet", "both"].includes(networkInput)
    ? networkInput
    : "both";

const output = {
    platforms: JSON.stringify(platforms),
    tests: JSON.stringify(tests),
    network,
    logsDir: "log",
    obsTestDetailsDir: "log/.obs_test_details-default",
};

const outputFile = process.env.GITHUB_OUTPUT;
if (!outputFile) {
    console.error("GITHUB_OUTPUT is not set.");
    process.exit(1);
}

const lines = Object.entries(output)
    .map(([key, value]) => `${key}=${value}`)
    .join("\n");

fs.appendFileSync(outputFile, `${lines}\n`);
