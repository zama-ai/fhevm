require('dotenv').config({ quiet: true })
const { isAddress, Interface, JsonRpcProvider, getAddress, formatEther } = require('ethers')

const SCRIPT_NAME = 'aragonProposalInspector.js'
const RPC_ENV_VAR = 'RPC_ETHEREUM'
const ETHERSCAN_API_KEY_ENV_VAR = 'ETHERSCAN_API_KEY'
const ETHERSCAN_V2_URL = 'https://api.etherscan.io/v2/api'

// Etherscan v2 free-tier rate limit is around 3 req/s. All Etherscan requests in this
// script go through a single global queue that enforces a minimum gap between
// consecutive requests, and rate-limit errors are retried with backoff.
const ETHERSCAN_MIN_GAP_MS = 400 // -> ~2.5 req/s, safely under 3 req/s
const ETHERSCAN_MAX_RETRIES = 2
const ETHERSCAN_RETRY_DELAY_MS = 1500

// Minimal ABI of the Aragon OSx Multisig plugin. We only need the read-only
// functions used to inspect a proposal; we deliberately avoid pulling in any
// off-chain Aragon source (subgraph, hosted API) so the only trust roots for
// the proposal content itself are the RPC endpoint, ethers, and this ABI
// fragment.
//
// Optional Etherscan enrichment (contract names + abi-decoded calldata) is
// strictly additive: the raw (to, value, data) is still printed first
const MULTISIG_ABI = [
  'function getProposal(uint256 _proposalId) view returns (bool executed, uint16 approvals, tuple(uint16 minApprovals, uint64 snapshotBlock, uint64 startDate, uint64 endDate) parameters, tuple(address to, uint256 value, bytes data)[] actions, uint256 allowFailureMap)',
  'function canExecute(uint256 _proposalId) view returns (bool)',
]
const MULTISIG_IFACE = new Interface(MULTISIG_ABI)

function printUsage() {
  console.log(
    `Usage:
  npm run aragon-proposal-inspector -- --plugin <addr> --id <num> [--rpc <url>] [--json]
  node ${SCRIPT_NAME} --plugin <addr> --id <num> [--rpc <url>] [--json]

Flags:
  --plugin   REQUIRED  Address of the Aragon OSx Multisig plugin holding the proposal.
  --id       REQUIRED  Proposal id (decimal or 0x-hex non-negative integer).
  --rpc      OPTIONAL  Ethereum RPC URL. Default: env var ${RPC_ENV_VAR}.
  --json     OPTIONAL  Print machine-readable JSON instead of the human-readable summary.
  -h, --help           Show this help.

Fetches a proposal from an Aragon OSx Multisig plugin via direct eth_call
and prints its on-chain content. Useful as an independent sanity check
before voting, in case the Aragon front-end is compromised.

Trust path for the proposal itself: the chosen RPC endpoint, and ethers js library.
No Aragon subgraph or Aragon-hosted API is consulted.

Optional Etherscan enrichment: when ${ETHERSCAN_API_KEY_ENV_VAR} is set in
the env, every action's "to" address is looked up via the Etherscan v2 API
to print the contract name and abi-decode "data" with that contract's ABI
(falling back to the implementation's ABI for proxies). This is purely
additive — the raw data is still printed and you may ignore the
enrichment if you do not want to trust Etherscan.

Required env (unless --rpc is given):
  - ${RPC_ENV_VAR}

Optional env:
  - ${ETHERSCAN_API_KEY_ENV_VAR}  (enables contract-name + calldata decoding)
`
  )
}

function parseArgs(argv) {
  const args = { plugin: undefined, id: undefined, rpc: undefined, json: false }

  for (let i = 2; i < argv.length; i++) {
    const flag = argv[i]
    switch (flag) {
      case '-h':
      case '--help':
        printUsage()
        process.exit(0)
      case '--plugin': {
        const value = argv[++i]
        if (!value) throw new Error(`Missing value for ${flag}`)
        args.plugin = value
        break
      }
      case '--id': {
        const value = argv[++i]
        if (!value) throw new Error(`Missing value for ${flag}`)
        args.id = value
        break
      }
      case '--rpc': {
        const value = argv[++i]
        if (!value) throw new Error(`Missing value for ${flag}`)
        args.rpc = value
        break
      }
      case '--json':
        args.json = true
        break
      default:
        throw new Error(`Unknown flag: ${flag}`)
    }
  }

  if (!args.plugin) throw new Error('Missing required flag: --plugin <address>')
  if (args.id === undefined) throw new Error('Missing required flag: --id <number>')

  return args
}

function parseProposalId(raw) {
  let id
  try {
    // BigInt accepts both decimal ("5") and 0x-hex ("0x5") strings.
    id = BigInt(raw)
  } catch {
    throw new Error(
      `--id must be a decimal or 0x-hex non-negative integer (got "${raw}")`
    )
  }
  if (id < 0n) throw new Error(`--id must be non-negative (got ${id.toString()})`)
  return id
}

function formatTimestamp(seconds) {
  if (seconds === 0n) return '0 (unset)'
  const millis = Number(seconds) * 1000
  return `${seconds.toString()} (${new Date(millis).toISOString()})`
}

// ---------------------------------------------------------------------------
// Etherscan v2 contract metadata + ABI fetcher.
//
// Returns one of:
//   { kind: 'verified',   name, abi, isProxy, implementation }
//   { kind: 'unverified', name, isProxy, implementation }   // implementation may also be set when Etherscan flags the contract as a proxy without verifying it
//   { kind: 'error',      message }
//
// Caches the resolved value (success or error) per address for the duration
// of one run, since proposals often re-target the same contract.
// ---------------------------------------------------------------------------

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

function isRateLimitMessage(msg) {
  return typeof msg === 'string' && /rate limit|too many|throttl/i.test(msg)
}

function makeEtherscanClient(chainId, apiKey) {
  const cache = new Map() // checksummed-address -> Promise<info>

  // Single global queue of pending Etherscan calls, paced by ETHERSCAN_MIN_GAP_MS.
  // We chain off the previous request's completion AND off the timestamp of the
  // previous *start*, so even fast responses cannot slip past the rate limit.
  let queueTail = Promise.resolve()
  let lastRequestStartedAt = 0

  function schedule(fn) {
    const next = queueTail.then(async () => {
      const waitMs = lastRequestStartedAt + ETHERSCAN_MIN_GAP_MS - Date.now()
      if (waitMs > 0) await sleep(waitMs)
      lastRequestStartedAt = Date.now()
      return fn()
    })
    // Don't let a rejected fn() poison the queue for subsequent callers.
    queueTail = next.catch(() => {})
    return next
  }

  async function rawFetch(address) {
    const url = new URL(ETHERSCAN_V2_URL)
    url.searchParams.set('chainid', chainId.toString())
    url.searchParams.set('module', 'contract')
    url.searchParams.set('action', 'getsourcecode')
    url.searchParams.set('address', address)
    url.searchParams.set('apikey', apiKey)

    let resp
    try {
      resp = await fetch(url, { method: 'GET' })
    } catch (err) {
      return { kind: 'error', message: `network error: ${err.message}` }
    }
    if (!resp.ok) {
      return { kind: 'error', message: `HTTP ${resp.status}` }
    }
    let body
    try {
      body = await resp.json()
    } catch (err) {
      return { kind: 'error', message: `invalid JSON: ${err.message}` }
    }

    // status="0" with a string result is the typical error envelope
    // (rate limit, invalid key, unsupported chain, ...).
    if (body.status === '0' && typeof body.result === 'string') {
      return { kind: 'error', message: body.result }
    }
    if (!Array.isArray(body.result) || body.result.length === 0) {
      return { kind: 'error', message: 'empty result' }
    }
    const r = body.result[0]
    const isProxy = r.Proxy === '1'
    const implementation =
      r.Implementation && /^0x[0-9a-fA-F]{40}$/.test(r.Implementation)
        ? getAddress(r.Implementation)
        : null

    const verified =
      typeof r.ABI === 'string' &&
      r.ABI !== '' &&
      r.ABI !== 'Contract source code not verified'

    if (!verified) {
      return {
        kind: 'unverified',
        name: r.ContractName || null,
        isProxy,
        implementation,
      }
    }

    let abi
    try {
      abi = JSON.parse(r.ABI)
    } catch (err) {
      return { kind: 'error', message: `failed to parse ABI JSON: ${err.message}` }
    }
    return {
      kind: 'verified',
      name: r.ContractName || null,
      abi,
      isProxy,
      implementation,
    }
  }

  async function fetchWithRetry(address) {
    for (let attempt = 0; attempt <= ETHERSCAN_MAX_RETRIES; attempt++) {
      const result = await schedule(() => rawFetch(address))
      if (result.kind !== 'error') return result
      if (!isRateLimitMessage(result.message)) return result
      if (attempt === ETHERSCAN_MAX_RETRIES) return result
      await sleep(ETHERSCAN_RETRY_DELAY_MS)
    }
  }

  return async function get(address) {
    const key = getAddress(address) // checksummed
    if (!cache.has(key)) cache.set(key, fetchWithRetry(key))
    return cache.get(key)
  }
}

// ---------------------------------------------------------------------------
// ABI-based calldata decoder.
//
// Returns one of:
//   { kind: 'decoded',  signature, args }      // args = [{ name, type, value }]
//   { kind: 'no-match', selector }              // ABI doesn't contain that selector
//   { kind: 'error',    message }
// ---------------------------------------------------------------------------

function tryDecodeCalldata(abi, data) {
  if (typeof data !== 'string' || !data.startsWith('0x') || data.length < 10) {
    return { kind: 'error', message: 'calldata is shorter than a 4-byte selector' }
  }
  let iface
  try {
    iface = new Interface(abi)
  } catch (err) {
    return { kind: 'error', message: `Interface construction failed: ${err.message}` }
  }
  let parsed
  try {
    parsed = iface.parseTransaction({ data })
  } catch (err) {
    return {
      kind: 'error',
      message: `parseTransaction failed: ${err.shortMessage || err.message}`,
    }
  }
  if (!parsed) {
    return { kind: 'no-match', selector: data.slice(0, 10) }
  }
  return {
    kind: 'decoded',
    signature: parsed.signature,
    args: parsed.fragment.inputs.map((input, i) => ({
      name: input.name || `arg${i}`,
      type: input.format(),
      value: jsonifyValue(parsed.args[i], input),
    })),
  }
}

// Recursively converts an ethers decoded value into something JSON-stringifiable
// and human-friendly: BigInts -> decimal strings, tuples -> plain objects keyed
// by the component names from the ABI, arrays -> arrays of converted children.
function jsonifyValue(value, paramType) {
  if (typeof value === 'bigint') return value.toString()
  if (paramType.baseType === 'array') {
    return value.map((item) => jsonifyValue(item, paramType.arrayChildren))
  }
  if (paramType.baseType === 'tuple') {
    const obj = {}
    paramType.components.forEach((comp, i) => {
      obj[comp.name || `field${i}`] = jsonifyValue(value[i], comp)
    })
    return obj
  }
  return value
}

// ---------------------------------------------------------------------------
// Per-action enrichment: contract name + decoded calldata, with proxy
// fallback. Errors are non-fatal and reported inline.
// ---------------------------------------------------------------------------

async function enrichAction(action, etherscan) {
  const info = await etherscan(action.to)
  if (info.kind === 'error') {
    return { etherscanError: info.message }
  }

  const result = {
    contractName: info.name,
    verified: info.kind === 'verified',
    isProxy: info.isProxy,
    implementation: info.implementation,
  }

  // Try the contract's own ABI first.
  if (info.kind === 'verified') {
    const r = tryDecodeCalldata(info.abi, action.data)
    if (r.kind === 'decoded') {
      result.decoded = r
      return result
    }
    result.directDecode = r // 'no-match' or 'error'
  } else {
    result.directDecode = { kind: 'unverified' }
  }

  // If proxy, try the implementation's ABI.
  if (info.implementation) {
    const implInfo = await etherscan(info.implementation)
    result.implementationVerified = implInfo.kind === 'verified'
    result.implementationName =
      implInfo.kind === 'verified' || implInfo.kind === 'unverified' ? implInfo.name : null

    if (implInfo.kind === 'verified') {
      const r = tryDecodeCalldata(implInfo.abi, action.data)
      if (r.kind === 'decoded') {
        result.decoded = r
        result.decodedVia = 'implementation'
        return result
      }
      result.implementationDecode = r
    } else if (implInfo.kind === 'error') {
      result.implementationError = implInfo.message
    }
  }

  return result
}

// ---------------------------------------------------------------------------
// Output helpers
// ---------------------------------------------------------------------------

function describeAction(action, index) {
  return {
    index,
    to: action.to,
    value: action.value.toString(),
    data: action.data,
  }
}

function formatNameLine(enrichment) {
  if (!enrichment) return null
  if (enrichment.etherscanError) {
    return `<Etherscan unavailable: ${enrichment.etherscanError}>`
  }
  const own = enrichment.contractName || '<unnamed>'
  const ownStatus = enrichment.verified ? 'verified' : 'unverified'
  if (!enrichment.isProxy || !enrichment.implementation) {
    return `${own} (${ownStatus})`
  }
  const implName = enrichment.implementationName || '<unnamed>'
  const implStatus =
    enrichment.implementationVerified === true
      ? 'verified'
      : enrichment.implementationVerified === false
        ? 'unverified'
        : `error: ${enrichment.implementationError || 'unknown'}`
  return `${own} (${ownStatus}) -> proxy to ${implName} at ${enrichment.implementation} (${implStatus})`
}

// Renders the decoded function + args under an action. Scalars are inlined;
// complex types (tuples / arrays of tuples) are shown as indented JSON.
function printDecoded(enrichment) {
  if (!enrichment) return
  if (enrichment.etherscanError) return // already surfaced via "name:"

  if (!enrichment.decoded) {
    let reason
    if (enrichment.directDecode?.kind === 'unverified') {
      reason = 'contract is not verified on Etherscan'
    } else if (enrichment.directDecode?.kind === 'no-match') {
      reason = `no function with selector ${enrichment.directDecode.selector} in ABI`
      if (enrichment.implementationDecode?.kind === 'no-match') {
        reason += ` (also not in implementation ABI)`
      } else if (enrichment.implementationError) {
        reason += ` (implementation lookup failed: ${enrichment.implementationError})`
      }
    } else if (enrichment.directDecode?.kind === 'error') {
      reason = enrichment.directDecode.message
    } else {
      reason = 'unknown'
    }
    console.log(`      function: <not decoded: ${reason}>`)
    return
  }

  const via = enrichment.decodedVia === 'implementation' ? ' (via implementation ABI)' : ''
  console.log(`      function: ${enrichment.decoded.signature}${via}`)
  for (const arg of enrichment.decoded.args) {
    const label = `        ${arg.name} (${arg.type}):`
    if (typeof arg.value === 'object' && arg.value !== null) {
      console.log(label)
      const json = JSON.stringify(arg.value, null, 2)
      for (const line of json.split('\n')) console.log(`          ${line}`)
    } else {
      console.log(`${label} ${arg.value}`)
    }
  }
}

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

async function main() {
  let parsed
  try {
    parsed = parseArgs(process.argv)
  } catch (err) {
    console.error(`Error: ${err.message}\n`)
    printUsage()
    process.exit(1)
  }

  if (!isAddress(parsed.plugin)) {
    console.error(
      `Error: --plugin (${parsed.plugin}) is not a well-formed 0x-prefixed 20-byte hex address.`
    )
    process.exit(1)
  }

  let proposalId
  try {
    proposalId = parseProposalId(parsed.id)
  } catch (err) {
    console.error(`Error: ${err.message}`)
    process.exit(1)
  }

  const rpcUrl = parsed.rpc || process.env[RPC_ENV_VAR]
  if (!rpcUrl || rpcUrl.trim() === '') {
    console.error(
      `Error: missing --rpc and env var ${RPC_ENV_VAR}. Set one of them (see .env.example).`
    )
    process.exit(1)
  }

  const provider = new JsonRpcProvider(rpcUrl)

  let net, blockNumber
  try {
    ;[net, blockNumber] = await Promise.all([
      provider.getNetwork(),
      provider.getBlockNumber(),
    ])
  } catch (err) {
    console.error(
      `Error: failed to connect to RPC ${rpcUrl}: ${err.shortMessage || err.message}`
    )
    process.exit(1)
  }

  const code = await provider.getCode(parsed.plugin)
  if (code === '0x') {
    console.error(
      `Error: plugin (${parsed.plugin}) has no bytecode on chainId ${net.chainId.toString()}.`
    )
    process.exit(1)
  }

  let decoded
  try {
    const callData = MULTISIG_IFACE.encodeFunctionData('getProposal', [proposalId])
    const ret = await provider.call({ to: parsed.plugin, data: callData })
    decoded = MULTISIG_IFACE.decodeFunctionResult('getProposal', ret)
  } catch (err) {
    console.error(
      `Error: getProposal(${proposalId.toString()}) call reverted on ${parsed.plugin}: ${err.shortMessage || err.message}`
    )
    console.error(
      '(Most likely the proposal id does not exist on this plugin, or the contract is not an Aragon OSx Multisig plugin.)'
    )
    process.exit(1)
  }

  const [executed, approvals, parameters, actions, allowFailureMap] = decoded

  if(allowFailureMap!==0n){
    console.error('Error: allowFailureMap is not 0, this is probably not a valid regular proposal made via the Aragon UI')
    process.exit(1)
  }

  // Best-effort: ask the plugin whether the proposal is currently executable.
  // Any revert here (e.g. plugin variant without canExecute) is reported as
  // "unknown" rather than fatal — the proposal content is what matters.
  let canExecute = null
  let canExecuteError = null
  try {
    const callData = MULTISIG_IFACE.encodeFunctionData('canExecute', [proposalId])
    const ret = await provider.call({ to: parsed.plugin, data: callData })
    ;[canExecute] = MULTISIG_IFACE.decodeFunctionResult('canExecute', ret)
  } catch (err) {
    canExecuteError = err.shortMessage || err.message
  }

  const describedActions = actions.map((a, i) => describeAction(a, i))

  // Etherscan enrichment is opt-in via env. When the key is missing we just
  // skip it and print actions exactly like before.
  const etherscanApiKey = process.env[ETHERSCAN_API_KEY_ENV_VAR]
  let enrichments = describedActions.map(() => null)
  let etherscanEnabled = false
  if (etherscanApiKey && etherscanApiKey.trim() !== '') {
    etherscanEnabled = true
    const etherscan = makeEtherscanClient(net.chainId, etherscanApiKey.trim())
    // Per-action enrichment runs sequentially; pacing + retry on rate-limit
    // errors is handled inside makeEtherscanClient.
    for (let i = 0; i < describedActions.length; i++) {
      enrichments[i] = await enrichAction(describedActions[i], etherscan)
    }
  }

  if (parsed.json) {
    const out = {
      plugin: parsed.plugin,
      chainId: net.chainId.toString(),
      blockNumber,
      proposalId: proposalId.toString(),
      executed,
      approvals: Number(approvals),
      minApprovals: Number(parameters.minApprovals),
      snapshotBlock: parameters.snapshotBlock.toString(),
      startDate: parameters.startDate.toString(),
      endDate: parameters.endDate.toString(),
      canExecute,
      canExecuteError,
      etherscanEnabled,
      actions: describedActions.map((a, i) => ({ ...a, enrichment: enrichments[i] })),
    }
    console.log(JSON.stringify(out, null, 2))
    return
  }

  const now = BigInt(Math.floor(Date.now() / 1000))
  const startSec = parameters.startDate
  const endSec = parameters.endDate
  let windowStatus
  if (startSec > now) {
    windowStatus = `not yet open (starts ${formatTimestamp(startSec)})`
  } else if (endSec === 0n) {
    windowStatus = 'open (no end date)'
  } else if (endSec > now) {
    windowStatus = `open (ends ${formatTimestamp(endSec)})`
  } else {
    windowStatus = `closed (ended ${formatTimestamp(endSec)})`
  }

  console.log(`Plugin:           ${parsed.plugin}`)
  console.log(`Chain id:         ${net.chainId.toString()}`)
  console.log(`Latest block:     ${blockNumber}`)
  console.log(`Proposal id:      ${proposalId.toString()}`)
  console.log(`Executed:         ${executed}`)
  console.log(
    `Approvals:        ${Number(approvals)} / ${Number(parameters.minApprovals)} required`
  )
  if (canExecute !== null) {
    console.log(`canExecute:       ${canExecute}`)
  } else {
    console.log(`canExecute:       unknown (${canExecuteError})`)
  }
  console.log(`Start date:       ${formatTimestamp(parameters.startDate)}`)
  console.log(`End date:         ${formatTimestamp(parameters.endDate)}`)
  console.log(`Window status:    ${windowStatus}`)
  console.log(
    `Etherscan:        ${etherscanEnabled ? 'enabled' : `disabled (set ${ETHERSCAN_API_KEY_ENV_VAR} to enable contract-name + calldata decoding)`}`
  )
  console.log(`Actions:          ${describedActions.length}`)

  for (let i = 0; i < describedActions.length; i++) {
    const a = describedActions[i]
    const e = enrichments[i]
    const hasCode = (await provider.getCode(a.to)) !== '0x'
    console.log('')
    console.log(`  [${a.index}] to:    ${a.to}`)
    if (!hasCode && a.data !== '0x') {
      console.log('      !!! WARNING: NO CODE AT THIS ADDRESS, BUT DATA IS NON-EMPTY !!!')
      console.log('      !!! the call will be silently discarded by the EVM          !!!')
    }
    if (hasCode) {
      const nameLine = formatNameLine(e)
      if (nameLine) console.log(`      name:  ${nameLine}`)
    }
    const valueWei = BigInt(a.value)
    console.log(`      value: ${valueWei === 0n ? '0' : `${formatEther(valueWei)} ETH (${a.value} wei)`}`)
    console.log(`      data:  ${a.data}`)
    if (etherscanEnabled && hasCode) printDecoded(e)
  }
}

main().catch((err) => {
  console.error(`Error: ${err.stack || err.message || err}`)
  process.exit(1)
})
