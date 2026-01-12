import { Options } from '@layerzerolabs/lz-v2-utilities'
import { task, types } from 'hardhat/config'
import { HardhatRuntimeEnvironment } from 'hardhat/types'

// The SENTINEL_OWNERS is the head of the linked list in Safe's owner management
const SENTINEL_OWNERS = '0x0000000000000000000000000000000000000001'

/**
 * Conservative gas estimates for Safe owner management operations.
 * These are intentionally high to ensure transactions don't run out of gas.
 * The actual gas usage will likely be lower.
 *
 * Formula: baseOverhead + (numOperations * gasPerOperation)
 * TODO: this could be optimized later with proper gas profiling,
 * see https://github.com/zama-ai/fhevm-internal/issues/660
 */
const GAS_ESTIMATES = {
    gasPerOperation: 100000, // Conservative estimate per Safe operation (swap/add/remove/changeThreshold)
    baseOverhead: 100000, // Base overhead for LZ receive, message decoding, AdminModule, Safe proxy
}

interface SafeOwnerChangeOutput {
    targets: string[]
    values: string[]
    functionSignatures: string[]
    datas: string[]
    operations: number[]
    optionsBytes: string
}

/**
 * Finds the previous owner in the linked list for a given owner.
 * In Safe, owners are stored as a linked list: SENTINEL -> owner1 -> owner2 -> ... -> ownerN -> SENTINEL
 * @param owners Array of current owners (in linked list order, first owner is pointed to by SENTINEL)
 * @param owner The owner to find the previous owner for
 * @returns The previous owner address (or SENTINEL if owner is the first)
 */
function findPrevOwner(owners: string[], owner: string): string {
    const ownerLower = owner.toLowerCase()
    const index = owners.findIndex((o) => o.toLowerCase() === ownerLower)
    if (index === -1) {
        throw new Error(`Owner ${owner} not found in owners list`)
    }
    if (index === 0) {
        return SENTINEL_OWNERS
    }
    return owners[index - 1]
}

/**
 * Computes the sequence of Safe owner management calls needed to transform
 * the current owner set to the desired owner set with the new threshold.
 *
 * Strategy:
 * 1. First, swap owners that need to be replaced (keeping count stable)
 * 2. Then, either add new owners OR remove extra owners (never both, because new set of owners is either larger OR smaller than current set)
 * 3. Set the final threshold on the last add/remove to avoid extra changeThreshold call
 * 4. If only swaps (or no owner changes), add changeThreshold if needed
 */
async function computeOwnerChanges(
    hre: HardhatRuntimeEnvironment,
    currentOwners: string[],
    currentThreshold: number,
    newOwners: string[],
    newThreshold: number
): Promise<SafeOwnerChangeOutput> {
    const targets: string[] = []
    const values: string[] = []
    const functionSignatures: string[] = []
    const datas: string[] = []
    const operations: number[] = []

    const currentOwnersSet = new Set(currentOwners.map((o) => o.toLowerCase()))
    const newOwnersSet = new Set(newOwners.map((o) => o.toLowerCase()))

    // Find owners to remove (in current but not in new)
    const ownersToRemove = currentOwners.filter((o) => !newOwnersSet.has(o.toLowerCase()))
    // Find owners to add (in new but not in current)
    const ownersToAdd = newOwners.filter((o) => !currentOwnersSet.has(o.toLowerCase()))
    // Find owners to keep (in both)
    const ownersToKeep = currentOwners.filter((o) => newOwnersSet.has(o.toLowerCase()))

    console.log('\n📊 Owner change analysis:')
    console.log(`   New owners (${newOwners.length}):`, newOwners)
    console.log(`   Owners to keep (${ownersToKeep.length}):`, ownersToKeep)
    console.log(`   Owners to remove (${ownersToRemove.length}):`, ownersToRemove)
    console.log(`   Owners to add (${ownersToAdd.length}):`, ownersToAdd)
    console.log(`   New threshold: ${newThreshold}`)

    // Validate threshold
    if (newThreshold < 1) {
        throw new Error('Threshold must be at least 1')
    }
    if (newThreshold > newOwners.length) {
        throw new Error(`Threshold (${newThreshold}) cannot be greater than number of owners (${newOwners.length})`)
    }

    // Track the current state as we build operations
    let workingOwners = [...currentOwners]

    // Step 1: Perform swaps first (to replace owners while keeping count stable)
    // Match owners to remove with owners to add for swapping
    const swapCount = Math.min(ownersToRemove.length, ownersToAdd.length)

    for (let i = 0; i < swapCount; i++) {
        const oldOwner = ownersToRemove[i]
        const newOwner = ownersToAdd[i]
        const prevOwner = findPrevOwner(workingOwners, oldOwner)

        console.log(`\n🔄 Swap: ${oldOwner} -> ${newOwner} (prev: ${prevOwner})`)

        targets.push('SAFE_ADDRESS') // Placeholder, will be replaced
        values.push('0')
        functionSignatures.push('swapOwner(address,address,address)')
        datas.push(
            hre.ethers.utils.defaultAbiCoder.encode(['address', 'address', 'address'], [prevOwner, oldOwner, newOwner])
        )
        operations.push(0) // Operation.Call

        // Update working owners
        const oldIndex = workingOwners.findIndex((o) => o.toLowerCase() === oldOwner.toLowerCase())
        workingOwners[oldIndex] = newOwner
    }

    // Note: We never have both remainingToAdd > 0 AND remainingToRemove > 0
    // because swapCount = min(ownersToRemove.length, ownersToAdd.length)
    const remainingToAdd = ownersToAdd.slice(swapCount)
    const remainingToRemove = ownersToRemove.slice(swapCount)

    // Step 2-uno: Add remaining new owners (only if newOwners.length > currentOwners.length)
    if (remainingToAdd.length > 0) {
        for (let i = 0; i < remainingToAdd.length; i++) {
            const newOwner = remainingToAdd[i]
            const isLastAdd = i === remainingToAdd.length - 1
            // Use final threshold on last add, otherwise use 1 (safe intermediate value)
            const addThreshold = isLastAdd ? newThreshold : 1

            console.log(
                `\n➕ Add owner: ${newOwner} (threshold: ${addThreshold}${isLastAdd ? ' - final' : ' - intermediate'})`
            )

            targets.push('SAFE_ADDRESS')
            values.push('0')
            functionSignatures.push('addOwnerWithThreshold(address,uint256)')
            datas.push(hre.ethers.utils.defaultAbiCoder.encode(['address', 'uint256'], [newOwner, addThreshold]))
            operations.push(0) // Operation.Call

            // Update working owners (new owners are added at the beginning of the list)
            workingOwners = [newOwner, ...workingOwners]
        }
    }

    // Step 2-bis (exclusive with step 2-uno): Remove remaining old owners (only if newOwners.length < currentOwners.length)
    if (remainingToRemove.length > 0) {
        for (let i = 0; i < remainingToRemove.length; i++) {
            const oldOwner = remainingToRemove[i]
            const prevOwner = findPrevOwner(workingOwners, oldOwner)
            const isLastRemove = i === remainingToRemove.length - 1
            // Use final threshold on last remove, otherwise use 1 (safe intermediate value)
            const removeThreshold = isLastRemove ? newThreshold : 1

            console.log(
                `\n➖ Remove owner: ${oldOwner} (prev: ${prevOwner}, threshold: ${removeThreshold}${isLastRemove ? ' - final' : ' - intermediate'})`
            )

            targets.push('SAFE_ADDRESS')
            values.push('0')
            functionSignatures.push('removeOwner(address,address,uint256)')
            datas.push(
                hre.ethers.utils.defaultAbiCoder.encode(
                    ['address', 'address', 'uint256'],
                    [prevOwner, oldOwner, removeThreshold]
                )
            )
            operations.push(0) // Operation.Call

            // Update working owners
            workingOwners = workingOwners.filter((o) => o.toLowerCase() !== oldOwner.toLowerCase())
        }
    }

    // Step 4: Set final threshold only if:
    // - No adds and no removes happened (only swaps or no owner changes)
    // - AND threshold needs to change
    if (remainingToAdd.length === 0 && remainingToRemove.length === 0 && currentThreshold !== newThreshold) {
        console.log(`\n🔢 Change threshold: ${currentThreshold} -> ${newThreshold}`)

        targets.push('SAFE_ADDRESS')
        values.push('0')
        functionSignatures.push('changeThreshold(uint256)')
        datas.push(hre.ethers.utils.defaultAbiCoder.encode(['uint256'], [newThreshold]))
        operations.push(0) // Operation.Call
    }

    // Calculate estimated gas using conservative formula
    const numOperations = targets.length
    const estimatedGas = GAS_ESTIMATES.baseOverhead + numOperations * GAS_ESTIMATES.gasPerOperation

    console.log(`\n✅ Total operations: ${numOperations}`)
    console.log(`⛽ Estimated gas (conservative): ${estimatedGas}`)

    // Generate LZ options bytes using estimated gas
    const optionsBytes = Options.newOptions().addExecutorLzReceiveOption(estimatedGas, 0).toHex().toString()

    return { targets, values, functionSignatures, datas, operations, optionsBytes }
}

// Usage: npx hardhat task:getSafeOwnerChangeArgs --safe 0x... --new-owners "0x...,0x...,0x..." --threshold 2 --network gateway-mainnet
task('task:getSafeOwnerChangeArgs', 'Computes sendRemoteProposal arguments to change Safe owners and threshold')
    .addParam('safe', 'Address of the deployed Safe contract', undefined, types.string)
    .addParam(
        'newOwners',
        'Comma-separated list of new owner addresses which is replacing the current list of owners',
        undefined,
        types.string
    )
    .addParam('threshold', 'New threshold value', undefined, types.int)
    .setAction(async function (
        taskArgs: { safe: string; newOwners: string; threshold: number },
        hre: HardhatRuntimeEnvironment
    ) {
        const safeAddress = hre.ethers.utils.getAddress(taskArgs.safe)
        const newThreshold = taskArgs.threshold

        // Parse and validate new owners
        const rawOwners = taskArgs.newOwners.split(',').map((addr) => addr.trim())
        const newOwners: string[] = []

        for (const addr of rawOwners) {
            // Check if address is valid
            if (!hre.ethers.utils.isAddress(addr)) {
                throw new Error(`❌ Invalid address format: "${addr}"`)
            }
            newOwners.push(hre.ethers.utils.getAddress(addr)) // Normalize to checksummed format
        }

        // Check for duplicates
        const seenAddresses = new Set<string>()
        for (const addr of newOwners) {
            const lowerAddr = addr.toLowerCase()
            if (seenAddresses.has(lowerAddr)) {
                throw new Error(`❌ Duplicate owner address found: "${addr}"`)
            }
            seenAddresses.add(lowerAddr)
        }

        // Check that we have at least one owner
        if (newOwners.length === 0) {
            throw new Error('❌ At least one owner address must be provided')
        }

        console.log('\n🔐 Safe Owner Change Arguments Generator')
        console.log('=========================================')
        console.log(`Safe address: ${safeAddress}`)
        console.log(`Network: ${hre.network.name}`)

        // Get the Safe contract to read current owners
        const safeAbi = [
            'function getOwners() view returns (address[])',
            'function getThreshold() view returns (uint256)',
        ]
        const safeContract = new hre.ethers.Contract(safeAddress, safeAbi, hre.ethers.provider)

        let currentOwners: string[]
        let currentThreshold: number
        try {
            currentOwners = await safeContract.getOwners()
            currentThreshold = (await safeContract.getThreshold()).toNumber()
            console.log(`\n📋 Current state:`)
            console.log(`   Owners (${currentOwners.length}):`, currentOwners)
            console.log(`   Threshold: ${currentThreshold}`)
        } catch (error) {
            console.error(
                '❌ Failed to read Safe contract. Make sure the address is correct and the network is configured.'
            )
            throw error
        }

        // Check if any changes are needed
        const newOwnersSet = new Set(newOwners.map((o) => o.toLowerCase()))
        const ownersMatch =
            currentOwners.length === newOwners.length && currentOwners.every((o) => newOwnersSet.has(o.toLowerCase()))

        if (ownersMatch && currentThreshold === newThreshold) {
            console.log('\n✅ No changes needed. Current state matches desired state.')
            return
        }

        // Compute the owner changes
        const result = await computeOwnerChanges(hre, currentOwners, currentThreshold, newOwners, newThreshold)

        // Replace placeholders with actual Safe address
        const finalTargets = result.targets.map(() => safeAddress)

        // Helper to format array without quotes
        const formatArray = (arr: (string | number)[]) => '[\n' + arr.map((v) => `  ${v}`).join('\n') + '\n]'

        // Output the arguments
        console.log('\n' + '='.repeat(80))
        console.log('📤 sendRemoteProposal ARGUMENTS (to be used via the Aragon DAO)')
        console.log('='.repeat(80))

        console.log('\n// targets (address[]):')
        console.log(formatArray(finalTargets))

        console.log('\n// values (uint256[]):')
        console.log(formatArray(result.values))

        console.log('\n// functionSignatures (string[]):')
        console.log(formatArray(result.functionSignatures))

        console.log('\n// datas (bytes[]) - ABI encoded parameters:')
        console.log(formatArray(result.datas))

        console.log('\n// operations (Operation[]) - 0 = Call, 1 = DelegateCall:')
        console.log(formatArray(result.operations))

        console.log('\n// options (bytes) - LayerZero execution options:')
        console.log(result.optionsBytes)

        return {
            targets: finalTargets,
            values: result.values,
            functionSignatures: result.functionSignatures,
            datas: result.datas,
            operations: result.operations,
            options: result.optionsBytes,
        }
    })
