import * as fs from 'fs'
import * as path from 'path'

interface WiringItem {
    Endpoint: string
    OmniAddress: string
    Description: string
    Data: string
}

interface RemoteProposal {
    targets: string[]
    functionSignatures: string[]
    datas: string[]
}

function convertToRemoteProposal(inputPath: string, outputPath: string): void {
    const items: WiringItem[] = JSON.parse(fs.readFileSync(inputPath, 'utf-8'))

    const proposal: RemoteProposal = {
        targets: items.map((item) => item.OmniAddress),
        // The wiring `Data` already embeds the 4-byte selector, so the signature is
        // left empty and the full calldata is passed through as-is.
        functionSignatures: items.map(() => ''),
        datas: items.map((item) => item.Data),
    }

    fs.writeFileSync(outputPath, JSON.stringify(proposal, null, 2) + '\n')

    console.log(`✅ Converted ${items.length} wiring entries to remote proposal format`)
    console.log(`📄 Output saved to: ${outputPath}`)
}

const inputFile = process.argv[2] || 'polygon-bridge-wiring.json'
const outputFile = process.argv[3] || 'remote-proposal-temp.json'

const inputPath = path.resolve(process.cwd(), inputFile)
const outputPath = path.resolve(process.cwd(), outputFile)

if (!fs.existsSync(inputPath)) {
    console.error(`❌ Input file not found: ${inputPath}`)
    process.exit(1)
}

convertToRemoteProposal(inputPath, outputPath)
