#!/usr/bin/env node

const fs = require("node:fs");
const path = require("node:path");
const { spawnSync } = require("node:child_process");

const cwd = process.cwd();

const readEnv = (name) => {
  const value = process.env[name];
  if (!value) throw new Error(`Missing required env: ${name}`);
  return value;
};

const resolveFromCwd = (value) => path.resolve(cwd, value);
const toTaskPath = (value) => path.relative(cwd, value).split(path.sep).join("/");

const parseVersion = (filePath) => {
  const content = fs.readFileSync(filePath, "utf8");
  const match = content.match(/REINITIALIZER_VERSION\s*=\s*(\d+)/);
  if (!match) throw new Error(`Failed to parse REINITIALIZER_VERSION from ${filePath}`);
  return match[1];
};

const run = (args) => {
  const result = spawnSync(args[0], args.slice(1), { cwd, stdio: "inherit" });
  if (result.status !== 0) process.exit(result.status ?? 1);
};

const copyDir = (source, target) => {
  fs.rmSync(target, { force: true, recursive: true });
  fs.mkdirSync(target, { recursive: true });
  fs.cpSync(source, target, { recursive: true });
};

const generatedPreviousRoot = resolveFromCwd(readEnv("UPGRADE_GENERATED_PREVIOUS_ROOT"));
const workPreviousRoot = resolveFromCwd(readEnv("UPGRADE_WORK_PREVIOUS_ROOT"));
const currentContractsDir = resolveFromCwd(process.env.UPGRADE_CURRENT_CONTRACTS_DIR ?? "contracts");
const previousContractsDir = path.join(workPreviousRoot, process.env.UPGRADE_PREVIOUS_CONTRACTS_DIR ?? "contracts");
const addressesDir = resolveFromCwd(process.env.UPGRADE_GENERATED_ADDRESSES_DIR ?? "addresses");
const previousAddressesDir = path.join(workPreviousRoot, process.env.UPGRADE_PREVIOUS_ADDRESSES_DIR ?? "addresses");
const manifestPath = resolveFromCwd(process.env.UPGRADE_MANIFEST_PATH ?? "upgrade-manifest.json");
const copyAddressFiles = (process.env.UPGRADE_COPY_ADDRESS_FILES ?? "")
  .split(",")
  .map((value) => value.trim())
  .filter(Boolean);
const deployNew = process.env.UPGRADE_DEPLOY_NEW === "true";

copyDir(generatedPreviousRoot, workPreviousRoot);
if (copyAddressFiles.length) {
  fs.mkdirSync(previousAddressesDir, { recursive: true });
  for (const fileName of copyAddressFiles) {
    fs.copyFileSync(path.join(addressesDir, fileName), path.join(previousAddressesDir, fileName));
  }
}

const manifest = JSON.parse(fs.readFileSync(manifestPath, "utf8"));
if (!Array.isArray(manifest)) throw new Error(`Upgrade manifest must be an array: ${manifestPath}`);

for (const name of manifest) {
  const oldFile = path.join(previousContractsDir, `${name}.sol`);
  const newFile = path.join(currentContractsDir, `${name}.sol`);

  if (!fs.existsSync(newFile)) {
    throw new Error(`${name} listed in ${manifestPath} but ${newFile} not found`);
  }

  if (!fs.existsSync(oldFile)) {
    if (!deployNew) {
      console.log(`Skipping ${name} (not present in previous release)`);
      continue;
    }
    console.log(`Deploying ${name} (new in target release)`);
    run(["npx", "hardhat", `task:deploy${name}`]);
    continue;
  }

  const oldVersion = parseVersion(oldFile);
  const newVersion = parseVersion(newFile);
  if (oldVersion === newVersion) {
    console.log(`Skipping ${name} (reinitializer unchanged: ${oldVersion})`);
    continue;
  }

  console.log(`Upgrading ${name} (reinitializer ${oldVersion} -> ${newVersion})`);
  run([
    "npx",
    "hardhat",
    `task:upgrade${name}`,
    "--current-implementation",
    `${toTaskPath(oldFile)}:${name}`,
    "--new-implementation",
    `${toTaskPath(newFile)}:${name}`,
    "--use-internal-proxy-address",
    "true",
    "--verify-contract",
    "false",
  ]);
}
