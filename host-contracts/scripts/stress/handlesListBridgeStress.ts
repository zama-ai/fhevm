/**
 * Cross-chain stress test for the {HandlesListConfidentialOApp} example app.
 *
 * Given a `HandlesListConfidentialOApp` already deployed and wired (peers set) on
 * both chains (see LIVE_TESTNET_BRIDGE_RUNBOOK.md, Step 7.6), this script:
 *
 *   1. From the chosen *source* chain, fires one `generateAndSendHandlesList` tx for
 *      every (countHandles, payloadLength) couple of:
 *         handleCounts = [1, 2, 4, 8, 16, 32]
 *         payloadLens  = [0, 1, 64, 256, 1024, 8192]
 *      i.e. 36 bridging transactions in total. For each, the source-side guid is
 *      derived from the EndpointV2 `PacketSent(encodedPayload, ...)` event (the guid
 *      is embedded in `encodedPayload` at byte offset 81).
 *
 *   2. Then waits (max 10 minutes, otherwise it throws a timeout error) while polling
 *      the *destination* chain's EndpointV2 for the outcome of each guid:
 *         - ComposeDelivered(guid) -> both lzReceive and lzCompose succeeded  (SUCCESS, terminal)
 *         - LzReceiveAlert(guid)   -> an lzReceive attempt failed             (transient)
 *         - LzComposeAlert(guid)   -> an lzCompose attempt failed             (transient)
 *
 *      IMPORTANT: the alerts are NOT terminal. The LayerZero executor automatically
 *      retries failed legs, so a guid that alerts now can still reach SUCCESS moments
 *      later. This can be due to transient out-of-gas error due to race conditions
 *      (which create gaps in the lazy inbound nonce costing more gas) or other issues.
 *      We therefore only treat `ComposeDelivered` as a terminal SUCCESS,
 *      merely count the alerts (as retries), and fall back to the last
 *      alerting leg only for guids that never reach SUCCESS before the deadline
 *      (those are genuine persistent failures, e.g. lzCompose out-of-gas).
 *
 *      `PacketDelivered` (lzReceive success) is intentionally NOT tracked: the bridge's
 *      `_lzReceive` always calls `endpoint.sendCompose(...)`, so a successful lzReceive is
 *      always followed by a compose event (ComposeDelivered or LzComposeAlert).
 *
 *   3. Prints a table summarizing the status of each of the 36 transactions.
 *
 * Usage:
 *   ts-node --transpile-only scripts/stress/handlesListBridgeStress.ts <source>
 *     <source> = sepolia | amoy           (testnet, BRIDGE_ENV=testnet, the default)
 *     <source> = ethereum | polygon       (mainnet, BRIDGE_ENV=mainnet)
 *   (the lzSend originates on <source>; the other chain of the same environment is the destination)
 *
 * The environment is selected by `BRIDGE_ENV` (`testnet` by default, or `mainnet`). The
 * source/destination pair is then resolved within that environment.
 *
 * Required env (.env at repo root): DEPLOYER_PRIVATE_KEY, and the per-chain RPC URLs:
 *   - testnet: SEPOLIA_RPC_URL, POLYGON_AMOY_RPC_URL
 *   - mainnet: ETHEREUM_MAINNET_RPC_URL, POLYGON_MAINNET_RPC_URL
 * The app/bridge addresses are read from the per-chain snapshots:
 *   - testnet: addresses-sepolia/.env.host, addresses-amoy/.env.host
 *   - mainnet: addresses-ethereum/.env.host, addresses-polygon/.env.host
 *
 * Optional env: per-chain EndpointV2 address overrides (SEPOLIA_LZ_ENDPOINT,
 * POLYGON_AMOY_LZ_ENDPOINT, ETHEREUM_LZ_ENDPOINT, POLYGON_LZ_ENDPOINT). Each chain defaults
 * to the correct shared endpoint for its environment (testnet vs mainnet); override only if
 * a specific chain's EndpointV2 address differs.
 */
import * as dotenv from 'dotenv';
import { ethers } from 'ethers';
import { existsSync, readFileSync } from 'fs';
import { resolve } from 'path';

dotenv.config({ path: resolve(__dirname, '..', '..', '.env') });

// Default LayerZero V2 endpoint addresses. The address is NOT guaranteed identical
// across chains but happens to match on Ethereum and Polygon mainnet, and
// is also the same on Sepolia and Amoy but with a different shared value. 
// Each chain picks its environment default and can still override it via
// its `endpointEnvVar` env var (see CHAINS).
const TESTNET_LZ_ENDPOINT = '0x6EDCE65403992e310A62460808c4b910D972f10f';
const MAINNET_LZ_ENDPOINT = '0x1a44076050125825900e736c501f859c50fE728c';

// guid lives at byte offset 81 (len 32) inside the PacketV1 encodedPayload (see
// PacketV1Codec: version[1] nonce[8] srcEid[4] sender[32] dstEid[4] receiver[32] = 81).
const GUID_BYTE_OFFSET = 81;

// Test matrix: every couple of these is exercised (6 x 6 = 36 transactions).
const HANDLE_COUNTS = [1, 2, 4, 8, 16, 32];
const PAYLOAD_LENS = [0, 1, 64, 256, 1024, 8192];

// Destination-side lzCompose gas budget formula. Kept in sync with task:sendHandlesList
// in tasks/examples/handlesList.ts: base + per-handle + per-payload-byte.
const COMPOSE_GAS_BASE = 200_000n;
const COMPOSE_GAS_PER_HANDLE = 100_000n;
const COMPOSE_GAS_PER_PAYLOAD_BYTE = 50n;

// Wait / polling knobs.
const MAX_WAIT_MS = 10 * 60 * 1000; // 10 minutes
const POLL_INTERVAL_MS = 15_000;
const MAX_BLOCK_RANGE = 9_000; // keep eth_getLogs windows under common 10k RPC caps

const ENDPOINT_ABI = [
  'event PacketSent(bytes encodedPayload, bytes options, address sendLibrary)',
  'event LzReceiveAlert(address indexed receiver, address indexed executor, (uint32 srcEid, bytes32 sender, uint64 nonce) origin, bytes32 guid, uint256 gas, uint256 value, bytes message, bytes extraData, bytes reason)',
  'event ComposeDelivered(address from, address to, bytes32 guid, uint16 index)',
  'event LzComposeAlert(address indexed from, address indexed to, address indexed executor, bytes32 guid, uint16 index, uint256 gas, uint256 value, bytes message, bytes extraData, bytes reason)',
];

const HANDLES_APP_ABI = [
  'function peers(uint32 eid) view returns (bytes32)',
  'function confidentialBridge() view returns (address)',
  'function quoteGenerateAndSendHandlesList(uint32 dstEid, uint256 countHandles, bytes customPayload, uint64 lzComposeGas) view returns (tuple(uint256 nativeFee, uint256 lzTokenFee) fee)',
  'function generateAndSendHandlesList(uint32 dstEid, uint256 countHandles, bytes customPayload, uint64 lzComposeGas) payable returns (tuple(bytes32 guid, uint64 nonce, tuple(uint256 nativeFee, uint256 lzTokenFee) fee) receipt)',
];

type BridgeEnv = 'testnet' | 'mainnet';

interface ChainSpec {
  /** Environment this chain belongs to. Source/destination are paired within one env. */
  env: BridgeEnv;
  name: string;
  aliases: string[];
  chainId: number;
  lzEid: number;
  rpcEnvVar: string;
  rpcDefault: string;
  addressesEnv: string;
  // Optional override for the EndpointV2 address on this chain (defaults to
  // `endpointDefault`). Needed if a chain's endpoint address differs.
  endpointEnvVar: string;
  // Environment-appropriate default EndpointV2 address.
  endpointDefault: string;
}

const CHAINS: ChainSpec[] = [
  // ── testnet ──────────────────────────────────────────────────────────────
  {
    env: 'testnet',
    name: 'sepolia',
    aliases: ['sepolia', 'eth', 'ethereum'],
    chainId: 11155111,
    lzEid: 40161,
    rpcEnvVar: 'SEPOLIA_RPC_URL',
    rpcDefault: 'https://sepolia.drpc.org',
    addressesEnv: 'addresses-sepolia/.env.host',
    endpointEnvVar: 'SEPOLIA_LZ_ENDPOINT',
    endpointDefault: TESTNET_LZ_ENDPOINT,
  },
  {
    env: 'testnet',
    name: 'polygonAmoy',
    aliases: ['polygonamoy', 'amoy', 'polygon-amoy', 'polygon'],
    chainId: 80002,
    lzEid: 40267,
    rpcEnvVar: 'POLYGON_AMOY_RPC_URL',
    rpcDefault: 'https://rpc-amoy.polygon.technology',
    addressesEnv: 'addresses-amoy/.env.host',
    endpointEnvVar: 'POLYGON_AMOY_LZ_ENDPOINT',
    endpointDefault: TESTNET_LZ_ENDPOINT,
  },
  // ── mainnet ──────────────────────────────────────────────────────────────
  {
    env: 'mainnet',
    name: 'ethereum',
    aliases: ['ethereum', 'eth', 'mainnet', 'ethereum-mainnet'],
    chainId: 1,
    lzEid: 30101,
    rpcEnvVar: 'ETHEREUM_MAINNET_RPC_URL',
    rpcDefault: 'https://eth.llamarpc.com',
    addressesEnv: 'addresses-ethereum/.env.host',
    endpointEnvVar: 'ETHEREUM_LZ_ENDPOINT',
    endpointDefault: MAINNET_LZ_ENDPOINT,
  },
  {
    env: 'mainnet',
    name: 'polygon',
    aliases: ['polygon', 'matic', 'polygon-mainnet'],
    chainId: 137,
    lzEid: 30109,
    rpcEnvVar: 'POLYGON_MAINNET_RPC_URL',
    rpcDefault: 'https://polygon-rpc.com',
    addressesEnv: 'addresses-polygon/.env.host',
    endpointEnvVar: 'POLYGON_LZ_ENDPOINT',
    endpointDefault: MAINNET_LZ_ENDPOINT,
  },
];

/** Resolve the active bridge environment from `BRIDGE_ENV` (default `testnet`). */
function activeBridgeEnv(): BridgeEnv {
  const raw = (process.env.BRIDGE_ENV ?? 'testnet').trim().toLowerCase();
  if (raw === 'mainnet' || raw === 'main' || raw === 'prod') return 'mainnet';
  if (raw === 'testnet' || raw === 'test' || raw === '') return 'testnet';
  throw new Error(`Invalid BRIDGE_ENV="${process.env.BRIDGE_ENV}". Use "testnet" (default) or "mainnet".`);
}

type Status =
  | 'PENDING'
  | 'SUCCESS'
  | 'RECEIVE_FAILED'
  | 'COMPOSE_FAILED'
  | 'SEND_FAILED'
  | 'TIMEOUT'
  | 'NO_PACKET_SENT';

interface SendRecord {
  index: number;
  count: number;
  payloadLen: number;
  composeGas: bigint;
  nativeFee?: bigint;
  txHash?: string;
  guid?: string;
  status: Status;
  detail?: string;
  // Alerts are *transient*: the LayerZero executor retries failed legs, so an alert is
  // only a "last-known failure", not a terminal state. We count them for visibility and
  // fall back to the most recent one as the final status if a guid never reaches SUCCESS.
  receiveAlerts: number;
  composeAlerts: number;
  lastAlert?: 'RECEIVE_FAILED' | 'COMPOSE_FAILED';
}

function parseEnvFile(path: string): Record<string, string> {
  if (!existsSync(path)) {
    throw new Error(
      `Addresses snapshot not found at ${path}. Deploy + snapshot this chain first (see runbook Step 7).`,
    );
  }
  const env: Record<string, string> = {};
  for (const raw of readFileSync(path, 'utf8').split('\n')) {
    const line = raw.trim();
    if (line === '' || line.startsWith('#')) continue;
    const eq = line.indexOf('=');
    if (eq < 0) continue;
    const key = line.slice(0, eq).trim();
    let val = line.slice(eq + 1).trim();
    if ((val.startsWith('"') && val.endsWith('"')) || (val.startsWith("'") && val.endsWith("'"))) {
      val = val.slice(1, -1);
    }
    env[key] = val;
  }
  return env;
}

function resolveChain(arg: string): ChainSpec {
  const env = activeBridgeEnv();
  const needle = arg.trim().toLowerCase();
  const candidates = CHAINS.filter((c) => c.env === env);
  const spec = candidates.find((c) => c.name.toLowerCase() === needle || c.aliases.includes(needle));
  if (!spec) {
    throw new Error(
      `Unknown chain "${arg}" for BRIDGE_ENV=${env}. ` +
        `Valid sources: ${candidates.map((c) => c.name).join(', ')} (or aliases).`,
    );
  }
  return spec;
}

interface ChainRuntime {
  spec: ChainSpec;
  provider: ethers.JsonRpcProvider;
  appAddress: string;
  bridgeAddress: string;
  endpointAddress: string;
}

function loadChainRuntime(spec: ChainSpec): ChainRuntime {
  const env = parseEnvFile(resolve(__dirname, '..', '..', spec.addressesEnv));
  const appAddress = env.HANDLES_LIST_OAPP_CONTRACT_ADDRESS;
  const bridgeAddress = env.CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS;
  if (!appAddress) {
    throw new Error(`${spec.addressesEnv} is missing HANDLES_LIST_OAPP_CONTRACT_ADDRESS (deploy it on ${spec.name}).`);
  }
  if (!bridgeAddress) {
    throw new Error(`${spec.addressesEnv} is missing CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS on ${spec.name}.`);
  }
  const endpointAddress = (process.env[spec.endpointEnvVar] ?? spec.endpointDefault).trim();
  if (!ethers.isAddress(endpointAddress)) {
    throw new Error(`${spec.endpointEnvVar}="${endpointAddress}" is not a valid EndpointV2 address on ${spec.name}.`);
  }
  const rpcUrl = (process.env[spec.rpcEnvVar] ?? spec.rpcDefault).trim();
  const provider = new ethers.JsonRpcProvider(rpcUrl, spec.chainId);
  return { spec, provider, appAddress, bridgeAddress, endpointAddress };
}

function guidFromEncodedPayload(encodedPayload: string): string {
  const hex = encodedPayload.startsWith('0x') ? encodedPayload.slice(2) : encodedPayload;
  const start = GUID_BYTE_OFFSET * 2;
  const guid = hex.slice(start, start + 64);
  if (guid.length !== 64) {
    throw new Error(`encodedPayload too short to contain a guid (len ${hex.length / 2} bytes)`);
  }
  return ('0x' + guid).toLowerCase();
}

function composeGasFor(count: number, payloadLen: number): bigint {
  return COMPOSE_GAS_BASE + COMPOSE_GAS_PER_HANDLE * BigInt(count) + COMPOSE_GAS_PER_PAYLOAD_BYTE * BigInt(payloadLen);
}

function buildPayload(payloadLen: number): string {
  return payloadLen === 0 ? '0x' : '0x' + 'ff'.repeat(payloadLen);
}

function shortGuid(guid?: string): string {
  if (!guid) return '-';
  return `${guid.slice(0, 10)}…${guid.slice(-6)}`;
}

const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

/** Query a single event filter across [fromBlock, toBlock] in <=maxRange windows. */
async function chunkedQueryFilter(
  contract: ethers.Contract,
  filter: ethers.DeferredTopicFilter,
  fromBlock: number,
  toBlock: number,
): Promise<ethers.EventLog[]> {
  const out: ethers.EventLog[] = [];
  let from = fromBlock;
  while (from <= toBlock) {
    const to = Math.min(toBlock, from + MAX_BLOCK_RANGE - 1);
    const logs = await contract.queryFilter(filter, from, to);
    for (const log of logs) {
      if ('args' in log) out.push(log as ethers.EventLog);
    }
    from = to + 1;
  }
  return out;
}

async function main(): Promise<void> {
  const sourceArg = process.argv[2] ?? process.env.STRESS_SOURCE ?? '';
  if (!sourceArg || sourceArg === '--help' || sourceArg === '-h') {
    console.log(
      'Usage: ts-node --transpile-only scripts/stress/handlesListBridgeStress.ts <source>\n' +
        '  BRIDGE_ENV=testnet (default): <source> = sepolia | amoy\n' +
        '  BRIDGE_ENV=mainnet:          <source> = ethereum | polygon',
    );
    process.exit(sourceArg ? 0 : 2);
  }

  const privateKey = process.env.DEPLOYER_PRIVATE_KEY;
  if (!privateKey) throw new Error('DEPLOYER_PRIVATE_KEY is not set in .env');

  const srcSpec = resolveChain(sourceArg);
  const dstSpec = CHAINS.find((c) => c.env === srcSpec.env && c.name !== srcSpec.name);
  if (!dstSpec) throw new Error('Could not resolve a distinct destination chain.');

  const src = loadChainRuntime(srcSpec);
  const dst = loadChainRuntime(dstSpec);

  const wallet = new ethers.Wallet(privateKey, src.provider);
  const srcApp = new ethers.Contract(src.appAddress, HANDLES_APP_ABI, wallet);
  const endpointIface = new ethers.Interface(ENDPOINT_ABI);
  const dstEndpoint = new ethers.Contract(dst.endpointAddress, ENDPOINT_ABI, dst.provider);

  console.log(`Stress test: ${srcSpec.name} (eid ${srcSpec.lzEid}) -> ${dstSpec.name} (eid ${dstSpec.lzEid})`);
  console.log(`  source app:      ${src.appAddress} (endpoint ${src.endpointAddress})`);
  console.log(`  destination app: ${dst.appAddress} (bridge ${dst.bridgeAddress}, endpoint ${dst.endpointAddress})`);
  console.log(`  sender:          ${wallet.address}`);

  // Verify the source app is wired to the destination app before spending fees.
  const configuredPeer: string = await srcApp.peers(dstSpec.lzEid);
  const expectedPeer = ethers.zeroPadValue(dst.appAddress, 32).toLowerCase();
  if (configuredPeer.toLowerCase() !== expectedPeer) {
    throw new Error(
      `Source app peer for eid ${dstSpec.lzEid} is ${configuredPeer}, expected ${expectedPeer}. ` +
        `Run task:wireHandlesListConfidentialOApp on ${srcSpec.name} first.`,
    );
  }

  // Snapshot the destination head BEFORE sending; lzReceive/lzCompose can only land later.
  const dstStartBlock = await dst.provider.getBlockNumber();
  console.log(`  destination scan starts at block ${dstStartBlock}`);

  // ---- Phase 1: fire all 36 sends from the source chain ----
  const records: SendRecord[] = [];
  let index = 0;
  for (const count of HANDLE_COUNTS) {
    for (const payloadLen of PAYLOAD_LENS) {
      records.push({
        index: index++,
        count,
        payloadLen,
        composeGas: composeGasFor(count, payloadLen),
        status: 'PENDING',
        receiveAlerts: 0,
        composeAlerts: 0,
      });
    }
  }

  console.log(`\nSending ${records.length} bridging transactions from ${srcSpec.name}...`);
  let nonce = await wallet.getNonce();
  const txByIndex = new Map<number, ethers.TransactionResponse>();

  for (const rec of records) {
    const payload = buildPayload(rec.payloadLen);
    try {
      const fee = await srcApp.quoteGenerateAndSendHandlesList(dstSpec.lzEid, rec.count, payload, rec.composeGas);
      rec.nativeFee = fee.nativeFee as bigint;
      const tx: ethers.TransactionResponse = await srcApp.generateAndSendHandlesList(
        dstSpec.lzEid,
        rec.count,
        payload,
        rec.composeGas,
        { value: rec.nativeFee, nonce },
      );
      rec.txHash = tx.hash;
      txByIndex.set(rec.index, tx);
      nonce++;
      console.log(
        `  [${rec.index + 1}/${records.length}] count=${rec.count} payloadLen=${rec.payloadLen} ` +
          `composeGas=${rec.composeGas} fee=${rec.nativeFee} wei tx=${tx.hash}`,
      );
    } catch (err) {
      rec.status = 'SEND_FAILED';
      rec.detail = err instanceof Error ? err.message.split('\n')[0] : String(err);
      console.error(`  [${rec.index + 1}/${records.length}] SEND FAILED: ${rec.detail}`);
    }
  }

  // Wait for inclusion and pull the guid out of each PacketSent event.
  console.log('\nWaiting for source-chain inclusion and extracting guids...');
  const guidToRecord = new Map<string, SendRecord>();
  for (const rec of records) {
    const tx = txByIndex.get(rec.index);
    if (!tx) continue;
    try {
      const receipt = await tx.wait();
      const packetLog = receipt?.logs.find(
        (l) =>
          l.address.toLowerCase() === src.endpointAddress.toLowerCase() &&
          l.topics[0] === endpointIface.getEvent('PacketSent')!.topicHash,
      );
      if (!packetLog) {
        rec.status = 'NO_PACKET_SENT';
        rec.detail = 'No PacketSent event found in receipt';
        continue;
      }
      const parsed = endpointIface.parseLog({ topics: [...packetLog.topics], data: packetLog.data });
      const guid = guidFromEncodedPayload(parsed!.args.encodedPayload as string);
      rec.guid = guid;
      guidToRecord.set(guid, rec);
    } catch (err) {
      rec.status = 'SEND_FAILED';
      rec.detail = err instanceof Error ? err.message.split('\n')[0] : String(err);
    }
  }

  const pending = new Set<string>(guidToRecord.keys());
  console.log(`\n${pending.size}/${records.length} transactions sent successfully; waiting for destination delivery.`);
  if (pending.size === 0) {
    printTable(records);
    throw new Error('No transactions were sent successfully; nothing to track.');
  }

  // ---- Phase 2: poll the destination endpoint for terminal events ----
  const recvAlert = dstEndpoint.filters.LzReceiveAlert(dst.bridgeAddress);
  const composeAlert = dstEndpoint.filters.LzComposeAlert(dst.bridgeAddress, dst.bridgeAddress);
  const composeDelivered = dstEndpoint.filters.ComposeDelivered();

  const deadline = Date.now() + MAX_WAIT_MS;
  let scanFrom = dstStartBlock;

  // SUCCESS (ComposeDelivered) is the ONLY terminal state we act on while polling.
  // ComposeDelivered implies lzReceive succeeded too (the bridge always sendCompose's
  // from a successful _lzReceive), so it proves both legs landed.
  const markSuccess = (guid: string): void => {
    const rec = guidToRecord.get(guid.toLowerCase());
    if (rec && pending.has(rec.guid!)) {
      rec.status = 'SUCCESS';
      pending.delete(rec.guid!);
    }
  };

  // Alerts do NOT remove a guid from `pending`: the executor retries failed legs, so a
  // guid that alerts now may still reach SUCCESS shortly. We just count them for visibility.
  const noteAlert = (guid: string, kind: 'RECEIVE_FAILED' | 'COMPOSE_FAILED'): void => {
    const rec = guidToRecord.get(guid.toLowerCase());
    if (!rec || !pending.has(rec.guid!)) return;
    if (kind === 'RECEIVE_FAILED') rec.receiveAlerts++;
    else rec.composeAlerts++;
    rec.lastAlert = kind;
    const leg = kind === 'RECEIVE_FAILED' ? 'lzReceive' : 'lzCompose';
    console.log(`    alert #${rec.index + 1} ${leg} guid=${shortGuid(rec.guid)} (will retry)`);
  };

  while (pending.size > 0) {
    const head = await dst.provider.getBlockNumber();
    if (head >= scanFrom) {
      const [alerts, calerts, delivered] = await Promise.all([
        chunkedQueryFilter(dstEndpoint, recvAlert, scanFrom, head),
        chunkedQueryFilter(dstEndpoint, composeAlert, scanFrom, head),
        chunkedQueryFilter(dstEndpoint, composeDelivered, scanFrom, head),
      ]);
      // Note alerts first, then apply successes so that a guid which both alerted and
      // ultimately delivered within the same window ends up SUCCESS.
      for (const ev of alerts) noteAlert(ev.args.guid as string, 'RECEIVE_FAILED');
      for (const ev of calerts) noteAlert(ev.args.guid as string, 'COMPOSE_FAILED');
      for (const ev of delivered) markSuccess(ev.args.guid as string);
      scanFrom = head + 1;
    }

    if (pending.size === 0) break;

    const remainingMs = deadline - Date.now();
    if (remainingMs <= 0) {
      // Finalize whatever never reached SUCCESS. A guid that kept alerting until the
      // deadline is a *persistent* failure (e.g. lzCompose OOG); one with no alert at
      // all is simply still in flight (TIMEOUT).
      for (const guid of pending) {
        const rec = guidToRecord.get(guid);
        if (rec) rec.status = rec.lastAlert ?? 'TIMEOUT';
      }
      printTable(records);
      throw new Error(
        `Timed out after ${MAX_WAIT_MS / 60000} minutes with ${pending.size}/${records.length} deliveries not confirmed SUCCESS.`,
      );
    }

    const alerted = [...pending].filter((g) => {
      const r = guidToRecord.get(g);
      return r && (r.receiveAlerts > 0 || r.composeAlerts > 0);
    }).length;
    console.log(
      `  SUCCESS ${records.length - pending.size}/${guidToRecord.size}; ${pending.size} pending ` +
        `(${alerted} currently alerted/retrying); ${Math.ceil(remainingMs / 1000)}s left — re-checking in ${POLL_INTERVAL_MS / 1000}s`,
    );
    await sleep(Math.min(POLL_INTERVAL_MS, remainingMs));
  }

  console.log('\nAll deliveries confirmed SUCCESS.');
  printTable(records);
}

/** Render an aligned ASCII table without console.table's leading "(index)" column. */
function renderTable(rows: Record<string, string>[], columns: string[]): string {
  const widths = columns.map((c) => Math.max(c.length, ...rows.map((row) => (row[c] ?? '').length)));
  const sep = '+' + widths.map((w) => '-'.repeat(w + 2)).join('+') + '+';
  const fmt = (cells: string[]) => '| ' + cells.map((cell, i) => cell.padEnd(widths[i])).join(' | ') + ' |';
  const lines = [sep, fmt(columns), sep, ...rows.map((row) => fmt(columns.map((c) => row[c] ?? ''))), sep];
  return lines.join('\n');
}

function printTable(records: SendRecord[]): void {
  const rows = records.map((r) => ({
    'Tx#': String(r.index + 1),
    count: String(r.count),
    payloadLen: String(r.payloadLen),
    composeGas: r.composeGas.toString(),
    fee_wei: r.nativeFee !== undefined ? r.nativeFee.toString() : '-',
    // Transient executor retries before the leg ultimately settled (r = lzReceive
    // alerts, c = lzCompose alerts). Non-zero with status SUCCESS == raced then retried.
    alerts: `${r.receiveAlerts}r/${r.composeAlerts}c`,
    guid: shortGuid(r.guid),
    status: r.status,
  }));
  const columns = ['Tx#', 'count', 'payloadLen', 'composeGas', 'fee_wei', 'alerts', 'guid', 'status'];
  console.log('\n=== HandlesListConfidentialOApp bridge stress results ===');
  console.log(renderTable(rows, columns));

  const summary: Record<string, number> = {};
  for (const r of records) summary[r.status] = (summary[r.status] ?? 0) + 1;
  console.log('Summary:', summary);
}

main().catch((err) => {
  console.error('\n[stress] fatal:', err instanceof Error ? err.message : err);
  process.exit(1);
});
