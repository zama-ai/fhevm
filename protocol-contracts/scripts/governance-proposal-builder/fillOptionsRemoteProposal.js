const fs = require('node:fs')
const path = require('node:path')
require('dotenv').config({ quiet: true })
const { Options } = require('@layerzerolabs/lz-v2-utilities')
const {
  isAddress,
  isHexString,
  Interface,
  FunctionFragment,
  AbiCoder,
  JsonRpcProvider,
  keccak256,
  toUtf8Bytes,
  dataSlice,
  concat,
} = require('ethers')
const { resolveDestination, listDestinationIds } = require('./destinations')

const DEFAULT_INPUT = 'remote-proposal-temp.json'
const FILLED_OUTPUT_FILENAME = 'remote-proposal-filled.json'
const ARAGON_OUTPUT_FILENAME = 'aragonProposal.json'
const SCRIPT_NAME = 'fillOptionsRemoteProposal.js'

const SEND_REMOTE_PROPOSAL_ABI = [
  'function sendRemoteProposal(address[] targets, uint256[] values, string[] functionSignatures, bytes[] datas, uint8[] operations, bytes options) payable',
]
const SEND_REMOTE_PROPOSAL_IFACE = new Interface(SEND_REMOTE_PROPOSAL_ABI)

const EXPECTED_METHOD = 'sendRemoteProposal'
const EXPECTED_EMPTY_OPTIONS = '0x'

// Buffers added on top of the summed per-call gas estimates to cover the
// pieces this script does NOT measure directly (lzReceive entry, abi.decode
// of the message, Safe module wrapping, event emission, RPC noise). The
// constant base overhead is in gas units; the proportional buffer is in
// basis points (10000 = 100%).
//
// NOTE: these were calibrated for the Ethereum -> Gateway path. They may not
// hold for other destinations (e.g. Polygon Amoy) — if a proposal gets stuck
// on delivery, recover it via the manual-execution runbook and recalibrate
// these constants here (a fix here benefits every future proposal).
const GAS_BASE_OVERHEAD = 130_000
const GAS_BUFFER_BPS = 3000 // 30% safety margin on top of the total

const ABI_CODER = AbiCoder.defaultAbiCoder()

// Input shape. The user provides ONLY these three equal-length arrays; the
// script fills everything else (to = the destination's GovernanceOAppSender,
// method, values = all 0, operations = all 0 (Call), options):
//   { "targets": [...], "functionSignatures": [...], "datas": [...] }
// Every governance proposal (add owner, change threshold, pause/unpause, grant
// role, upgrade) is value 0 / Call, so no other shape is needed.
const SIMPLE_KEYS = ['targets', 'functionSignatures', 'datas']

// Old full-shape files carried these top-level keys; used only to give a
// helpful hint if someone passes one to the (now minimal-only) --input.
const FULL_SHAPE_KEYS = ['to', 'method', 'arguments']

function fail(message) {
  console.error(`Error: ${message}`)
  process.exit(1)
}

function failList(header, messages) {
  console.error(`Error: ${header}`)
  for (const m of messages) console.error(`  - ${m}`)
  process.exit(1)
}

function printUsage() {
  console.log(
    `Usage:
  npm run fill-options-remote-proposal -- --destination <id> [--input <file>]
  node ${SCRIPT_NAME} --destination <id> [--input <file>]

Flags:
  --destination    REQUIRED  Destination chain id. One of:
                               ${listDestinationIds().join('\n                               ')}
  --input <file>   OPTIONAL  Minimal proposal file (default: ${DEFAULT_INPUT}).
                             Contains ONLY three equal-length arrays:
                               { "targets", "functionSignatures", "datas" }
                             The script fills "to" (the destination's
                             GovernanceOAppSender), "method", "values" (all 0),
                             "operations" (all 0) and "options".
  --allowEmptyFunctionSignatures
                   OPTIONAL  Allow empty "functionSignatures[i]" entries. By
                             default every entry is required so each call
                             decodes to an auditable signature. When set, an
                             empty entry is treated as raw calldata: the
                             matching "datas[i]" is used verbatim as the
                             on-chain calldata (selector included) and is NOT
                             decoded in the sanity check. Off by default; only
                             needed for calls without a human-readable ABI
                             signature.
  -h, --help                 Show this help.

For each call, decodes datas[i] against functionSignatures[i] and prints it as
a built-in sanity check (aborts on a signature/datas mismatch). Then runs
eth_estimateGas against the destination RPC (from .env, falling back to the
registry default) with the destination multisig as the (unsigned) "from" to
size the lzReceive gas, and writes:
  - ${FILLED_OUTPUT_FILENAME}  (the full filled proposal, human-readable record)
  - ${ARAGON_OUTPUT_FILENAME}  (the same call as a single Aragon transaction,
                               [{ to, value, data }], ready to upload)

Destinations and their RPC env vars are defined in destinations.js
(run \`node destinations.js\` to list them). Refuses to overwrite either
output file if it already exists.
`
  )
}

function parseArgs(argv) {
  const args = {
    input: undefined,
    destinationId: undefined,
    allowEmptyFunctionSignatures: false,
  }

  for (let i = 2; i < argv.length; i++) {
    const flag = argv[i]
    switch (flag) {
      case '-h':
      case '--help':
        printUsage()
        process.exit(0)
      case '--input': {
        const value = argv[++i]
        if (!value) throw new Error(`Missing value for ${flag}`)
        args.input = value
        break
      }
      case '--destination': {
        const value = argv[++i]
        if (!value) throw new Error(`Missing value for ${flag}`)
        if (args.destinationId && args.destinationId !== value) {
          throw new Error('--destination was passed more than once with different values.')
        }
        args.destinationId = value
        break
      }
      case '--allowEmptyFunctionSignatures':
        args.allowEmptyFunctionSignatures = true
        break
      default:
        throw new Error(`Unknown flag: ${flag}`)
    }
  }

  if (!args.destinationId) {
    throw new Error(`Missing required flag: --destination <${listDestinationIds().join('|')}>`)
  }

  return args
}

function loadJson(filePath) {
  const absolutePath = path.resolve(process.cwd(), filePath)
  if (!fs.existsSync(absolutePath)) {
    throw new Error(`Proposal file not found: ${absolutePath}`)
  }
  try {
    return { absolutePath, json: JSON.parse(fs.readFileSync(absolutePath, 'utf8')) }
  } catch (err) {
    throw new Error(`Failed to parse JSON in ${absolutePath}: ${err.message}`)
  }
}

function isPlainObject(v) {
  return v !== null && typeof v === 'object' && !Array.isArray(v)
}

function checkExactKeys(obj, expectedKeys, label, errors) {
  const expectedSet = new Set(expectedKeys)
  for (const key of expectedKeys) {
    if (!(key in obj)) errors.push(`${label} is missing key "${key}".`)
  }
  for (const key of Object.keys(obj)) {
    if (!expectedSet.has(key)) errors.push(`${label} has unexpected key "${key}".`)
  }
}

// Validates and returns { targets, functionSignatures, datas } string arrays of
// equal length from a minimal input file. When allowEmptyFunctionSignatures is
// true, empty "functionSignatures[i]" entries are permitted (treated as raw
// calldata downstream); otherwise every entry is required.
function validateSimpleInput(input, allowEmptyFunctionSignatures) {
  const errors = []
  if (!isPlainObject(input)) {
    failList('input file must be a JSON object', [
      'expected { "targets": [...], "functionSignatures": [...], "datas": [...] }',
    ])
  }
  checkExactKeys(input, SIMPLE_KEYS, '<root>', errors)
  if (errors.length && Object.keys(input).some((k) => FULL_SHAPE_KEYS.includes(k))) {
    errors.push(
      'this looks like an old full proposal file ({ to, method, arguments }); this tool now takes only { targets, functionSignatures, datas } and fills the rest.'
    )
  }
  for (const key of SIMPLE_KEYS) {
    if (!(key in input)) continue
    if (!Array.isArray(input[key])) {
      errors.push(`"${key}" must be an array.`)
      continue
    }
    input[key].forEach((v, idx) => {
      if (typeof v !== 'string') {
        errors.push(`"${key}[${idx}]" must be a string (got ${typeof v}).`)
      } else if (key === 'functionSignatures' && v.trim() === '' && !allowEmptyFunctionSignatures) {
        // Enforced by default: even though the receiver accepts raw calldata
        // (empty signature), we require the human-readable signature here so
        // every proposal decodes to an auditable call. Pass
        // --allowEmptyFunctionSignatures to override for calls that have no
        // human-readable ABI signature.
        errors.push(
          `"functionSignatures[${idx}]" must not be empty — always provide the human-readable signature, e.g. "addOwnerWithThreshold(address,uint256)" (or pass --allowEmptyFunctionSignatures to treat "datas[${idx}]" as raw calldata).`
        )
      }
    })
  }
  if (Array.isArray(input.targets)) {
    if (input.targets.length === 0) errors.push('"targets" must not be empty.')
    for (const key of ['functionSignatures', 'datas']) {
      if (Array.isArray(input[key]) && input[key].length !== input.targets.length) {
        errors.push(
          `"${key}" length (${input[key].length}) does not match "targets" length (${input.targets.length}).`
        )
      }
    }
  }
  if (errors.length) failList('invalid input file:', errors)
  return { targets: input.targets, functionSignatures: input.functionSignatures, datas: input.datas }
}

// Builds the full canonical proposal from validated arguments.
function buildCanonicalProposal(destination, argsObj) {
  return {
    to: destination.oappSender,
    method: EXPECTED_METHOD,
    arguments: {
      targets: argsObj.targets,
      values: argsObj.values,
      functionSignatures: argsObj.functionSignatures,
      datas: argsObj.datas,
      operations: argsObj.operations,
      options: EXPECTED_EMPTY_OPTIONS,
    },
  }
}

// Built-in sanity check: for each call, decode datas[i] against
// functionSignatures[i] and print it. Aborts on a signature/datas mismatch so
// a mis-encoded call is caught before the proposal is ever submitted.
function sanityCheckAndReport(args) {
  const { targets, values, functionSignatures, datas, operations } = args
  console.log('Sanity check — decoded calls:')
  for (let i = 0; i < targets.length; i++) {
    const sig = functionSignatures[i]
    const data = datas[i]
    if (!isHexString(data)) {
      fail(`datas[${i}] must be a 0x-prefixed hex string with whole bytes (got ${JSON.stringify(data)}).`)
    }
    console.log(`  [${i}] target:    ${targets[i]}`)
    console.log(`      value:     ${values[i]}   operation: ${operations[i]}`)
    // Empty signature (only reachable with --allowEmptyFunctionSignatures):
    // datas[i] is the full raw calldata, so there is nothing to decode. Print
    // the raw calldata and flag that it was NOT decoded/audited here.
    if (sig.trim() === '') {
      console.log(`      call:      <raw calldata — empty functionSignature, not decoded>`)
      console.log(`      calldata:  ${data}`)
      continue
    }
    let fragment
    try {
      fragment = FunctionFragment.from(sig)
    } catch (err) {
      fail(`functionSignatures[${i}] ("${sig}") is not a valid function signature: ${err.message}`)
    }
    let decoded
    try {
      decoded = ABI_CODER.decode(fragment.inputs, data)
    } catch (err) {
      fail(
        `datas[${i}] does not match functionSignatures[${i}] ("${sig}"): ${err.shortMessage || err.message}. ` +
          `Remember datas must be ABI-encoded WITHOUT the 4-byte selector.`
      )
    }
    console.log(`      call:      ${fragment.name}(${fragment.inputs.map((p) => p.type).join(',')})`)
    fragment.inputs.forEach((p, j) => {
      console.log(`        ${p.name || `arg${j}`} (${p.type}): ${decoded[j]}`)
    })
  }
  console.log('')
}

function computeLZOptions(gasLimit, nativeValue) {
  return Options.newOptions().addExecutorLzReceiveOption(gasLimit, nativeValue).toHex().toString()
}

function buildOnChainCalldata(functionSignature, data) {
  // Empty signature (only with --allowEmptyFunctionSignatures): data already is
  // the full raw calldata (selector included), so use it verbatim.
  if (functionSignature.trim() === '') return data
  const selector = dataSlice(keccak256(toUtf8Bytes(functionSignature)), 0, 4)
  return concat([selector, data])
}

async function estimatePerCallGas(provider, safeProxy, proposalArgs, destinationLabel) {
  const { targets, functionSignatures, datas } = proposalArgs
  const estimates = []
  for (let i = 0; i < targets.length; i++) {
    const code = await provider.getCode(targets[i])
    if (code === '0x') {
      throw new Error(`target #${i} (${targets[i]}) has no bytecode on ${destinationLabel}`)
    }

    const calldata = buildOnChainCalldata(functionSignatures[i], datas[i])
    let gas
    try {
      gas = await provider.estimateGas({ from: safeProxy, to: targets[i], data: calldata, value: 0 })
    } catch (err) {
      const reason = err.shortMessage || err.message
      throw new Error(
        `eth_estimateGas failed for call #${i} (target=${targets[i]}, signature="${functionSignatures[i]}"): ${reason}`
      )
    }
    estimates.push(gas)
  }
  return estimates
}

function applyGasBuffers(perCallEstimates) {
  const sumEstimates = perCallEstimates.reduce((acc, g) => acc + g, 0n)
  const subtotal = sumEstimates + BigInt(GAS_BASE_OVERHEAD)
  const buffered = (subtotal * BigInt(10_000 + GAS_BUFFER_BPS)) / 10_000n
  return { sumEstimates, subtotal, buffered }
}

/**
 * Builds the Aragon proposal payload corresponding to a filled remote
 * proposal: a single Aragon transaction that calls sendRemoteProposal on the
 * GovernanceOAppSender with the same arguments. The shape matches what the
 * Aragon front-end expects when uploading a JSON proposal: an array of
 * { to, value, data } objects.
 */
function buildAragonProposal(filledProposal) {
  const a = filledProposal.arguments
  const data = SEND_REMOTE_PROPOSAL_IFACE.encodeFunctionData('sendRemoteProposal', [
    a.targets,
    a.values,
    a.functionSignatures,
    a.datas,
    a.operations,
    a.options,
  ])
  return [{ to: filledProposal.to, value: 0, data }]
}

function writeOutputFile(outputDir, filename, data) {
  const outputPath = path.join(outputDir, filename)
  // 'wx' makes open+create atomic and fails if the file already exists, so we
  // never overwrite an existing output file (even on a TOCTOU race).
  try {
    fs.writeFileSync(outputPath, JSON.stringify(data, null, 2) + '\n', { flag: 'wx' })
  } catch (err) {
    if (err.code === 'EEXIST') throw new Error(`Refusing to overwrite existing file: ${outputPath}`)
    throw err
  }
  return outputPath
}

function assertOutputDoesNotExist(outputDir, filename) {
  const outputPath = path.join(outputDir, filename)
  if (fs.existsSync(outputPath)) throw new Error(`Refusing to overwrite existing file: ${outputPath}`)
}

async function main() {
  let args
  try {
    args = parseArgs(process.argv)
  } catch (err) {
    console.error(`Error: ${err.message}\n`)
    printUsage()
    process.exit(1)
  }

  let destination
  try {
    destination = resolveDestination(args.destinationId)
  } catch (err) {
    fail(err.message)
  }

  // Load the minimal input file and build the full canonical proposal: the
  // user supplies only { targets, functionSignatures, datas }; the script fills
  // to/method/values(0)/operations(0)/options.
  const inputPath = args.input || DEFAULT_INPUT
  let absolutePath, json
  try {
    ;({ absolutePath, json } = loadJson(inputPath))
  } catch (err) {
    fail(err.message)
  }

  const simple = validateSimpleInput(json, args.allowEmptyFunctionSignatures)
  // Each entry in targets must be a well-formed address.
  const invalid = simple.targets.filter((t) => !isAddress(t))
  if (invalid.length) failList('input "targets" contains invalid addresses:', invalid)
  const proposal = buildCanonicalProposal(destination, {
    targets: simple.targets,
    values: simple.targets.map(() => '0'),
    functionSignatures: simple.functionSignatures,
    datas: simple.datas,
    operations: simple.targets.map(() => '0'),
  })

  // Built-in sanity check: decode each datas[i] against functionSignatures[i].
  sanityCheckAndReport(proposal.arguments)

  // Estimate the lzReceive gas limit by running eth_estimateGas against the
  // destination RPC with the destination multisig as the (unsigned) `from`,
  // for each (target, calldata) one by one.
  const rpcUrl = process.env[destination.rpcEnvVar] || destination.defaultRpc
  if (!rpcUrl || rpcUrl.trim() === '') {
    fail(`no RPC URL for ${destination.id}. Set ${destination.rpcEnvVar} in .env (see .env.example).`)
  }

  const safeProxy = destination.destinationExecutor
  const provider = new JsonRpcProvider(rpcUrl)
  let chainId
  try {
    chainId = (await provider.getNetwork()).chainId
  } catch (err) {
    fail(`failed to connect to ${destination.rpcEnvVar}=${rpcUrl}: ${err.shortMessage || err.message}`)
  }

  let perCallEstimates
  try {
    perCallEstimates = await estimatePerCallGas(provider, safeProxy, proposal.arguments, destination.displayName)
  } catch (err) {
    fail(err.message)
  }
  const { buffered } = applyGasBuffers(perCallEstimates)
  const gasLimit = buffered

  const lzOptions = computeLZOptions(gasLimit, 0) // always assuming native value to be sent is 0

  const filledProposal = { ...proposal, arguments: { ...proposal.arguments, options: lzOptions } }
  const aragonProposal = buildAragonProposal(filledProposal)

  // Write both outputs next to the input, refusing to overwrite either.
  const outputDir = path.dirname(absolutePath)
  let filledPath, aragonPath
  try {
    assertOutputDoesNotExist(outputDir, FILLED_OUTPUT_FILENAME)
    assertOutputDoesNotExist(outputDir, ARAGON_OUTPUT_FILENAME)
    filledPath = writeOutputFile(outputDir, FILLED_OUTPUT_FILENAME, filledProposal)
    aragonPath = writeOutputFile(outputDir, ARAGON_OUTPUT_FILENAME, aragonProposal)
  } catch (err) {
    fail(err.message)
  }

  console.log(`Input:                 ${absolutePath}`)
  console.log(`Destination:           ${destination.id} — ${destination.displayName} (chainId ${chainId.toString()})`)
  console.log(
    `LZ executor option (gas limit estimated via eth_estimateGas with the destination multisig as from):    lzReceive(gasLimit=${gasLimit.toString()})`
  )
  console.log(`                       ${lzOptions}`)
  console.log(`Wrote filled proposal: ${filledPath}`)
  console.log(`Wrote Aragon proposal: ${aragonPath}`)
}

main().catch((err) => {
  console.error(`Error: ${err.stack || err.message || err}`)
  process.exit(1)
})
