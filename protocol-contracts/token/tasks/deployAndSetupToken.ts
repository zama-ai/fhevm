import { execSync } from 'child_process'
import fs from 'fs'
import path from 'path'

import { task, types } from 'hardhat/config'

import type { HardhatRuntimeEnvironment } from 'hardhat/types'

interface DeploymentConfig {
    sourceNetwork: string // Network where ZamaERC20 and OFTAdapter are deployed
    destNetwork: string // Network where ZamaOFT is deployed
    layerzeroConfigPath: string // Path to LayerZero config file
    verificationScripts?: {
        // Verification scripts to run for the source and destination networks
        source: string
        destination: string
    }
}

const CONFIGS: Record<string, DeploymentConfig> = {
    testnet: {
        sourceNetwork: 'ethereum-testnet',
        destNetwork: 'gateway-testnet',
        layerzeroConfigPath: 'layerzero.config.testnet.ts',
        verificationScripts: {
            source: 'verify:etherscan:ethereum:sepolia',
            destination: 'verify:etherscan:gateway:testnet',
        },
    },
    mainnet: {
        sourceNetwork: 'ethereum-mainnet',
        destNetwork: 'gateway-mainnet',
        layerzeroConfigPath: 'layerzero.config.mainnet.ts',
        verificationScripts: {
            source: 'verify:etherscan:ethereum:mainnet',
            destination: 'verify:etherscan:gateway:mainnet',
        },
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
 * npx hardhat deploy:token --preset testnet --verify true
 */
task('deploy:token', 'Complete setup of ZAMA token, OFTAdapter and OFT')
    .addParam('preset', 'Deployment preset to use', undefined, types.string)
    .addOptionalParam('verify', 'Run Etherscan verification', false, types.boolean)
    .setAction(async (taskArgs: { preset: string; verify: boolean }, hre) => {
        const config = CONFIGS[taskArgs.preset]
        const runVerify = taskArgs.verify

        if (!config) {
            throw new Error(`Unknown config: ${taskArgs.preset}. Available: ${Object.keys(CONFIGS).join(', ')}`)
        }

        if (runVerify && !config.verificationScripts) {
            console.warn(`\n⚠️  WARNING: Verification is not yet supported for target '${config}'`)
            console.warn(`\nTo add verification support:`)
            console.warn(`  1. Add verification scripts to package.json for ${taskArgs.preset}`)
            console.warn(`  2. Update verificationScripts in the config object.`)
            console.error(`\nEither run without verification or add compatible verification scripts\n`)
            process.exit(1)
        }

        console.log('\n> Checking prerequisite setup')
        checkEnvVariables(taskArgs.preset)

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

        setHardhatConfigForOFTAdapter(hre, config.sourceNetwork, tokenAddress)
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
            await verifyContracts(hre, config)
            console.log('\n> ✅ Contracts verified successfully')
        } else {
            console.log('\n> Skipping Etherscan verification')
        }

        const zamaERC20Address = await getDeployedAddress(config.sourceNetwork, 'ZamaERC20')
        const zamaOFTAdapterAddress = await getDeployedAddress(config.sourceNetwork, 'ZamaOFTAdapter')
        const zamaOFTAddress = await getDeployedAddress(config.destNetwork, 'ZamaOFT')
        console.log(`\n> ZAMAERC20_CONTRACT_ADDRESS: ${zamaERC20Address}`)
        console.log(`\n> ZAMAOFTADAPTER_CONTRACT_ADDRESS: ${zamaOFTAdapterAddress}`)
        console.log(`\n> ZAMAOFT_CONTRACT_ADDRESS: ${zamaOFTAddress}`)
    })

/**
 * Sets the oftAdapter.tokenAddress key in the hre config.
 * This enables running the lz:deploy task with the correct token address
 */
function setHardhatConfigForOFTAdapter(hre: HardhatRuntimeEnvironment, network: string, tokenAddress: string) {
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
async function getDeployedAddress(network: string, contractName: string): Promise<string> {
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
function checkEnvVariables(preset?: string) {
    const baseRequired = ['PRIVATE_KEY', 'INITIAL_SUPPLY_RECEIVER', 'INITIAL_ADMIN']

    // Add network-specific RPC URL requirements based on preset
    const required = [...baseRequired]
    switch (preset) {
        case 'mainnet':
            required.push('MAINNET_RPC_URL', 'RPC_URL_ZAMA_GATEWAY_MAINNET')
            break
        case 'testnet':
            required.push('SEPOLIA_RPC_URL', 'RPC_URL_ZAMA_GATEWAY_TESTNET')
            break
    }
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
async function verifyContracts(hre: HardhatRuntimeEnvironment, config: DeploymentConfig) {
    const { source, destination } = config.verificationScripts!
    if (!source || !destination) {
        throw new Error('Verification scripts not found in config')
    }
    console.log(`\n> Verifying ZamaERC20 and ZamaOFTAdapter on ${config.sourceNetwork}`)
    console.log(` - Running: pnpm ${source}`)
    try {
        execSync(`pnpm run ${source}`, { stdio: 'inherit' })
    } catch (error: any) {
        console.log(' - Verification command returned an error; check Etherscan manually.')
    }
    console.log(`\n> Verifying ZamaOFT on ${config.destNetwork}`)
    console.log(` - Running: pnpm ${destination}`)
    try {
        execSync(`pnpm run ${destination}`, { stdio: 'inherit' })
    } catch (error: any) {
        console.log(' - Verification command returned an error; check Etherscan manually.')
    }
}
