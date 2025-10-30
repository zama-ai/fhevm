#!/usr/bin/env bash
set -euo pipefail

# Streamlines README steps 4–6 for deploying and wiring Zama OFT contracts.

RUN_VERIFY=false

while [[ $# -gt 0 ]]; do
	case "$1" in
	--verify)
		RUN_VERIFY=true
		shift
		;;
	-h | --help)
		cat <<'USAGE'
Usage: ./deploy_zama_oft_gateway_testnet.sh [--verify]

Options:
  --verify   Run the optional Etherscan verification commands.
USAGE
		exit 0
		;;
	*)
		printf 'Error: unknown option %s\n' "$1" >&2
		exit 1
		;;
	esac
done

function info() {
	printf '\n> %s\n' "$1"
}

function note() {
	printf ' - %s\n' "$1"
}

function ensure_command() {
	if ! command -v "$1" >/dev/null 2>&1; then
		printf 'Error: %s is required but not installed.\n' "$1" >&2
		exit 1
	fi
}

function require_env_value() {
	local key="$1"
	local line
	line=$(grep -E "^${key}=" .env 2>/dev/null | tail -n1 || true)
	if [[ -z ${line} ]]; then
		printf 'Error: %s must be set in .env before running this script.\n' "$key" >&2
		exit 1
	fi
	local value="${line#*=}"
	value="${value//\"/}"          # strip quotes if present
	value="${value//\'/}"          # strip single quotes if present
	value="${value//[[:space:]]/}" # remove whitespace
	if [[ -z ${value} ]]; then
		printf 'Error: %s cannot be empty in .env.\n' "${key}" >&2
		exit 1
	fi
}

ensure_command pnpm
ensure_command npx
ensure_command node

# Pre-flight reminders
info "Checking prerequisite setup"
if [[ ! -f .env ]]; then
	printf 'Error: .env file not found. Make sure to setup your secrets before running this script.\n' >&2
	exit 1
fi

require_env_value PRIVATE_KEY
require_env_value SEPOLIA_RPC_URL
require_env_value RPC_URL_ZAMA_GATEWAY_TESTNET
require_env_value INITIAL_SUPPLY_RECEIVER
require_env_value INITIAL_ADMIN

note "Ensure the deployer wallet is funded on Ethereum Sepolia and Gateway Testnet."

# Step 4: Deploy contracts
info "Installing dependencies with pnpm"
pnpm install --frozen-lockfile

info "Deploy ZamaERC20 on Ethereum Sepolia"
note "Running lz:deploy in CI mode for ZamaERC20 on ethereum-testnet."
npx hardhat lz:deploy --ci --networks ethereum-testnet --tags ZamaERC20

info "Updating hardhat.config.ts with the deployed ZamaERC20 address"
deployments_file="deployments/ethereum-testnet/ZamaERC20.json"
if [[ ! -f $deployments_file ]]; then
	printf 'Error: %s not found. Ensure the deployment completed successfully.\n' "$deployments_file" >&2
	exit 1
fi

zama_token_address=$(node -e "const fs=require('fs'); const data=JSON.parse(fs.readFileSync('${deployments_file}','utf8')); if(!data.address){process.exit(1);} console.log(data.address);")

if [[ -z $zama_token_address ]]; then
	printf 'Error: Could not read ZamaERC20 address from %s.\n' "$deployments_file" >&2
	exit 1
fi

note "Detected ZamaERC20 address: $zama_token_address"

TARGET_ADDRESS="$zama_token_address" node <<'NODE'
const fs = require('fs');
const address = process.env.TARGET_ADDRESS;
if (!address) {
  console.error('Error: TARGET_ADDRESS env var is missing.');
  process.exit(1);
}

const configPath = 'hardhat.config.ts';
let contents = fs.readFileSync(configPath, 'utf8');
const blockPattern = /(oftAdapter:\s*\{)([\s\S]*?)(\n\s*\},)/g;

let match;
let updated = contents;
let replacementCount = 0;
const regex = new RegExp(blockPattern);

const matches = [];
while ((match = regex.exec(contents)) !== null) {
  matches.push({
    fullMatch: match[0],
    opening: match[1],
    content: match[2],
    closing: match[3],
    index: match.index
  });
}

if (matches.length === 0) {
  console.error('Error: oftAdapter block not found.');
  process.exit(1);
}

console.log(`Found ${matches.length} oftAdapter block(s)`);

for (let i = matches.length - 1; i >= 0; i--) {
  const m = matches[i];

  const closingIndentMatch = m.closing.match(/\n(\s*)\},/);
  const closingIndent = closingIndentMatch ? closingIndentMatch[1] : '';
  const innerIndent = `${closingIndent}    `;

  const replacement = `${m.opening}\n${innerIndent}tokenAddress: '${address}',${m.closing}`;

  updated = updated.substring(0, m.index) + replacement + updated.substring(m.index + m.fullMatch.length);
  replacementCount++;
}

if (updated === contents) {
  console.log('No changes needed.');
  process.exit(0);
}

fs.writeFileSync(configPath, updated);
console.log(`Updated ${replacementCount} oftAdapter block(s) with address: ${address}`);
NODE

current_token_address=$(
	node <<'NODE'
const fs = require('fs');
const text = fs.readFileSync('hardhat.config.ts', 'utf8');
const match = text.match(/tokenAddress:\s*["\'](0x[0-9a-fA-F]+)["\']/);
if (!match) {
  console.error('Error: tokenAddress entry not found after update.');
  process.exit(1);
}
console.log(match[1]);
NODE
)

note "networks.ethereum-testnet.oftAdapter.tokenAddress is now set to $current_token_address"

info "Deploy ZamaOFTAdapter on Ethereum Sepolia"
note "Running lz:deploy in CI mode for ZamaOFTAdapter on ethereum-testnet."
npx hardhat lz:deploy --ci --networks ethereum-testnet --tags ZamaOFTAdapter

info "Deploy ZamaOFT on Zama Gateway Testnet"
note "Running lz:deploy in CI mode for ZamaOFT on gateway-testnet."
npx hardhat lz:deploy --ci --networks gateway-testnet --tags ZamaOFT

# Step 5: Wire contracts
info "Wire ZamaOFTAdapter (Ethereum Sepolia) with ZamaOFT (Gateway Testnet)"
note "Running lz:oapp:wire in CI mode using layerzero.config.gatewaytestnet.ts."
npx hardhat lz:oapp:wire --ci --oapp-config layerzero.config.gatewaytestnet.ts

info "Deployment flow complete"

# Step 6: Optional verification
if [[ $RUN_VERIFY == true ]]; then
	info "Verifying ZamaERC20 and ZamaOFTAdapter on Ethereum Sepolia"
	if ! pnpm verify:etherscan:ethereum:sepolia; then
		note "Verification command returned an error; check Etherscan manually."
	fi

	info "Verifying ZamaOFT on Gateway Testnet"
	if ! pnpm verify:etherscan:gateway:testnet; then
		note "Verification command returned an error; check Blockscout manually."
	fi
else
	info "Skipping Etherscan verification"
fi
