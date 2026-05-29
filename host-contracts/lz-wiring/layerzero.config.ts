/**
 * LayerZero V2 wiring config for the ConfidentialBridge (Sepolia ↔ Amoy).
 *
 * Consumed by `npx hardhat lz:oapp:wire --oapp-config layerzero.config.ts`.
 * The bridge proxies are referenced by `address` (read from env vars), so this
 * workspace doesn't need the bridge's hardhat-deploy artifacts.
 *
 * Env vars required:
 *   SEPOLIA_BRIDGE_ADDRESS       — ConfidentialBridge proxy on Sepolia
 *   POLYGON_AMOY_BRIDGE_ADDRESS  — ConfidentialBridge proxy on Polygon Amoy
 *
 * Both are written to addresses/.env.host as CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS
 * by `task:deployBridge` in the parent project. After deploying on each chain,
 * copy that value into the matching env var here (or in the parent ../.env,
 * which we also load).
 *
 * DVN/confirmations come from the pathway definition below; defaults are
 * testnet-grade (single LZ Labs DVN, 1 confirmation). For production replace
 * with at least 2 independent DVNs from
 * https://docs.layerzero.network/v2/deployments/dvn-addresses.
 */
import { EndpointId } from '@layerzerolabs/lz-definitions';
import { ExecutorOptionType } from '@layerzerolabs/lz-v2-utilities';
import { TwoWayConfig, generateConnectionsConfig } from '@layerzerolabs/metadata-tools';
import { OAppEnforcedOption, OmniPointHardhat } from '@layerzerolabs/toolbox-hardhat';

// The bridge proxies are referenced by `contractName` (the canonical LZ pattern
// for EVM chains). The actual addresses live in `deployments/<network>/ConfidentialBridge.json`
// — those JSON files are generated from env vars by `scripts/sync-deployments.ts`,
// which runs automatically before `pnpm wire` (see package.json's `prewire`
// script). Each per-network deployments dir also carries a `.chainId` file.
const sepoliaContract: OmniPointHardhat = {
  eid: EndpointId.SEPOLIA_V2_TESTNET,
  contractName: 'ConfidentialBridge',
};

const polygonAmoyContract: OmniPointHardhat = {
  eid: EndpointId.AMOY_V2_TESTNET,
  contractName: 'ConfidentialBridge',
};

/**
 * Per-message execution options floor applied by the LZ executor. Our
 * ConfidentialBridge does NOT inherit `OAppOptionsType3` (HandlesSender
 * computes lzReceive gas dynamically from `LZ_RECEIVE_BASE_GAS +
 * n * LZ_RECEIVE_PER_HANDLE_GAS`), so `lz:oapp:wire` will report these as
 * "no-op" for `setEnforcedOptions` and skip the on-chain call. They're kept
 * here as the canonical value for future use if the bridge ever adds
 * enforced-options support.
 *
 * Sized to cover the worst-case MAX_HANDLES (32) payload:
 *   80k base + 32 * 60k per-handle ≈ 2_000_000 gas.
 */
/*const EVM_ENFORCED_OPTIONS: OAppEnforcedOption[] = [
  {
    msgType: 1,
    optionType: ExecutorOptionType.LZ_RECEIVE,
    gas: 2_000_000,
    value: 0,
  },
];
*/

const pathways: TwoWayConfig[] = [
  [
    sepoliaContract,
    polygonAmoyContract,
    // [requiredDVN[], [optionalDVN[], threshold]] — TESTNET-grade single DVN.
    // For production replace with at least 2 independent operators, e.g.
    //   [['LayerZero Labs', 'Google Cloud'], []]
    [['LayerZero Labs'], []],
    // Block confirmations [src→dst, dst→src]. Testnet is permissive; production
    // chooses per-chain reorg-safe values (Sepolia commonly 15+, Amoy 10+).
    [2, 3],
    [undefined, undefined],
  ],
];

export default async function () {
  const connections = await generateConnectionsConfig(pathways);
  return {
    contracts: [{ contract: sepoliaContract }, { contract: polygonAmoyContract }],
    connections,
  };
}
