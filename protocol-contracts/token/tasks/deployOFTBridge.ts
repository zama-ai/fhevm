import { task, types } from 'hardhat/config'
import type { HardhatRuntimeEnvironment } from 'hardhat/types'
import fs from 'fs'
import path from 'path'
import { execSync } from 'child_process'

interface DeploymentConfig {
    sourceNetwork: string // Network where ZamaERC20 and OFTAdapter are deployed
    destNetwork: string // Network where ZamaOFT is deployed
    layerzeroConfigPath: string // Path to LayerZero config file
}

const CONFIGS: Record<string, DeploymentConfig> = {
    arbitrum_testnet: {
        sourceNetwork: 'ethereum-testnet',
        destNetwork: 'arbitrum-testnet',
        layerzeroConfigPath: 'layerzero.config.arbitrumtestnet.ts',
    },
    gateway_testnet: {
        sourceNetwork: 'ethereum-testnet',
        destNetwork: 'gateway-testnet',
        layerzeroConfigPath: 'layerzero.config.gatewaytestnet.ts',
    },
}

// Verification script mapping that matches package.json scripts
const VERIFICATION_SCRIPTS: Record<string, { ethereum: string; destination: string }> = {
    arbitrum_testnet: {
        ethereum: 'verify:etherscan:ethereum:sepolia',
        destination: 'verify:etherscan:arbitrum:sepolia',
    },
    gateway_testnet: {
        ethereum: 'verify:etherscan:ethereum:sepolia',
        destination: 'verify:etherscan:gateway:testnet',
    },
}

/**
 * Orchestrates the full OFT bridge deployment:
 * 1. Deploy ZamaERC20 on source network
 * 2. Update hardhat config with deployed token address
 * 3. Deploy ZamaOFTAdapter on source network
 * 4. Deploy ZamaOFT on destination network
 * 5. Wire the contracts together
 *
 * Example usage:
 * npx hardhat deploy:oft-bridge --target "arbitrum_testnet" --verify true
 */
task('deploy:oft-bridge', 'Deploy complete OFT bridge setup')
    .addParam('target', 'Target network', undefined, types.string)
    .addOptionalParam('verify', 'Run Etherscan verification', false, types.boolean)
    .setAction(async (taskArgs: { target: string; verify: boolean }, hre) => {
        const target = taskArgs.target
        const runVerify = taskArgs.verify

        if (!CONFIGS[target]) {
            throw new Error(`Unknown target: ${target}. Available: ${Object.keys(CONFIGS).join(', ')}`)
        }

        if (runVerify && !VERIFICATION_SCRIPTS[target]) {
            console.warn(`\n⚠️  WARNING: Verification is not yet supported for target '${target}'`)
            console.warn(`Available verification targets: ${Object.keys(VERIFICATION_SCRIPTS).join(', ')}`)
            console.warn(`\nTo add verification support:`)
            console.warn(`  1. Add verification scripts to package.json for ${target}`)
            console.warn(`  2. Update VERIFICATION_SCRIPTS in tasks/deployOFTBridge.ts`)
            console.error(`\nEither run without verification or add compatible verification scripts\n`)
            process.exit(1)
        }

        const config = CONFIGS[target]
        console.log('\n> Checking prerequisite setup')
        checkEnvVariables()

        console.log(`\n> Deploy ZamaERC20 on ${config.sourceNetwork}`)
        console.log(` - Running lz:deploy in CI mode for ZamaERC20 on ${config.sourceNetwork}.`)
        await hre.run('lz:deploy', {
            ci: true,
            networks: [config.sourceNetwork],
            tags: ['ZamaERC20'],
        })

        console.log('\n> Reading deployed ZamaERC20 address')
        const tokenAddress = await getDeployedAddress(config.sourceNetwork, 'ZamaERC20')
        console.log(` - Detected ZamaERC20 address: ${tokenAddress}`)

        patchHardhatConfigForOFTAdapter(hre, config.sourceNetwork, tokenAddress)
        console.log(` - Set ${config.sourceNetwork}.oftAdapter.tokenAddress to ${tokenAddress}`)

        console.log(`\n> Deploy ZamaOFTAdapter on ${config.sourceNetwork}`)
        console.log(` - Running lz:deploy in CI mode for ZamaOFTAdapter on ${config.sourceNetwork}.`)
        await hre.run('lz:deploy', {
            ci: true,
            networks: [config.sourceNetwork],
            tags: ['ZamaOFTAdapter'],
        })

        console.log(`\n> Deploy ZamaOFT on ${config.destNetwork}`)
        console.log(` - Running lz:deploy in CI mode for ZamaOFT on ${config.destNetwork}.`)
        await hre.run('lz:deploy', {
            ci: true,
            networks: [config.destNetwork],
            tags: ['ZamaOFT'],
        })

        console.log(`\n> Wire ZamaOFTAdapter (${config.sourceNetwork}) with ZamaOFT (${config.destNetwork})`)
        console.log(` - Running lz:oapp:wire in CI mode using ${config.layerzeroConfigPath}.`)
        await hre.run('lz:oapp:wire', {
            ci: true,
            oappConfig: config.layerzeroConfigPath,
        })

        console.log('\n✅ OFT Bridge deployment successful!')

        // Optional verification
        if (runVerify) {
            console.log('\n> Verifying contracts on Etherscan')
            await verifyContracts(hre, target)
        } else {
            console.log('\n> Skipping Etherscan verification')
            return;
        }

        console.log('\n> ✅ Contracts verified successfully')

    })

/**
 * Patches the hardhat config in memory to set the oftAdapter.tokenAddress
 * This enables running the lz:deploy task with the correct token address
 */
function patchHardhatConfigForOFTAdapter(hre: HardhatRuntimeEnvironment, network: string, tokenAddress: string) {
    if (!hre.config.networks[network]) {
        throw new Error(`Network ${network} not found in hardhat config`)
    }
    const networkConfig = hre.config.networks[network] as any
    networkConfig.oftAdapter = {
        tokenAddress,
    }
}

/**
 * Gets the deployed contract address from hardhat-deploy artifacts
 */
async function getDeployedAddress(
    network: string,
    contractName: string
): Promise<string> {
    const deploymentsPath = path.join('deployments', network, `${contractName}.json`)
    try {
        const data = JSON.parse(fs.readFileSync(deploymentsPath, 'utf8'))
        if (!data.address) {
            throw new Error(`No address found in ${deploymentsPath}`)
        }
        return data.address
    } catch (error) {
        throw new Error(`Failed to read deployment from ${deploymentsPath}: ${error}`)
    }
}

/**
 * Checks that required environment variables are set
 */
function checkEnvVariables() {
    const required = ['PRIVATE_KEY', 'SEPOLIA_RPC_URL', 'INITIAL_SUPPLY_RECEIVER', 'INITIAL_ADMIN']

    const missing = required.filter((key) => !process.env[key])

    if (missing.length > 0) {
        throw new Error(`Missing required environment variables: ${missing.join(', ')}`)
    }

    console.log(' - All required environment variables are set')
}

/**
 * Runs verification using package.json scripts
 * Must use the scripts defined in package.json as they contain the correct API endpoints
 */
async function verifyContracts(hre: HardhatRuntimeEnvironment, target: string) {
    const scripts = VERIFICATION_SCRIPTS[target]
    if (!scripts) {
        console.error(' - Verification scripts not found for this target')
        process.exit(1)
    }

    console.log(`\n> Verifying ZamaERC20 and ZamaOFTAdapter on Ethereum Sepolia`)
    console.log(` - Running: pnpm ${scripts.ethereum}`)
    try {
        execSync(`pnpm run ${scripts.ethereum}`, { stdio: 'inherit' })
    } catch (error: any) {
        console.log(' - Verification command returned an error; check Etherscan manually.')
    }

    const config = CONFIGS[target]
    console.log(`\n> Verifying ZamaOFT on ${config.destNetwork}`)
    console.log(` - Running: pnpm ${scripts.destination}`)
    try {
        execSync(`pnpm run ${scripts.destination}`, { stdio: 'inherit' })
    } catch (error: any) {
        console.log(' - Verification command returned an error; check block explorer manually.')
    }
}
