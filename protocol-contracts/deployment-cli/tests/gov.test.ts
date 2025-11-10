import { afterAll, beforeAll, describe, expect, it } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { EndpointId } from "@layerzerolabs/lz-definitions";
import { Options } from "@layerzerolabs/lz-v2-utilities";
import dotenv from "dotenv";
import {
    type Abi,
    type Address,
    createWalletClient,
    encodeFunctionData,
    type GetContractReturnType,
    getAddress,
    getContract,
    http,
    parseEther,
} from "viem";
import { FORKS_CONFIG } from "../src/config/forks-config.js";
import { resolveProjectRoot } from "../src/utils/project-paths.js";
import type { ExtendedTestClient } from "./types.js";
import {
    type AnvilProcess,
    startAnvilFork,
    stopAnvil,
    waitForRpcReady,
} from "./utils/anvil.js";
import { deliverToReceiver, encodeGovernanceMessage } from "./utils/helpers.js";

dotenv.config();
interface DeploymentAddresses {
    readonly [key: string]: string;
    DAO_ADDRESS: string;
    GOVERNANCE_OAPP_SENDER: string;
    GOVERNANCE_OAPP_RECEIVER: string;
    ADMIN_MODULE_ADDRESS: string;
    PAUSER_SET_GATEWAY: string;
    PAUSER_SET_WRAPPER: string;
    SAFE_ADDRESS: string;
}

async function loadContractABIs(): Promise<{
    governanceOAppSenderAbi: Abi;
    governanceOAppReceiverAbi: Abi;
    pauserSetAbi: Abi;
}> {
    const senderArtifactPath = path.resolve(
        resolveProjectRoot(),
        "protocol-contracts/governance/artifacts/contracts/GovernanceOAppSender.sol/GovernanceOAppSender.json",
    );
    const governanceOAppSenderAbi = JSON.parse(
        readFileSync(senderArtifactPath, "utf-8"),
    ).abi;
    const receiverArtifactPath = path.resolve(
        resolveProjectRoot(),
        "protocol-contracts/governance/artifacts/contracts/GovernanceOAppReceiver.sol/GovernanceOAppReceiver.json",
    );
    const governanceOAppReceiverAbi = JSON.parse(
        readFileSync(receiverArtifactPath, "utf-8"),
    ).abi;
    const pauserSetArtifactPath = path.resolve(
        resolveProjectRoot(),
        "host-contracts/artifacts/contracts/immutable/PauserSet.sol/PauserSet.json",
    );
    const pauserSetAbi = JSON.parse(
        readFileSync(pauserSetArtifactPath, "utf-8"),
    ).abi;
    return { governanceOAppSenderAbi, governanceOAppReceiverAbi, pauserSetAbi };
}

describe("Post-Deployment Governance Integration", () => {
    let addresses: DeploymentAddresses;
    // Anvil processes
    let anvilProcessL1: { proc: AnvilProcess; client: ExtendedTestClient };
    let anvilProcessGateway: { proc: AnvilProcess; client: ExtendedTestClient };

    // Deployed contract ABIs and addresses
    let governanceOAppSender: GetContractReturnType<
        Abi,
        ExtendedTestClient,
        Address
    >;
    let governanceOAppReceiver: GetContractReturnType<
        Abi,
        ExtendedTestClient,
        Address
    >;
    let pauserSetGateway: GetContractReturnType<
        Abi,
        ExtendedTestClient,
        Address
    >;

    beforeAll(async () => {
        // Validate environment variables
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

        // Start anvil forks from L1 and Gateway
        anvilProcessL1 = await startAnvilFork({
            forkUrl: process.env.TESTNET_ETHEREUM_RPC_URL,
            chainId: FORKS_CONFIG.SEPOLIA_CHAIN_ID,
            port: FORKS_CONFIG.ANVIL_L1_PORT,
        });
        await waitForRpcReady(anvilProcessL1.client);

        anvilProcessGateway = await startAnvilFork({
            forkUrl: process.env.TESTNET_GATEWAY_RPC_URL,
            chainId: FORKS_CONFIG.GATEWAY_CHAIN_ID,
            port: FORKS_CONFIG.ANVIL_GATEWAY_PORT,
        });
        await waitForRpcReady(anvilProcessGateway.client);

        // Load deployment addresses
        const addressesPath = path.resolve(
            resolveProjectRoot(),
            "protocol-contracts/deployment-cli/deployment-state",
            FORKS_CONFIG.DEPLOYMENT_STATE_FILE,
        );
        if (!existsSync(addressesPath)) {
            throw new Error(
                `Deployment addresses file not found at ${addressesPath}. Make sure you ran the deployment CLI first: bun run start deploy --network testnet`,
            );
        }
        addresses = JSON.parse(
            readFileSync(addressesPath, "utf-8"),
        ) as DeploymentAddresses;

        try {
            const {
                governanceOAppSenderAbi: _governanceOAppSenderAbi,
                governanceOAppReceiverAbi: _governanceOAppReceiverAbi,
                pauserSetAbi: _pauserSetAbi,
            } = await loadContractABIs();
            governanceOAppSender = getContract({
                address: addresses.GOVERNANCE_OAPP_SENDER as Address,
                abi: _governanceOAppSenderAbi,
                client: anvilProcessL1.client,
            });
            governanceOAppReceiver = getContract({
                address: addresses.GOVERNANCE_OAPP_RECEIVER as Address,
                abi: _governanceOAppReceiverAbi,
                client: anvilProcessGateway.client,
            });
            pauserSetGateway = getContract({
                address: addresses.PAUSER_SET_GATEWAY as Address,
                abi: _pauserSetAbi,
                client: anvilProcessGateway.client,
            });
        } catch (error) {
            throw new Error(
                `Failed to load contract ABIs: ${error}. Make sure the contracts are compiled and the artifacts are present in the expected paths.`,
            );
        }
    });

    afterAll(async () => {
        await stopAnvil(anvilProcessL1?.proc);
        await stopAnvil(anvilProcessGateway?.proc);
    });

    describe("Cross-chain governance verification", () => {
        it("should verify deployed contracts are properly configured", async () => {
            // GovernanceOAppSender should be owned by DAO
            const senderOwner =
                (await governanceOAppSender.read.owner()) as Address;
            expect(senderOwner).toBe(getAddress(addresses.DAO_ADDRESS));

            // GovernanceOAppReceiver should be owned by the Safe address
            const receiverOwner =
                (await governanceOAppReceiver.read.owner()) as Address;
            expect(receiverOwner).toBe(getAddress(addresses.SAFE_ADDRESS));

            // GovernanceOAppReceiver should have the adminSafeModule set to the AdminModule address
            const adminSafeModule =
                (await governanceOAppReceiver.read.adminSafeModule()) as Address;
            expect(adminSafeModule).toBe(
                getAddress(addresses.ADMIN_MODULE_ADDRESS),
            );
        });

        it("should sendRemoteProposal and execute on Gateway (adds pauser)", async () => {
            const testPauser = "0x0000000000000000000000000000000000000001";
            const pauserSetAddress = addresses.PAUSER_SET_GATEWAY;

            // Initially, the testPauser should not be a valid pauser
            const initialState = await pauserSetGateway.read.isPauser([
                testPauser as Address,
            ]);
            expect(initialState).toBe(false);

            // Build proposal
            const addPauserData = encodeFunctionData({
                abi: pauserSetGateway.abi,
                functionName: "addPauser",
                args: [testPauser as Address],
            });

            const targets = [pauserSetAddress];
            const values = [0];
            const functionSignatures = [""];
            const datas = [addPauserData];
            const operations = [0]; // Call

            // Build options with realistic gas limit for single action
            // Using 300,000 gas to ensure sufficient gas for execution
            const options = Options.newOptions().addExecutorLzReceiveOption(
                200000,
                0,
            );
            const optionsHex = options.toHex().toString();

            const quotedFee =
                (await governanceOAppSender.read.quoteSendCrossChainTransaction(
                    [
                        targets,
                        values,
                        functionSignatures,
                        datas,
                        operations,
                        optionsHex,
                    ],
                )) as bigint;

            // Impersonate DAO to send proposal
            const daoAddress = getAddress(addresses.DAO_ADDRESS) as Address;
            await anvilProcessL1.client.impersonateAccount({
                address: daoAddress,
            });
            await anvilProcessL1.client.setBalance({
                address: daoAddress,
                value: parseEther("10"),
            });
            const daoSigner = createWalletClient({
                account: daoAddress,
                chain: anvilProcessL1.client.chain,
                transport: http(
                    anvilProcessL1.client.chain?.rpcUrls.default.http[0],
                ),
            });
            const senderAsDAO = getContract({
                address: addresses.GOVERNANCE_OAPP_SENDER as Address,
                abi: governanceOAppSender.abi,
                client: daoSigner,
            });

            const hash = await senderAsDAO.write.sendRemoteProposal(
                [
                    targets,
                    values,
                    functionSignatures,
                    datas,
                    operations,
                    optionsHex,
                ],
                { value: quotedFee },
            );

            await anvilProcessL1.client.waitForTransactionReceipt({ hash });
            await anvilProcessL1.client.stopImpersonatingAccount({
                address: daoAddress,
            });

            // Simulate LayerZero delivery by calling receiver.lzReceive from the endpoint on Gateway
            const message = encodeGovernanceMessage(
                targets,
                values,
                functionSignatures,
                datas,
                operations,
            );

            // Origin: Sepolia -> Receiver expects SEPOLIA_V2_TESTNET and sender address
            await deliverToReceiver(
                anvilProcessGateway.client,
                addresses.GOVERNANCE_OAPP_RECEIVER as Address,
                governanceOAppReceiver.abi,
                EndpointId.SEPOLIA_V2_TESTNET,
                addresses.GOVERNANCE_OAPP_SENDER,
                message,
                options,
            );

            // After delivery, the pauser should be added
            const after = await pauserSetGateway.read.isPauser([
                testPauser as Address,
            ]);
            expect(after).toBe(true);
        });

        it("should send batch proposal and execute on Gateway (adds two pausers)", async () => {
            const pauser1 = "0x0000000000000000000000000000000000000003";
            const pauser2 = "0x0000000000000000000000000000000000000004";
            const pauserSetAddress = addresses.PAUSER_SET_GATEWAY;

            const initialState1 = await pauserSetGateway.read.isPauser([
                pauser1 as Address,
            ]);
            const initialState2 = await pauserSetGateway.read.isPauser([
                pauser2 as Address,
            ]);
            expect(initialState1).toBe(false);
            expect(initialState2).toBe(false);

            const addPauser1Data = encodeFunctionData({
                abi: pauserSetGateway.abi,
                functionName: "addPauser",
                args: [pauser1 as Address],
            });
            const addPauser2Data = encodeFunctionData({
                abi: pauserSetGateway.abi,
                functionName: "addPauser",
                args: [pauser2 as Address],
            });

            const targets = [pauserSetAddress, pauserSetAddress];
            const values = [0, 0];
            const functionSignatures = ["", ""];
            const datas = [addPauser1Data, addPauser2Data];
            const operations = [0, 0];

            // Build options with realistic gas limit for batch action (2 operations)
            // Using 400,000 gas to ensure sufficient gas for batch execution
            const options = Options.newOptions().addExecutorLzReceiveOption(
                200000,
                0,
            );
            const optionsHex = options.toHex().toString();

            const quotedFee =
                (await governanceOAppSender.read.quoteSendCrossChainTransaction(
                    [
                        targets,
                        values,
                        functionSignatures,
                        datas,
                        operations,
                        optionsHex,
                    ],
                )) as bigint;

            // Impersonate DAO and send the batch proposal
            const daoAddress = getAddress(addresses.DAO_ADDRESS) as Address;
            await anvilProcessL1.client.impersonateAccount({
                address: daoAddress,
            });
            await anvilProcessL1.client.setBalance({
                address: daoAddress,
                value: parseEther("10"),
            });

            const daoSigner = createWalletClient({
                account: daoAddress,
                chain: anvilProcessL1.client.chain,
                transport: http(
                    anvilProcessL1.client.chain?.rpcUrls.default.http[0],
                ),
            });
            const senderAsDAO = getContract({
                address: addresses.GOVERNANCE_OAPP_SENDER as Address,
                abi: governanceOAppSender.abi,
                client: daoSigner,
            });

            const hash = await senderAsDAO.write.sendRemoteProposal(
                [
                    targets,
                    values,
                    functionSignatures,
                    datas,
                    operations,
                    optionsHex,
                ],
                { value: quotedFee },
            );

            await anvilProcessL1.client.waitForTransactionReceipt({ hash });
            await anvilProcessL1.client.stopImpersonatingAccount({
                address: daoAddress,
            });

            // Simulate LayerZero delivery on the Gateway chain and verify the state change
            const message = encodeGovernanceMessage(
                targets,
                values,
                functionSignatures,
                datas,
                operations,
            );
            await deliverToReceiver(
                anvilProcessGateway.client,
                addresses.GOVERNANCE_OAPP_RECEIVER as Address,
                governanceOAppReceiver.abi,
                EndpointId.SEPOLIA_V2_TESTNET,
                addresses.GOVERNANCE_OAPP_SENDER,
                message,
                options,
            );

            // Verify that pausers are now added on Gateway chain
            const afterState1 = await pauserSetGateway.read.isPauser([
                pauser1 as Address,
            ]);
            const afterState2 = await pauserSetGateway.read.isPauser([
                pauser2 as Address,
            ]);
            expect(afterState1).toBe(true);
            expect(afterState2).toBe(true);
        });
    });
});
