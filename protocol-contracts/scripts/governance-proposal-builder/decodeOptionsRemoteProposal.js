const { Options } = require('@layerzerolabs/lz-v2-utilities')

const SCRIPT_NAME = 'decodeOptionsRemoteProposal.js'
const EMPTY_OPTIONS = '0x'

function printUsage() {
  console.log(
    `Usage:
  npm run decode-options-remote-proposal -- --options <hex>
  node ${SCRIPT_NAME} --options <hex>

Flags:
  --options    REQUIRED  The LayerZero options hex string to decode
                         (e.g. 0x000301001101000000000000000000000000000493e0).
  -h, --help             Show this help.

Reverse of computeLZOptions in fillOptionsRemoteProposal.js: takes a
LayerZero options hex string (a single executor lzReceive option) and prints
the decoded gas limit and native value.
`
  )
}

function parseArgs(argv) {
  const args = { options: undefined }

  for (let i = 2; i < argv.length; i++) {
    const flag = argv[i]
    switch (flag) {
      case '-h':
      case '--help':
        printUsage()
        process.exit(0)
      case '--options': {
        const value = argv[++i]
        if (!value) throw new Error(`Missing value for ${flag}`)
        args.options = value
        break
      }
      default:
        throw new Error(`Unknown flag: ${flag}`)
    }
  }

  if (args.options === undefined) {
    throw new Error('Missing required flag: --options <hex>')
  }

  return args
}

/**
 * Reverse of computeLZOptions in fillOptionsRemoteProposal.js.
 *
 * Decodes a LayerZero options hex string produced by
 *   Options.newOptions().addExecutorLzReceiveOption(gasLimit, nativeValue).toHex()
 * and returns { gasLimit, nativeValue } as BigInts.
 *
 * Throws if the hex is empty ("0x"), malformed, or does not contain an
 * executor lzReceive option.
 */
function decodeLZOptions(hex) {
  if (typeof hex !== 'string') {
    throw new Error(`options must be a string (got ${typeof hex}).`)
  }
  if (hex === EMPTY_OPTIONS || hex === '') {
    throw new Error('options is empty ("0x"); nothing to decode.')
  }

  let parsed
  try {
    parsed = Options.fromOptions(hex)
  } catch (err) {
    throw new Error(`Failed to parse options hex "${hex}": ${err.message}`)
  }

  const decoded = parsed.decodeExecutorLzReceiveOption()
  if (!decoded) {
    throw new Error(`No executor lzReceive option found in: ${hex}`)
  }
  // The library returns { gas, value }; rename to match computeLZOptions's
  // (gasLimit, nativeValue) parameter names so the reverse mapping is obvious.
  return {
    gasLimit: BigInt(decoded.gas),
    nativeValue: BigInt(decoded.value),
  }
}

function main() {
  let args
  try {
    args = parseArgs(process.argv)
  } catch (err) {
    console.error(`Error: ${err.message}\n`)
    printUsage()
    process.exit(1)
  }

  let decoded
  try {
    decoded = decodeLZOptions(args.options)
  } catch (err) {
    console.error(`Error: ${err.message}`)
    process.exit(1)
  }

  console.log(`Options hex:   ${args.options}`)
  console.log(`gasLimit:      ${decoded.gasLimit.toString()}`)
  console.log(`nativeValue:   ${decoded.nativeValue.toString()}`)
}

if (require.main === module) {
  main()
}

module.exports = { decodeLZOptions }
