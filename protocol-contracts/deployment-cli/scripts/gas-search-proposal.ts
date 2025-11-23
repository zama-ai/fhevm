import { Options } from "@layerzerolabs/lz-v2-utilities";
import { Command } from "commander";
import dotenv from "dotenv";
import {
    type Abi,
    type Address,
    encodeFunctionData,
    getAddress,
    getContract,
} from "viem";
import { FORKS_CONFIG } from "../src/config/forks-config.js";
import {
    type AnvilForks,
    startAnvilForks,
    stopAnvilForks,
} from "./utils/anvil-setup.js";
import {
    type DeploymentAddresses,
    loadContractABIs,
    loadDeploymentAddresses,
    validateDeployment,
} from "./utils/deployment-loader.js";
import {
    executeProposal,
    type ProposalData,
    sendProposal,
} from "./utils/proposal-helpers.js";

dotenv.config();

interface GasSearchOptions {
    minGas: number;
    maxGas: number;
    target?: string;
    // Custom proposal data (alternative to using pre-built targets)
    targets?: string;
    values?: string;
    functionSignatures?: string;
    datas?: string;
    operations?: string;
}

interface CustomProposalData {
    targets: string[];
    values: bigint[] | number[];
    functionSignatures: string[];
    datas: string[];
    operations: number[];
}

function parseCustomProposalData(
    options: GasSearchOptions,
): CustomProposalData {
    if (
        !options.targets ||
        !options.values ||
        !options.functionSignatures ||
        !options.datas ||
        !options.operations
    ) {
        throw new Error(
            "Custom proposal requires all of: --targets, --values, --function-signatures, --datas, --operations",
        );
    }

    try {
        const targetsRaw = JSON.parse(options.targets) as string[];
        const valuesRaw = JSON.parse(options.values) as (
            | number
            | string
            | bigint
        )[];
        const functionSignatures = JSON.parse(
            options.functionSignatures,
        ) as string[];
        const datas = JSON.parse(options.datas) as string[];
        const operationsRaw = JSON.parse(options.operations) as number[];

        // Validate arrays have same length
        if (
            targetsRaw.length !== valuesRaw.length ||
            targetsRaw.length !== functionSignatures.length ||
            targetsRaw.length !== datas.length ||
            targetsRaw.length !== operationsRaw.length
        ) {
            throw new Error("All proposal arrays must have the same length");
        }

        // Normalize and validate
        const targets = targetsRaw.map((t) => getAddress(t));
        const values = valuesRaw.map((v) =>
            typeof v === "bigint" ? v : BigInt(v),
        );
        const operations = operationsRaw.map((op) => {
            if (op !== 0 && op !== 1)
                throw new Error("operations entries must be 0 or 1");
            return op;
        });

        return { targets, values, functionSignatures, datas, operations };
    } catch (error) {
        throw new Error(`Failed to parse custom proposal data: ${error}`);
    }
}

/**
 * Build a proposal based on the target function name or custom data
 * if `customData` is provided, use it to build the proposal.
 * if `target` is provided, use it to build the proposal.
 * if neither `customData` nor `target` is provided, throws an error.
 */
function buildProposal(
    target: string | undefined,
    customData: CustomProposalData | undefined,
    addresses: DeploymentAddresses,
    pauserSetAbi: Abi,
): ProposalData {
    if (customData) {
        return {
            targets: customData.targets,
            values: customData.values,
            functionSignatures: customData.functionSignatures,
            datas: customData.datas,
            operations: customData.operations,
        };
    }

    if (!target) {
        throw new Error(
            "Either --target or custom proposal data must be provided",
        );
    }

    switch (target) {
        case "addPauser": {
            // Example: Add a test pauser
            const testPauser = "0x0000000000000000000000000000000000000001";
            const pauserSetAddress = addresses.PAUSER_SET_GATEWAY;

            const addPauserData = encodeFunctionData({
                abi: pauserSetAbi,
                functionName: "addPauser",
                args: [testPauser as Address],
            });

            return {
                targets: [pauserSetAddress],
                values: [0],
                functionSignatures: [""],
                datas: [addPauserData],
                operations: [0], // Call operation
            };
        }
        default:
            throw new Error(
                `Unknown target: ${target}. Supported targets: addPauser`,
            );
    }
}

/**
 * Test if a proposal executes successfully with the given gas limit
 * Returns { success: boolean, gasUsed: bigint }
 */
async function testProposalWithGas(
    forks: AnvilForks,
    proposal: ProposalData,
    gasLimit: number,
    addresses: DeploymentAddresses,
    abis: Awaited<ReturnType<typeof loadContractABIs>>,
    originEid: number,
): Promise<{ success: boolean; gasUsed: bigint }> {
    const { l1, l2 } = forks;

    // Take snapshot before execution
    const l1Snapshot = await l1.client.snapshot();
    const l2Snapshot = await l2.client.snapshot();

    try {
        // Build options with the test gas limit
        const options = Options.newOptions().addExecutorLzReceiveOption(
            gasLimit,
            0,
        );

        const governanceOAppSender = getContract({
            address: addresses.GOVERNANCE_OAPP_SENDER as Address,
            abi: abis.governanceOAppSenderAbi,
            client: l1.client,
        });

        const governanceOAppReceiver = getContract({
            address: addresses.GOVERNANCE_OAPP_RECEIVER as Address,
            abi: abis.governanceOAppReceiverAbi,
            client: l2.client,
        });

        const daoAddress = getAddress(addresses.DAO_ADDRESS) as Address;

        await sendProposal(
            l1.client,
            governanceOAppSender,
            proposal,
            options,
            daoAddress,
        );

        const receipt = await executeProposal(
            l2.client,
            governanceOAppReceiver,
            proposal,
            options,
            originEid,
            addresses.GOVERNANCE_OAPP_SENDER,
        );

        const success = receipt.status === "success";
        const gasUsed = receipt.gasUsed;

        return { success, gasUsed };
    } catch (_error) {
        return { success: false, gasUsed: 0n };
    } finally {
        await l1.client.revert({ id: l1Snapshot });
        await l2.client.revert({ id: l2Snapshot });
    }
}

/**
 * Perform binary search to find minimum gas limit required for the proposal to succeed.
 * The `gasUsed` value is not necessarily the minimum gas limit required for the transaction to succeed;
 * as some transactions require more gas than the final gas used.
 */
async function binarySearchGas(
    forks: AnvilForks,
    proposal: ProposalData,
    userMinGas: number,
    userMaxGas: number,
    tolerance: number,
    addresses: DeploymentAddresses,
    abis: Awaited<ReturnType<typeof loadContractABIs>>,
    originEid: number,
): Promise<number> {
    if (userMinGas <= 0 || userMaxGas <= 0) {
        throw new Error("min-gas and max-gas must be positive integers");
    }
    if (userMinGas > userMaxGas) {
        throw new Error("min-gas must be less than or equal to max-gas");
    }

    console.log("\n🔍 Calibrating from maxGas to measure consumed gas...\n");

    // 1) Ensure the upper bound succeeds and measure consumed gas
    const upper = await testProposalWithGas(
        forks,
        proposal,
        userMaxGas,
        addresses,
        abis,
        originEid,
    );
    if (!upper.success) {
        throw new Error(
            `Execution failed even at max-gas=${userMaxGas.toLocaleString()}. Increase max-gas and retry.`,
        );
    }
    const consumed = Number(upper.gasUsed);
    console.log(
        `✅ Success at maxGas=${userMaxGas.toLocaleString()} (consumed: ${consumed.toLocaleString()})`,
    );

    // 2) Start from consumedGas; ensuring the tx passes at this gas limit
    let low = Math.max(userMinGas, consumed);
    let high = userMaxGas;
    let bestGas = userMaxGas;

    console.log(`\n⚡ Probing at consumed gas = ${low.toLocaleString()}`);
    const probe = await testProposalWithGas(
        forks,
        proposal,
        low,
        addresses,
        abis,
        originEid,
    );
    if (probe.success) {
        console.log(`   ✅ Succeeds at consumed gas`);
        bestGas = low;
        high = low;
        return bestGas;
    } else {
        console.log(
            `   ❌ Fails at consumed gas. Searching in [${(low + 1).toLocaleString()}, ${high.toLocaleString()}]...`,
        );
        low = low + 1;
    }

    while (high - low > tolerance) {
        const mid = Math.floor((low + high) / 2);
        console.log(`Testing gas limit: ${mid.toLocaleString()}`);

        const result = await testProposalWithGas(
            forks,
            proposal,
            mid,
            addresses,
            abis,
            originEid,
        );

        if (result.success) {
            bestGas = mid;
            high = mid;
            console.log(
                `✅ Success with ${mid.toLocaleString()} (used: ${result.gasUsed.toLocaleString()})`,
            );
        } else {
            low = mid + 1;
            console.log(`❌ Failed with ${mid.toLocaleString()}`);
        }
    }

    return bestGas;
}

async function main(options: GasSearchOptions): Promise<void> {
    console.log("⛽ Gas Search Proposal Tool\n");

    if (!process.env.TESTNET_ETHEREUM_RPC_URL) {
        throw new Error(
            "TESTNET_ETHEREUM_RPC_URL environment variable must be set",
        );
    }
    if (!process.env.TESTNET_GATEWAY_RPC_URL) {
        throw new Error(
            "TESTNET_GATEWAY_RPC_URL environment variable must be set",
        );
    }

    const addresses = loadDeploymentAddresses(
        FORKS_CONFIG.DEPLOYMENT_STATE_FILE,
    );
    validateDeployment(addresses);

    // Start anvil forks
    const forks = await startAnvilForks({
        l1RpcUrl: process.env.TESTNET_ETHEREUM_RPC_URL,
        l2RpcUrl: process.env.TESTNET_GATEWAY_RPC_URL,
        l1ChainId: FORKS_CONFIG.SEPOLIA_CHAIN_ID,
        l2ChainId: FORKS_CONFIG.GATEWAY_CHAIN_ID,
        l1Port: FORKS_CONFIG.ANVIL_L1_PORT,
        l2Port: FORKS_CONFIG.ANVIL_GATEWAY_PORT,
    });

    try {
        const abis = await loadContractABIs();

        const isCustomProposal = !!(
            options.targets ||
            options.values ||
            options.functionSignatures ||
            options.datas ||
            options.operations
        );

        let customData: CustomProposalData | undefined;

        if (isCustomProposal) {
            console.log("🎯 Building custom proposal from provided data");
            customData = parseCustomProposalData(options);
        } else if (options.target) {
            console.log(`🎯 Building proposal for target: ${options.target}`);
        } else {
            throw new Error(
                "Either --target or custom proposal data must be provided",
            );
        }

        const proposal = buildProposal(
            options.target,
            customData,
            addresses,
            abis.pauserSetAbi,
        );
        console.log(proposal);

        const minGas = await binarySearchGas(
            forks,
            proposal,
            options.minGas,
            options.maxGas,
            10_000,
            addresses,
            abis,
            FORKS_CONFIG.ORIGIN_EID,
        );

        console.log("━".repeat(60));
        console.log(
            `\n✨ Minimum gas limit found: ${minGas.toLocaleString()}\n`,
        );
        console.log("━".repeat(60));
    } finally {
        await stopAnvilForks(forks);
    }
}

const program = new Command();
program
    .name("gas-search-proposal")
    .description(
        "Find the minimum gas limit required for successful proposal execution",
    )
    .requiredOption("--min-gas <number>", "Minimum gas to test", (value) =>
        parseInt(value, 10),
    )
    .requiredOption("--max-gas <number>", "Maximum gas to test", (value) =>
        parseInt(value, 10),
    )
    .option(
        "--target <function_name>",
        "Pre-built target function name (e.g., addPauser). Mutually exclusive with custom proposal options.",
    )
    .option(
        "--targets <json>",
        "JSON array of target addresses for custom proposal",
    )
    .option("--values <json>", "JSON array of values for custom proposal")
    .option(
        "--function-signatures <json>",
        "JSON array of function signatures for custom proposal",
    )
    .option("--datas <json>", "JSON array of calldata for custom proposal")
    .option(
        "--operations <json>",
        "JSON array of operations (0=Call, 1=DelegateCall) for custom proposal",
    )
    .action(async (options: GasSearchOptions) => {
        try {
            await main(options);
        } catch (error) {
            console.error("\n❌ Error:", error);
            process.exit(1);
        }
    });

program.parse();
