#!/usr/bin/env bash
set -euo pipefail

# Streamlines README steps 4–6 for deploying and wiring Zama OFT contracts.

RUN_VERIFY=false
OFT_NETWORK="arbitrum-testnet"
OAPP_CONFIG="layerzero.config.ts"

SUPPORTED_OFT_NETWORKS=("arbitrum-testnet" "gateway-testnet")

function usage() {
	cat <<'USAGE'
Usage: ./scripts/deploy_zama_oft_testnet.sh [--verify] [--oft-network <network>] [--oapp-config <file>]

Options:
  --oapp-config     LayerZero OApp config file passed to lz:oapp:wire (default: layerzero.config.ts).
  --oft-network     Target network for the ZamaOFT deployment (arbitrum-testnet | gateway-testnet).
  --verify          Run the optional Etherscan verification commands.
USAGE
}

function ensure_supported_oft_network() {
	local candidate="$1"
	local network
	for network in "${SUPPORTED_OFT_NETWORKS[@]}"; do
		if [[ $candidate == "$network" ]]; then
			return 0
		fi
	done
	printf 'Error: unsupported OFT network "%s". Supported values: %s.\n' \
		"$candidate" "${SUPPORTED_OFT_NETWORKS[*]}" >&2
	exit 1
}

while [[ $# -gt 0 ]]; do
	case "$1" in
	--verify)
		RUN_VERIFY=true
		shift
		;;
	-h | --help)
		usage
		exit 0
		;;
	--oft-network)
		if [[ $# -lt 2 ]]; then
			printf 'Error: --oft-network requires a value.\n' >&2
			exit 1
		fi
		OFT_NETWORK="$2"
		shift 2
		;;
	--oft-network=*)
		OFT_NETWORK="${1#*=}"
		shift
		;;
	--oapp-config)
		if [[ $# -lt 2 ]]; then
			printf 'Error: --oapp-config requires a value.\n' >&2
			exit 1
		fi
		OAPP_CONFIG="$2"
		shift 2
		;;
	--oapp-config=*)
		OAPP_CONFIG="${1#*=}"
		shift
		;;
	--)
		shift
		break
		;;
	*)
		printf 'Error: unknown option %s\n' "$1" >&2
		exit 1
		;;
	esac
done

ensure_supported_oft_network "$OFT_NETWORK"

case "$OFT_NETWORK" in
arbitrum-testnet)
	OFT_NETWORK_LABEL="Arbitrum Sepolia"
	;;
gateway-testnet)
	OFT_NETWORK_LABEL="Gateway Testnet"
	;;
*)
	# This can't happen because of ensure_supported_oft_network, but keep a guard.
	OFT_NETWORK_LABEL="$OFT_NETWORK"
	;;
esac

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

if [[ ! -f $OAPP_CONFIG ]]; then
	printf 'Error: OApp config file %s not found.\n' "$OAPP_CONFIG" >&2
	exit 1
fi

require_env_value PRIVATE_KEY
require_env_value SEPOLIA_RPC_URL
require_env_value INITIAL_SUPPLY_RECEIVER
require_env_value INITIAL_ADMIN

note "Ensure the deployer wallet is funded on Ethereum Sepolia and ${OFT_NETWORK_LABEL}."

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
const contents = fs.readFileSync(configPath, 'utf8');
const blockPattern = /(oftAdapter:\s*\{)([\s\S]*?)(\n\s*\},)/;
const match = contents.match(blockPattern);

if (!match) {
  console.error('Error: oftAdapter block not found.');
  process.exit(1);
}

const closingIndentMatch = match[3].match(/\n(\s*)\},/);
const closingIndent = closingIndentMatch ? closingIndentMatch[1] : '';
const innerIndent = `${closingIndent}  `;
const replacement = `${match[1]}\n${innerIndent}tokenAddress: "${address}",${match[3]}`;
const updated = contents.replace(blockPattern, replacement);

if (updated === contents) {
  process.exit(0);
}

fs.writeFileSync(configPath, updated);
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

info "Deploy ZamaOFT on ${OFT_NETWORK_LABEL}"
note "Running lz:deploy in CI mode for ZamaOFT on ${OFT_NETWORK}."
npx hardhat lz:deploy --ci --networks "${OFT_NETWORK}" --tags ZamaOFT

# Step 5: Optional verification
if [[ $RUN_VERIFY == true ]]; then
	info "Verifying ZamaERC20 and ZamaOFTAdapter on Ethereum Sepolia"
	if ! pnpm verify:etherscan:ethereum:sepolia; then
		note "Verification command returned an error; check Etherscan manually."
	fi

	if [[ $OFT_NETWORK == "arbitrum-testnet" ]]; then
		info "Verifying ZamaOFT on ${OFT_NETWORK_LABEL}"
		if ! pnpm verify:etherscan:arbitrum:sepolia; then
			note "Verification command returned an error; check Arbiscan manually."
		fi
	else
		info "Skipping ZamaOFT verification on ${OFT_NETWORK_LABEL}"
		note "No verification command configured for ${OFT_NETWORK}; rerun once scripts are available."
	fi
else
	info "Skipping Etherscan verification"
fi

# Step 6: Wire contracts
info "Wire ZamaOFTAdapter (Ethereum Sepolia) with ZamaOFT (${OFT_NETWORK_LABEL})"
note "Running lz:oapp:wire in CI mode using ${OAPP_CONFIG}."
npx hardhat lz:oapp:wire --ci --oapp-config "${OAPP_CONFIG}"

info "Deployment flow complete"
