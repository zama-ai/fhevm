import { afterAll, beforeAll, describe, expect, test } from "bun:test";
import { AdminABI } from "@aragon/admin-plugin-artifacts";
import { DAOABI } from "@aragon/osx-artifacts";
import dotenv from "dotenv";
import {
    type Abi,
    type Address,
    createWalletClient,
    encodeFunctionData,
    getAddress,
    getContract,
    http,
    parseEther,
    type WalletClient,
} from "viem";
import {
    type DeploymentAddresses,
    loadContractABIs,
    loadDeploymentAddresses,
    validateDeployment,
} from "../scripts/utils/deployment-loader.js";
import { FORKS_CONFIG } from "../src/config/forks-config.js";
import type { ExtendedTestClient } from "./types.js";
import { type AnvilProcess, startAnvilFork, stopAnvil } from "./utils/anvil.js";

dotenv.config();

let addresses: DeploymentAddresses;
let anvilProcess: { proc: AnvilProcess; client: ExtendedTestClient };
let pauserSetAbi: Abi;
let adminPluginSigner: WalletClient;
const ADMIN_EXECUTOR_ADDRESS = (process.env.DAO_ADMIN_EXECUTOR ||
    "0x") as `0x${string}`;
const ADMIN_PLUGIN_ADDRESS = (process.env.DAO_ADMIN_PLUGIN ||
    "0x") as `0x${string}`;

async function executeViaAdminPlugin(
    actions: { to: `0x${string}`; value: bigint; data: `0x${string}` }[],
): Promise<void> {
    const adminPlugin = getContract({
        address: ADMIN_PLUGIN_ADDRESS as Address,
        abi: AdminABI,
        client: adminPluginSigner,
    });

    //@ts-expect-error - unknown type error here
    const hash = await adminPlugin.write.executeProposal(["0x", actions, 0n]);
    await anvilProcess.client.waitForTransactionReceipt({ hash });
}

beforeAll(async () => {
    addresses = loadDeploymentAddresses(FORKS_CONFIG.DEPLOYMENT_STATE_FILE);
    validateDeployment(addresses);

    anvilProcess = await startAnvilFork({
        forkUrl: process.env.TESTNET_ETHEREUM_RPC_URL || "",
        chainId: FORKS_CONFIG.SEPOLIA_CHAIN_ID,
        port: FORKS_CONFIG.ANVIL_L1_PORT,
    });

    pauserSetAbi = (await loadContractABIs()).pauserSetAbi;

    // Impersonate the admin executor address to have permission to execute via Admin plugin
    const checksummed = getAddress(ADMIN_EXECUTOR_ADDRESS) as Address;
    await anvilProcess.client.impersonateAccount({ address: checksummed });
    await anvilProcess.client.setBalance({
        address: checksummed,
        value: parseEther("10"),
    });

    adminPluginSigner = createWalletClient({
        account: checksummed,
        chain: anvilProcess.client.chain,
        transport: http(anvilProcess.client.chain?.rpcUrls.default.http[0]),
    });

    // Verify permissions before setting up admin plugin
    const dao = getContract({
        address: addresses.DAO_ADDRESS as Address,
        abi: DAOABI,
        client: anvilProcess.client,
    });

    const adminPluginContract = getContract({
        address: ADMIN_PLUGIN_ADDRESS as Address,
        abi: AdminABI,
        client: anvilProcess.client,
    });

    const executeProposalPermissionId =
        (await adminPluginContract.read.EXECUTE_PROPOSAL_PERMISSION_ID()) as `0x${string}`;
    const hasPermission = await dao.read.hasPermission([
        ADMIN_PLUGIN_ADDRESS,
        ADMIN_EXECUTOR_ADDRESS,
        executeProposalPermissionId,
        "0x",
    ]);

    if (!hasPermission) {
        throw new Error(
            "Admin executor address does not have the required EXECUTE_PROPOSAL_PERMISSION!",
        );
    }
});

afterAll(async () => {
    await stopAnvil(anvilProcess?.proc);
});

describe("Aragon DAO pauser management via Admin plugin", () => {
    test("addPauser: adds a new pauser to the pauser set through Admin plugin", async () => {
        const newPauser = getAddress(addresses.PAUSER_SET_WRAPPER);
        const hostContract = getContract({
            address: addresses.PAUSER_SET_HOST as Address,
            abi: pauserSetAbi,
            client: anvilProcess.client,
        });

        const before = await hostContract.read.isPauser([newPauser as Address]);
        expect(before).toBe(false);

        // Execute via Admin plugin (no voting required)
        await executeViaAdminPlugin([
            {
                to: getAddress(addresses.PAUSER_SET_HOST),
                value: 0n,
                data: encodeFunctionData({
                    abi: pauserSetAbi,
                    functionName: "addPauser",
                    args: [newPauser],
                }),
            },
        ]);

        // Verify pauser was added
        const after = await hostContract.read.isPauser([newPauser as Address]);
        expect(after).toBe(true);
    });

    test("swapPauser: replaces one pauser with another through Admin plugin", async () => {
        const oldPauser =
            "0x0000000000000000000000000000000000000003" as Address;
        const newPauser =
            "0x0000000000000000000000000000000000000004" as Address;

        const hostContract = getContract({
            address: addresses.PAUSER_SET_HOST as Address,
            abi: pauserSetAbi,
            client: anvilProcess.client,
        });

        await executeViaAdminPlugin([
            {
                to: getAddress(addresses.PAUSER_SET_HOST),
                value: 0n,
                data: encodeFunctionData({
                    abi: pauserSetAbi,
                    functionName: "addPauser",
                    args: [oldPauser],
                }),
            },
        ]);

        const oldPauserAdded = await hostContract.read.isPauser([oldPauser]);
        expect(oldPauserAdded).toBe(true);

        await executeViaAdminPlugin([
            {
                to: getAddress(addresses.PAUSER_SET_HOST),
                value: 0n,
                data: encodeFunctionData({
                    abi: pauserSetAbi,
                    functionName: "swapPauser",
                    args: [oldPauser, newPauser],
                }),
            },
        ]);

        const oldPauserRemoved = await hostContract.read.isPauser([oldPauser]);
        const newPauserAdded = await hostContract.read.isPauser([newPauser]);
        expect(oldPauserRemoved).toBe(false);
        expect(newPauserAdded).toBe(true);
    });

    test("removePauser: removes a pauser from the set through Admin plugin", async () => {
        const pauserToRemove =
            "0x0000000000000000000000000000000000000005" as Address;
        const hostContract = getContract({
            address: addresses.PAUSER_SET_HOST as Address,
            abi: pauserSetAbi,
            client: anvilProcess.client,
        });

        // First, add the pauser
        await executeViaAdminPlugin([
            {
                to: getAddress(addresses.PAUSER_SET_HOST),
                value: 0n,
                data: encodeFunctionData({
                    abi: pauserSetAbi,
                    functionName: "addPauser",
                    args: [pauserToRemove],
                }),
            },
        ]);

        const pauserAdded = await hostContract.read.isPauser([pauserToRemove]);
        expect(pauserAdded).toBe(true);

        await executeViaAdminPlugin([
            {
                to: getAddress(addresses.PAUSER_SET_HOST),
                value: 0n,
                data: encodeFunctionData({
                    abi: pauserSetAbi,
                    functionName: "removePauser",
                    args: [pauserToRemove],
                }),
            },
        ]);

        // Verify pauser was removed
        const pauserRemoved = await hostContract.read.isPauser([
            pauserToRemove,
        ]);
        expect(pauserRemoved).toBe(false);
    });
});
