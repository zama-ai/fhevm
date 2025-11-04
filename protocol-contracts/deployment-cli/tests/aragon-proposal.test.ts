import { afterAll, beforeAll, describe, expect, test } from "bun:test";
import { spawn } from "node:child_process";
import { readFileSync } from "node:fs";
import path from "node:path";
import { setTimeout as delay } from "node:timers/promises";
import dotenv from "dotenv";
import {
    Contract,
    getAddress,
    Interface,
    type InterfaceAbi,
    JsonRpcProvider,
    parseEther,
} from "ethers";
import { resolveProjectRoot } from "../src/utils/project-paths.js";

dotenv.config();

const TEST_CONFIG = {
    ANVIL_URL: "http://127.0.0.1:8545",
    ANVIL_PORT: 8545,
    SEPOLIA_CHAIN_ID: 11155111,
    ADMIN_EXECUTOR_BALANCE: parseEther("10").toString(),
    DEPLOYMENT_STATE_FILE: "zama-protocol-testnet-v0-9.addresses.json",
} as const;

const ARAGON_CONSTANTS = {
    EMPTY_METADATA: "0x",
    NO_FAILURE_MAP: 0,
} as const;

interface DeploymentAddresses {
    readonly [key: string]: string;
    DAO_ADDRESS: string;
    PAUSER_SET_HOST: string;
    PAUSER_SET_WRAPPER: string;
}

interface DaoAction {
    to: string;
    value: bigint;
    data: string;
}

let addresses: DeploymentAddresses;
let provider: JsonRpcProvider;
let hostContract: Contract;
let adminPlugin: Contract;
let anvilProcess: ReturnType<typeof spawn> | undefined;
let pauserSetInterface: Interface | null = null;
const ADMIN_EXECUTOR_ADDRESS = process.env.DAO_ADMIN_EXECUTOR || "";
const ADMIN_PLUGIN_ADDRESS = process.env.DAO_ADMIN_PLUGIN || "";

async function waitForRpcReady(
    rpcProvider: JsonRpcProvider,
    attempts = 100,
    intervalMs = 200,
): Promise<void> {
    for (let attempt = 0; attempt < attempts; attempt += 1) {
        try {
            await rpcProvider.getBlockNumber();
            return;
        } catch (error) {
            if (attempt === attempts - 1) throw error;
            await delay(intervalMs);
        }
    }
}

function getPauserSetInterface(): Interface {
    if (pauserSetInterface) {
        return pauserSetInterface;
    }

    const pauserSetArtifactPath = path.resolve(
        resolveProjectRoot(),
        "host-contracts",
        "artifacts",
        "contracts",
        "immutable",
        "PauserSet.sol",
        "PauserSet.json",
    );
    const pauserSetArtifact = JSON.parse(
        readFileSync(pauserSetArtifactPath, "utf-8"),
    ) as {
        abi: InterfaceAbi;
    };

    pauserSetInterface = new Interface(pauserSetArtifact.abi);
    return pauserSetInterface;
}

function buildAddPauserAction(
    hostAddress: string,
    pauserAddress: string,
): DaoAction {
    const hostInterface = getPauserSetInterface();
    const encoded = hostInterface.encodeFunctionData("addPauser", [
        getAddress(pauserAddress),
    ]);
    return {
        to: getAddress(hostAddress),
        value: 0n,
        data: encoded,
    };
}

function buildSwapPauserAction(
    hostAddress: string,
    oldPauserAddress: string,
    newPauserAddress: string,
): DaoAction {
    const hostInterface = getPauserSetInterface();
    const encoded = hostInterface.encodeFunctionData("swapPauser", [
        getAddress(oldPauserAddress),
        getAddress(newPauserAddress),
    ]);
    return {
        to: getAddress(hostAddress),
        value: 0n,
        data: encoded,
    };
}

function buildRemovePauserAction(
    hostAddress: string,
    pauserAddress: string,
): DaoAction {
    const hostInterface = getPauserSetInterface();
    const encoded = hostInterface.encodeFunctionData("removePauser", [
        getAddress(pauserAddress),
    ]);
    return {
        to: getAddress(hostAddress),
        value: 0n,
        data: encoded,
    };
}

async function executeViaAdminPlugin(actions: DaoAction[]): Promise<void> {
    const tx = await adminPlugin.executeProposal(
        ARAGON_CONSTANTS.EMPTY_METADATA,
        actions,
        ARAGON_CONSTANTS.NO_FAILURE_MAP,
    );
    await tx.wait();
}

beforeAll(async () => {
    const addressesPath = path.resolve(
        resolveProjectRoot(),
        "protocol-contracts",
        "deployment-cli",
        "deployment-state",
        TEST_CONFIG.DEPLOYMENT_STATE_FILE,
    );
    const raw = readFileSync(addressesPath, "utf-8");
    addresses = JSON.parse(raw) as DeploymentAddresses;

    // Fork Sepolia using Anvil
    anvilProcess = spawn(
        "anvil",
        [
            "--fork-url",
            process.env.TESTNET_ETHEREUM_RPC_URL || "",
            "--chain-id",
            TEST_CONFIG.SEPOLIA_CHAIN_ID.toString(),
            "--port",
            TEST_CONFIG.ANVIL_PORT.toString(),
        ],
        {
            stdio: "ignore",
        },
    );

    provider = new JsonRpcProvider(TEST_CONFIG.ANVIL_URL);
    await waitForRpcReady(provider);

    const pauserSetAbi = getPauserSetInterface();
    hostContract = new Contract(
        addresses.PAUSER_SET_HOST,
        pauserSetAbi,
        provider,
    );

    // Impersonate the admin executor address to have permission to execute via Admin plugin
    await provider.send("anvil_impersonateAccount", [ADMIN_EXECUTOR_ADDRESS]);
    await provider.send("anvil_setBalance", [
        ADMIN_EXECUTOR_ADDRESS,
        TEST_CONFIG.ADMIN_EXECUTOR_BALANCE,
    ]);

    // Verify permissions before setting up admin plugin
    const daoAbi = [
        "function hasPermission(address _where, address _who, bytes32 _permissionId, bytes memory _data) external view returns (bool)",
    ];
    const dao = new Contract(addresses.DAO_ADDRESS, daoAbi, provider);
    const adminPluginContract = new Contract(
        ADMIN_PLUGIN_ADDRESS,
        [
            "function EXECUTE_PROPOSAL_PERMISSION_ID() external view returns (bytes32)",
        ],
        provider,
    );
    const executeProposalPermissionId =
        await adminPluginContract.EXECUTE_PROPOSAL_PERMISSION_ID();

    const hasPermission = await dao.hasPermission(
        ADMIN_PLUGIN_ADDRESS,
        ADMIN_EXECUTOR_ADDRESS,
        executeProposalPermissionId,
        ARAGON_CONSTANTS.EMPTY_METADATA,
    );

    console.log(
        `Address ${ADMIN_EXECUTOR_ADDRESS} has EXECUTE_PROPOSAL_PERMISSION on Admin plugin: ${hasPermission}`,
    );

    if (!hasPermission) {
        throw new Error(
            "Admin executor address does not have the required EXECUTE_PROPOSAL_PERMISSION!",
        );
    }

    // Setup Admin Plugin with minimal ABI using the admin executor signer
    const adminPluginAbi = [
        "function executeProposal(bytes calldata _metadata, tuple(address to, uint256 value, bytes data)[] calldata _actions, uint256 _allowFailureMap) external",
    ];
    const adminExecutorSigner = await provider.getSigner(
        ADMIN_EXECUTOR_ADDRESS,
    );
    adminPlugin = new Contract(
        ADMIN_PLUGIN_ADDRESS,
        adminPluginAbi,
        adminExecutorSigner,
    );
});

afterAll(async () => {
    if (anvilProcess) {
        anvilProcess.kill();
        await new Promise((resolve) => {
            anvilProcess?.once("exit", resolve);
            setTimeout(resolve, 250).unref();
        });
    }
});

describe("Aragon DAO pauser management via Admin plugin", () => {
    test("addPauser: adds a new pauser to the pauser set through Admin plugin", async () => {
        const newPauser = getAddress(addresses.PAUSER_SET_WRAPPER);
        const action = buildAddPauserAction(
            addresses.PAUSER_SET_HOST,
            newPauser,
        );

        // Check pauser doesn't exist yet
        const before = await hostContract.isPauser(newPauser);
        expect(before).toBe(false);

        // Execute via Admin plugin (no voting required)
        await executeViaAdminPlugin([action]);

        // Verify pauser was added
        const after = await hostContract.isPauser(newPauser);
        expect(after).toBe(true);
    });

    test("swapPauser: replaces one pauser with another through Admin plugin", async () => {
        // Use dummy addresses for testing
        const oldPauser = "0x0000000000000000000000000000000000000001";
        const newPauser = "0x0000000000000000000000000000000000000002";

        // First, add the old pauser
        const addAction = buildAddPauserAction(
            addresses.PAUSER_SET_HOST,
            oldPauser,
        );
        await executeViaAdminPlugin([addAction]);

        // Verify old pauser was added
        const oldPauserAdded = await hostContract.isPauser(oldPauser);
        expect(oldPauserAdded).toBe(true);

        // Now swap old pauser with new pauser
        const swapAction = buildSwapPauserAction(
            addresses.PAUSER_SET_HOST,
            oldPauser,
            newPauser,
        );
        await executeViaAdminPlugin([swapAction]);

        // Verify old pauser was removed and new pauser was added
        const oldPauserRemoved = await hostContract.isPauser(oldPauser);
        const newPauserAdded = await hostContract.isPauser(newPauser);
        expect(oldPauserRemoved).toBe(false);
        expect(newPauserAdded).toBe(true);
    });

    test("removePauser: removes a pauser from the set through Admin plugin", async () => {
        // Use a dummy address for the pauser to remove
        const pauserToRemove = "0x0000000000000000000000000000000000000003";

        // First, add the pauser
        const addAction = buildAddPauserAction(
            addresses.PAUSER_SET_HOST,
            pauserToRemove,
        );
        await executeViaAdminPlugin([addAction]);

        // Verify pauser was added
        const pauserAdded = await hostContract.isPauser(pauserToRemove);
        expect(pauserAdded).toBe(true);

        // Now remove the pauser
        const removeAction = buildRemovePauserAction(
            addresses.PAUSER_SET_HOST,
            pauserToRemove,
        );
        await executeViaAdminPlugin([removeAction]);

        // Verify pauser was removed
        const pauserRemoved = await hostContract.isPauser(pauserToRemove);
        expect(pauserRemoved).toBe(false);
    });
});
