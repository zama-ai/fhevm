import * as fs from 'fs'
import * as path from 'path'
import { utils } from 'ethers'

interface WiringItem {
    Endpoint: string
    OmniAddress: string
    Description: string
    Data: string
}

const SET_DST_CHAIN_ID_ABI = ['function setDstChainId(uint32 dstEid, uint64 dstChainId)']

function requireEnv(name: string): string {
    const value = process.env[name]
    if (!value) {
        console.error(`❌ Missing required environment variable: ${name}`)
        process.exit(1)
    }
    return value
}

function requireFlag(name: string): string {
    const prefix = `--${name}=`
    for (let i = 2; i < process.argv.length; i++) {
        const arg = process.argv[i]
        if (arg === `--${name}`) {
            const value = process.argv[i + 1]
            if (!value || value.startsWith('--')) {
                break
            }
            return value
        }
        if (arg.startsWith(prefix)) {
            return arg.slice(prefix.length)
        }
    }
    console.error(`❌ Missing required flag: --${name} <value>`)
    process.exit(1)
}

function encodeSetDstChainId(dstEid: string, dstChainId: string): string {
    const iface = new utils.Interface(SET_DST_CHAIN_ID_ABI)
    return iface.encodeFunctionData('setDstChainId', [dstEid, dstChainId])
}

function appendSetDstChainId(wiringFile: string, bridgeAddress: string, dstEid: string, dstChainId: string): void {
    const wiringPath = path.resolve(process.cwd(), wiringFile)

    if (!fs.existsSync(wiringPath)) {
        console.error(`❌ Wiring file not found: ${wiringPath}`)
        process.exit(1)
    }

    const items: WiringItem[] = JSON.parse(fs.readFileSync(wiringPath, 'utf-8'))

    const data = encodeSetDstChainId(dstEid, dstChainId)

    // Keep the append idempotent: drop any previous setDstChainId call for the same bridge.
    const filtered = items.filter(
        (item) => !(item.OmniAddress.toLowerCase() === bridgeAddress.toLowerCase() && item.Data.startsWith(data.slice(0, 10)))
    )

    // Reuse the Endpoint of the existing wiring entries instead of a dedicated env var.
    const endpoint = filtered[0]?.Endpoint
    if (!endpoint) {
        console.error(`❌ Cannot derive Endpoint: no existing wiring entries in ${wiringFile}`)
        process.exit(1)
    }

    filtered.push({
        Endpoint: endpoint,
        OmniAddress: bridgeAddress,
        Description: `Setting bridge dstChainId for eid ${dstEid} to ${dstChainId}`,
        Data: data,
    })

    fs.writeFileSync(wiringPath, JSON.stringify(filtered, null, 2) + '\n')

    console.log(`✅ Appended setDstChainId(${dstEid}, ${dstChainId}) for ${bridgeAddress} to ${wiringFile}`)
}

const srcWiringFilename = requireFlag('src-wiring-filename')
const dstWiringFilename = requireFlag('dst-wiring-filename')

const srcEid = requireEnv('SRC_EID')
const dstEid = requireEnv('DST_EID')
const srcChainId = requireEnv('SRC_CHAIN_ID')
const dstChainId = requireEnv('DST_CHAIN_ID')
const srcBridgeAddress = requireEnv('SRC_BRIDGE_ADDRESS')
const dstBridgeAddress = requireEnv('DST_BRIDGE_ADDRESS')

// Ethereum/Sepolia bridge -> set the Polygon/Amoy destination chain id.
appendSetDstChainId(srcWiringFilename, srcBridgeAddress, dstEid, dstChainId)

// Polygon/Amoy bridge -> set the Ethereum/Sepolia destination chain id.
appendSetDstChainId(dstWiringFilename, dstBridgeAddress, srcEid, srcChainId)
