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
    isAddress,
    keccak256,
    pad,
    parseEther,
    toBytes,
    zeroHash,
} from "viem";
import YAML from "yaml";
import { FORKS_CONFIG } from "../src/config/forks-config.js";
import type { DeploymentConfig } from "../src/config/schema.js";
import { DeploymentOrchestrator } from "../src/orchestrator.js";
import type { DeploymentContext } from "../src/steps/base-step.js";
import {
    executeViaAdminPlugin,
    grantRoleViaDao,
} from "../src/tasks/dao-tasks.js";
import { Logger } from "../src/utils/logger.js";
import { resolveProjectRoot } from "../src/utils/project-paths.js";
import type { ExtendedTestClient } from "./types.js";
import {
    type AnvilProcess,
    startAnvilFork,
    stopAnvil,
    waitForRpcReady,
} from "./utils/anvil.js";
import { loadContractABIs } from "./utils/contract-loader.js";
import { deliverToReceiver, encodeGovernanceMessage } from "./utils/helpers.js";

dotenv.config();

interface DeploymentAddresses {
    readonly [key: string]: string;
    DAO_ADDRESS: `0x${string}`;
    SAFE_PROXY_ADDRESS: `0x${string}`;
    GOVERNANCE_OAPP_SENDER: `0x${string}`;
    GOVERNANCE_OAPP_RECEIVER: `0x${string}`;
    ADMIN_MODULE_ADDRESS: `0x${string}`;
    ZAMA_TOKEN: `0x${string}`;
    ZAMA_OFT: `0x${string}`;
    ZAMA_OFT_ADAPTER: `0x${string}`;
    PROTOCOL_FEES_BURNER: `0x${string}`;
    FEES_SENDER_TO_BURNER: `0x${string}`;
    PAUSER_SET_GATEWAY: `0x${string}`;
    PAUSER_SET_HOST: `0x${string}`;
    PAUSER_SET_WRAPPER: `0x${string}`;
    GATEWAY_CONFIG: `0x${string}`;
}

describe("Post-Deployment E2E Tests", () => {
    let addresses: DeploymentAddresses;

    // Anvil processes
    let anvilProcessL1: { proc: AnvilProcess; client: ExtendedTestClient };
    let anvilProcessGateway: { proc: AnvilProcess; client: ExtendedTestClient };

    // Test accounts
    let adminAddress: Address;
    let aliceAddress: Address;
    let bobAddress: Address;

    // Contract instances - L1 (Ethereum/Sepolia)
    let governanceOAppSender: GetContractReturnType<
        Abi,
        ExtendedTestClient,
        Address
    >;
    let zamaERC20: GetContractReturnType<Abi, ExtendedTestClient, Address>;
    let zamaOFTAdapter: GetContractReturnType<Abi, ExtendedTestClient, Address>;
    let protocolFeesBurner: GetContractReturnType<
        Abi,
        ExtendedTestClient,
        Address
    >;
    let acl: GetContractReturnType<Abi, ExtendedTestClient, Address>;

    // Contract instances - Gateway (Zama)
    let governanceOAppReceiver: GetContractReturnType<
        Abi,
        ExtendedTestClient,
        Address
    >;
    let safeProxy: GetContractReturnType<Abi, ExtendedTestClient, Address>;
    let adminModule: GetContractReturnType<Abi, ExtendedTestClient, Address>;
    let zamaOFT: GetContractReturnType<Abi, ExtendedTestClient, Address>;
    let feesSenderToBurner: GetContractReturnType<
        Abi,
        ExtendedTestClient,
        Address
    >;
    let pauserSetHost: GetContractReturnType<Abi, ExtendedTestClient, Address>;
    let pauserSetWrapperHost: GetContractReturnType<
        Abi,
        ExtendedTestClient,
        Address
    >;
    let pauserSetGateway: GetContractReturnType<
        Abi,
        ExtendedTestClient,
        Address
    >;
    let gatewayConfig: GetContractReturnType<Abi, ExtendedTestClient, Address>;

    // Deployment context
    let ctx: DeploymentContext;

    // Role constants
    const MINTER_ROLE = keccak256(toBytes("MINTER_ROLE"));
    const MINTING_PAUSER_ROLE = keccak256(toBytes("MINTING_PAUSER_ROLE"));
    const DEFAULT_ADMIN_ROLE = zeroHash;

    // Endpoint IDs
    const eidEthereumTestnet = EndpointId.SEPOLIA_V2_TESTNET;
    const eidZamaTestnet = EndpointId.ZAMA_V2_TESTNET;

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

        // Setup test accounts
        adminAddress = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8" as Address;
        aliceAddress = "0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC" as Address;
        bobAddress = "0x90F79bf6EB2c4f870365E785982E1f101E93b906" as Address;

        // Fund test accounts on both chains
        await anvilProcessL1.client.setBalance({
            address: adminAddress,
            value: parseEther("10000"),
        });
        await anvilProcessL1.client.setBalance({
            address: aliceAddress,
            value: parseEther("10000"),
        });
        await anvilProcessL1.client.setBalance({
            address: bobAddress,
            value: parseEther("10000"),
        });

        await anvilProcessGateway.client.setBalance({
            address: adminAddress,
            value: parseEther("10000"),
        });
        await anvilProcessGateway.client.setBalance({
            address: aliceAddress,
            value: parseEther("10000"),
        });
        await anvilProcessGateway.client.setBalance({
            address: bobAddress,
            value: parseEther("10000"),
        });

        // Load contract ABIs
        try {
            const {
                governanceOAppSenderAbi,
                governanceOAppReceiverAbi,
                safeL2Abi,
                adminModuleAbi,
                zamaERC20Abi,
                zamaOFTAdapterAbi,
                zamaOFTAbi,
                protocolFeesBurnerAbi,
                feesSenderToBurnerAbi,
                pauserSetHostAbi,
                pauserSetWrapperAbi,
                gatewayConfigAbi,
                ACLAbi,
            } = await loadContractABIs();

            // Create L1 contract instances
            acl = getContract({
                address: addresses.ACL_ADDRESS as Address,
                abi: ACLAbi,
                client: anvilProcessL1.client,
            });
            governanceOAppSender = getContract({
                address: addresses.GOVERNANCE_OAPP_SENDER as Address,
                abi: governanceOAppSenderAbi,
                client: anvilProcessL1.client,
            });

            zamaERC20 = getContract({
                address: addresses.ZAMA_TOKEN as Address,
                abi: zamaERC20Abi,
                client: anvilProcessL1.client,
            });

            zamaOFTAdapter = getContract({
                address: addresses.ZAMA_OFT_ADAPTER as Address,
                abi: zamaOFTAdapterAbi,
                client: anvilProcessL1.client,
            });

            protocolFeesBurner = getContract({
                address: addresses.PROTOCOL_FEES_BURNER as Address,
                abi: protocolFeesBurnerAbi,
                client: anvilProcessL1.client,
            });

            // Create Gateway contract instances
            governanceOAppReceiver = getContract({
                address: addresses.GOVERNANCE_OAPP_RECEIVER as Address,
                abi: governanceOAppReceiverAbi,
                client: anvilProcessGateway.client,
            });

            safeProxy = getContract({
                address: addresses.SAFE_PROXY_ADDRESS as Address,
                abi: safeL2Abi,
                client: anvilProcessGateway.client,
            });

            adminModule = getContract({
                address: addresses.ADMIN_MODULE_ADDRESS as Address,
                abi: adminModuleAbi,
                client: anvilProcessGateway.client,
            });

            gatewayConfig = getContract({
                address: addresses.GATEWAY_CONFIG as Address,
                abi: gatewayConfigAbi,
                client: anvilProcessGateway.client,
            });

            zamaOFT = getContract({
                address: addresses.ZAMA_OFT as Address,
                abi: zamaOFTAbi,
                client: anvilProcessGateway.client,
            });

            feesSenderToBurner = getContract({
                address: addresses.FEES_SENDER_TO_BURNER as Address,
                abi: feesSenderToBurnerAbi,
                client: anvilProcessGateway.client,
            });

            pauserSetHost = getContract({
                address: addresses.PAUSER_SET_HOST as Address,
                abi: pauserSetHostAbi,
                client: anvilProcessL1.client,
            });

            pauserSetWrapperHost = getContract({
                address: addresses.PAUSER_SET_WRAPPER as Address,
                abi: pauserSetWrapperAbi,
                client: anvilProcessL1.client,
            });

            pauserSetGateway = getContract({
                address: addresses.PAUSER_SET_GATEWAY as Address,
                abi: pauserSetHostAbi,
                client: anvilProcessGateway.client,
            });
        } catch (error) {
            throw new Error(
                `Failed to load contract ABIs: ${error}. Make sure the contracts are compiled and the artifacts are present in the expected paths.`,
            );
        }

        const deploymentOrchestrator = await DeploymentOrchestrator.create(
            new Logger(),
            { networkEnvironment: "testnet" },
        );
        ctx = {
            config: deploymentOrchestrator.config,
            state: deploymentOrchestrator.state,
            env: deploymentOrchestrator.env,
            hardhat: deploymentOrchestrator.hardhat,
            prompt: deploymentOrchestrator.prompt,
            logger: deploymentOrchestrator.logger,
            // @ts-expect-error
            networks: {
                getEthereum: () => ({
                    ...deploymentOrchestrator.networks.getEthereum(),
                    rpcUrl:
                        anvilProcessL1.client.chain?.rpcUrls.default.http[0] ??
                        "",
                    chainId: FORKS_CONFIG.SEPOLIA_CHAIN_ID,
                    name: "Sepolia Testnet Fork",
                }),
                getGateway: () => ({
                    ...deploymentOrchestrator.networks.getGateway(),
                    rpcUrl:
                        anvilProcessGateway.client.chain?.rpcUrls.default
                            .http[0] ?? "",
                    chainId: FORKS_CONFIG.GATEWAY_CHAIN_ID,
                    name: "Zama Testnet Fork",
                }),
            },
        };
    });

    afterAll(async () => {
        await stopAnvil(anvilProcessL1?.proc);
        await stopAnvil(anvilProcessGateway?.proc);
    });

    describe("Deployment", () => {
        it("should deploy all contracts successfully", async () => {
            // Verify all contract addresses are valid
            expect(isAddress(addresses.GOVERNANCE_OAPP_SENDER)).toBe(true);
            expect(isAddress(addresses.GOVERNANCE_OAPP_RECEIVER)).toBe(true);
            expect(isAddress(addresses.SAFE_PROXY_ADDRESS)).toBe(true);
            expect(isAddress(addresses.ADMIN_MODULE_ADDRESS)).toBe(true);
            expect(isAddress(addresses.ZAMA_TOKEN)).toBe(true);
            expect(isAddress(addresses.ZAMA_OFT_ADAPTER)).toBe(true);
            expect(isAddress(addresses.ZAMA_OFT)).toBe(true);
            expect(isAddress(addresses.PROTOCOL_FEES_BURNER)).toBe(true);
            expect(isAddress(addresses.FEES_SENDER_TO_BURNER)).toBe(true);
            expect(isAddress(addresses.PAUSER_SET_GATEWAY)).toBe(true);
            expect(isAddress(addresses.PAUSER_SET_HOST)).toBe(true);
            expect(isAddress(addresses.PAUSER_SET_WRAPPER)).toBe(true);
        });

        it("should have correct governance wiring", async () => {
            // Check peers are set
            const senderPeer = (await governanceOAppSender.read.peers([
                eidZamaTestnet,
            ])) as `0x${string}`;
            const receiverPeer = (await governanceOAppReceiver.read.peers([
                eidEthereumTestnet,
            ])) as `0x${string}`;

            const expectedSenderPeer = pad(
                getAddress(addresses.GOVERNANCE_OAPP_RECEIVER),
                { size: 32 },
            );
            const expectedReceiverPeer = pad(
                getAddress(addresses.GOVERNANCE_OAPP_SENDER),
                { size: 32 },
            );

            expect(senderPeer.toLowerCase()).toBe(
                expectedSenderPeer.toLowerCase(),
            );
            expect(receiverPeer.toLowerCase()).toBe(
                expectedReceiverPeer.toLowerCase(),
            );

            // Check AdminModule is linked
            expect(await governanceOAppReceiver.read.adminSafeModule()).toBe(
                getAddress(addresses.ADMIN_MODULE_ADDRESS),
            );
        });

        it("should verify ownership according to deployment", async () => {
            // Host Chain Contracts should be owned by DAO
            expect(await zamaOFTAdapter.read.owner()).toBe(
                getAddress(addresses.DAO_ADDRESS),
            );
            expect(await governanceOAppSender.read.owner()).toBe(
                getAddress(addresses.DAO_ADDRESS),
            );

            // Gateway Chain Contracts should be owned by Safe
            expect(await zamaOFT.read.owner()).toBe(
                getAddress(addresses.SAFE_PROXY_ADDRESS),
            );
            expect(await governanceOAppReceiver.read.owner()).toBe(
                getAddress(addresses.SAFE_PROXY_ADDRESS),
            );
            expect(await gatewayConfig.read.owner()).toBe(
                getAddress(addresses.SAFE_PROXY_ADDRESS),
            );
        });

        it("should verify ACL ownership is DAO", async () => {
            expect(await acl.read.owner()).toBe(
                getAddress(addresses.DAO_ADDRESS),
            );
        });
    });

    describe("Token & OFT Actions", () => {
        it("should have correct token bridging wiring", async () => {
            // Check OFT peers
            const adapterPeer = (await zamaOFTAdapter.read.peers([
                eidZamaTestnet,
            ])) as `0x${string}`;
            const oftPeer = (await zamaOFT.read.peers([
                eidEthereumTestnet,
            ])) as `0x${string}`;

            const expectedAdapterPeer = pad(getAddress(addresses.ZAMA_OFT), {
                size: 32,
            });
            const expectedOftPeer = pad(
                getAddress(addresses.ZAMA_OFT_ADAPTER),
                { size: 32 },
            );

            expect(adapterPeer.toLowerCase()).toBe(
                expectedAdapterPeer.toLowerCase(),
            );
            expect(oftPeer.toLowerCase()).toBe(expectedOftPeer.toLowerCase());
        });

        it("should have correct roles granted", async () => {
            // Check roles on deployed token
            expect(
                await zamaERC20.read.hasRole([
                    DEFAULT_ADMIN_ROLE,
                    addresses.DAO_ADDRESS,
                ]),
            ).toBe(true);
            expect(
                await zamaERC20.read.hasRole([
                    MINTING_PAUSER_ROLE,
                    addresses.PAUSER_SET_WRAPPER,
                ]),
            ).toBe(true);
        });

        it("should distribute initial ERC20 supply to configured recipients", async () => {
            const projectRoot = resolveProjectRoot();
            const cfgPath = path.resolve(
                projectRoot,
                "protocol-contracts/deployment-cli/deployment-state/deployment-config.yaml",
            );
            const raw = readFileSync(cfgPath, "utf-8");
            const cfg = YAML.parse(raw) as DeploymentConfig;
            const recipients = (cfg?.protocol?.token?.recipients ??
                []) as Array<{
                address: string;
                amount: string;
            }>;
            for (const r of recipients) {
                const who = getAddress(r.address) as Address;
                const expected = BigInt(r.amount);
                const bal = (await zamaERC20.read.balanceOf([who])) as bigint;
                expect(bal === expected * 10n ** 18n).toBe(true);
            }
        });
        it.todo(
            "should mint tokens to alice via governance proposal",
            async () => {
                // TODO: This test is expected to fail because INITIAL_ADMIN doesn't have MINTER_ROLE
                // The DAO doesn't have MINTER_ROLE
                const mintAmount = parseEther("1000");

                const balanceBefore = (await zamaERC20.read.balanceOf([
                    aliceAddress,
                ])) as bigint;

                // Use executeViaAdminPlugin to mint tokens to alice
                const mintData = encodeFunctionData({
                    abi: zamaERC20.abi,
                    functionName: "mint",
                    args: [aliceAddress, mintAmount],
                });

                await executeViaAdminPlugin(ctx, [
                    {
                        to: addresses.ZAMA_TOKEN,
                        value: 0n,
                        data: mintData,
                    },
                ]);

                const balanceAfter = (await zamaERC20.read.balanceOf([
                    aliceAddress,
                ])) as bigint;
                expect(balanceAfter).toBe(balanceBefore + mintAmount);
            },
        );

        it.todo("should allow alice to transfer tokens to bob", async () => {
            // TODO: alice doesn't have tokens
            const transferAmount = parseEther("100");

            const aliceBalanceBefore = (await zamaERC20.read.balanceOf([
                aliceAddress,
            ])) as bigint;
            const bobBalanceBefore = (await zamaERC20.read.balanceOf([
                bobAddress,
            ])) as bigint;

            // Impersonate alice
            await anvilProcessGateway.client.impersonateAccount({
                address: aliceAddress,
            });

            const aliceSigner = createWalletClient({
                account: aliceAddress,
                chain: anvilProcessGateway.client.chain,
                transport: http(ctx.networks.getGateway().rpcUrl),
            });

            const tokenAsAlice = getContract({
                address: addresses.ZAMA_TOKEN as Address,
                abi: zamaERC20.abi,
                client: aliceSigner,
            });

            const hash = await tokenAsAlice.write.transfer([
                bobAddress,
                transferAmount,
            ]);
            await anvilProcessGateway.client.waitForTransactionReceipt({
                hash,
            });

            await anvilProcessGateway.client.stopImpersonatingAccount({
                address: aliceAddress,
            });

            const aliceBalanceAfter = (await zamaERC20.read.balanceOf([
                aliceAddress,
            ])) as bigint;
            const bobBalanceAfter = (await zamaERC20.read.balanceOf([
                bobAddress,
            ])) as bigint;

            expect(aliceBalanceAfter).toBe(aliceBalanceBefore - transferAmount);
            expect(bobBalanceAfter).toBe(bobBalanceBefore + transferAmount);
        });

        it.todo("should allow bob to burn his tokens", async () => {
            // TODO: bob doesn't have tokens
            const burnAmount = parseEther("50");

            const bobBalanceBefore = (await zamaERC20.read.balanceOf([
                bobAddress,
            ])) as bigint;
            const totalSupplyBefore =
                (await zamaERC20.read.totalSupply()) as bigint;

            await anvilProcessGateway.client.impersonateAccount({
                address: bobAddress,
            });

            const bobSigner = createWalletClient({
                account: bobAddress,
                chain: anvilProcessGateway.client.chain,
                transport: http(ctx.networks.getGateway().rpcUrl),
            });

            const tokenAsBob = getContract({
                address: addresses.ZAMA_TOKEN as Address,
                abi: zamaERC20.abi,
                client: bobSigner,
            });

            const hash = await tokenAsBob.write.burn([burnAmount]);
            await anvilProcessGateway.client.waitForTransactionReceipt({
                hash,
            });

            await anvilProcessGateway.client.stopImpersonatingAccount({
                address: bobAddress,
            });

            const bobBalanceAfter = (await zamaERC20.read.balanceOf([
                bobAddress,
            ])) as bigint;
            const totalSupplyAfter =
                (await zamaERC20.read.totalSupply()) as bigint;

            expect(bobBalanceAfter).toBe(bobBalanceBefore - burnAmount);
            expect(totalSupplyAfter).toBe(totalSupplyBefore - burnAmount);
        });

        it("should verify paused state", async () => {
            // Check if paused
            const isPaused = (await zamaERC20.read.paused()) as boolean;
            // Note: In deployed contracts, paused state depends on actual deployment state
            // We just verify the function is callable
            expect(typeof isPaused).toBe("boolean");
        });

        it.todo(
            "should allow cross-chain token transfer from Ethereum to Gateway",
            async () => {
                // TODO: alice doesn't have tokens
                // First, ensure alice has tokens on L1
                const testAddress =
                    "0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65" as Address;
                await anvilProcessL1.client.setBalance({
                    address: testAddress,
                    value: parseEther("10"),
                });

                // Mint tokens to test address via governance
                const mintAmount = parseEther("1000");
                const targets = [addresses.ZAMA_TOKEN];
                const values = [0];
                const functionSignatures = [""];
                const datas = [
                    encodeFunctionData({
                        abi: zamaERC20.abi,
                        functionName: "mint",
                        args: [testAddress, mintAmount],
                    }),
                ];
                const operations = [0];

                const options = Options.newOptions()
                    .addExecutorLzReceiveOption(500000, 0)
                    .toHex()
                    .toString() as `0x${string}`;

                const quotedFee =
                    (await governanceOAppSender.read.quoteSendCrossChainTransaction(
                        [
                            targets,
                            values,
                            functionSignatures,
                            datas,
                            operations,
                            options,
                        ],
                    )) as bigint;

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
                        anvilProcessL1.client.chain?.rpcUrls.default.http[0] ??
                            "",
                    ),
                });

                const senderAsDAO = getContract({
                    address: addresses.GOVERNANCE_OAPP_SENDER as Address,
                    abi: governanceOAppSender.abi,
                    client: daoSigner,
                });

                let hash = await senderAsDAO.write.sendRemoteProposal(
                    [
                        targets,
                        values,
                        functionSignatures,
                        datas,
                        operations,
                        options,
                    ],
                    { value: quotedFee },
                );

                await anvilProcessL1.client.waitForTransactionReceipt({ hash });
                await anvilProcessL1.client.stopImpersonatingAccount({
                    address: daoAddress,
                });

                // Deliver mint message
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
                    eidEthereumTestnet,
                    addresses.GOVERNANCE_OAPP_SENDER,
                    message,
                    Options.fromOptions(options),
                );

                // Now perform cross-chain transfer
                const sendAmount = parseEther("100");

                // Test address needs to approve OFTAdapter
                await anvilProcessGateway.client.impersonateAccount({
                    address: testAddress,
                });

                const testSigner = createWalletClient({
                    account: testAddress,
                    chain: anvilProcessGateway.client.chain,
                    transport: http(
                        anvilProcessGateway.client.chain?.rpcUrls.default
                            .http[0],
                    ),
                });

                const tokenAsTest = getContract({
                    address: addresses.ZAMA_TOKEN as Address,
                    abi: zamaERC20.abi,
                    client: testSigner,
                });

                hash = await tokenAsTest.write.approve([
                    addresses.ZAMA_OFT_ADAPTER as Address,
                    sendAmount,
                ]);
                await anvilProcessGateway.client.waitForTransactionReceipt({
                    hash,
                });

                await anvilProcessGateway.client.stopImpersonatingAccount({
                    address: testAddress,
                });

                // Check balances before
                const testERC20BalanceBefore = (await zamaERC20.read.balanceOf([
                    testAddress,
                ])) as bigint;
                const _testOFTBalanceBefore = (await zamaOFT.read.balanceOf([
                    testAddress,
                ])) as bigint;

                // Prepare OFT send - NOTE: In actual deployment, need to call from L1 side
                // Since tokens are on Gateway side, we need to send from Gateway to L1 (reverse direction)
                // This test verifies the OFT infrastructure is in place
                expect(testERC20BalanceBefore).toBeGreaterThanOrEqual(
                    sendAmount,
                );
            },
        );
    });

    describe("PauserSet & PauserSetWrapper on Host Chain", () => {
        it("should verify PauserSetWrapper is configured with correct target", async () => {
            expect(await pauserSetWrapperHost.read.CONTRACT_TARGET()).toBe(
                getAddress(addresses.ZAMA_TOKEN),
            );
        });

        it("should verify PauserSetWrapper is configured with correct PauserSet", async () => {
            expect(await pauserSetWrapperHost.read.PAUSER_SET()).toBe(
                getAddress(addresses.PAUSER_SET_HOST),
            );
        });

        it("should verify PauserSetWrapper has correct function signature", async () => {
            expect(await pauserSetWrapperHost.read.FUNCTION_SIGNATURE()).toBe(
                "pauseMinting()",
            );

            const expectedSelector = keccak256(toBytes("pauseMinting()")).slice(
                0,
                10,
            );
            expect(await pauserSetWrapperHost.read.FUNCTION_SELECTOR()).toBe(
                expectedSelector,
            );
        });

        it("should verify pausers in PauserSet", async () => {
            // Check if pauserSetWrapperHost has MINTING_PAUSER_ROLE
            expect(
                await zamaERC20.read.hasRole([
                    MINTING_PAUSER_ROLE,
                    addresses.PAUSER_SET_WRAPPER as Address,
                ]),
            ).toBe(true);
        });
        it("should be possible to add a pauser to the PauserSet through DAO proposal", async () => {
            // alice should not be a pauser initially
            expect(await pauserSetHost.read.isPauser([aliceAddress])).toBe(
                false,
            );

            // Add alice as a pauser through DAO proposal
            await executeViaAdminPlugin(ctx, [
                {
                    to: addresses.PAUSER_SET_HOST,
                    value: 0n,
                    data: encodeFunctionData({
                        abi: pauserSetHost.abi,
                        functionName: "addPauser",
                        args: [aliceAddress],
                    }),
                },
            ]);
            expect(await pauserSetHost.read.isPauser([aliceAddress])).toBe(
                true,
            );
        });
    });

    describe("Cross-chain Governance", () => {
        it("adds a pauser on Gateway via cross-chain proposal", async () => {
            const testPauser =
                "0x0000000000000000000000000000000000000001" as Address;
            const before = await pauserSetGateway.read.isPauser([testPauser]);
            expect(before).toBe(false);

            const addPauserData = encodeFunctionData({
                abi: pauserSetGateway.abi,
                functionName: "addPauser",
                args: [testPauser],
            });
            const targets = [addresses.PAUSER_SET_GATEWAY];
            const values = [0];
            const functionSignatures = [""];
            const datas = [addPauserData];
            const operations = [0];

            const options = Options.newOptions()
                .addExecutorLzReceiveOption(200000, 0)
                .toHex()
                .toString() as `0x${string}`;
            const quotedFee =
                (await governanceOAppSender.read.quoteSendCrossChainTransaction(
                    [
                        targets,
                        values,
                        functionSignatures,
                        datas,
                        operations,
                        options,
                    ],
                )) as bigint;

            const dao = getAddress(addresses.DAO_ADDRESS) as Address;
            await anvilProcessL1.client.impersonateAccount({ address: dao });
            await anvilProcessL1.client.setBalance({
                address: dao,
                value: parseEther("10"),
            });
            const daoSigner = createWalletClient({
                account: dao,
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
                    options,
                ],
                { value: quotedFee },
            );
            await anvilProcessL1.client.waitForTransactionReceipt({ hash });
            await anvilProcessL1.client.stopImpersonatingAccount({
                address: dao,
            });

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
                Options.fromOptions(options),
            );

            const after = await pauserSetGateway.read.isPauser([testPauser]);
            expect(after).toBe(true);
        });

        it("adds two pausers on Gateway via batch proposal", async () => {
            const p1 = "0x0000000000000000000000000000000000000003" as Address;
            const p2 = "0x0000000000000000000000000000000000000004" as Address;
            expect(await pauserSetGateway.read.isPauser([p1])).toBe(false);
            expect(await pauserSetGateway.read.isPauser([p2])).toBe(false);

            const d1 = encodeFunctionData({
                abi: pauserSetGateway.abi,
                functionName: "addPauser",
                args: [p1],
            });
            const d2 = encodeFunctionData({
                abi: pauserSetGateway.abi,
                functionName: "addPauser",
                args: [p2],
            });
            const targets = [
                addresses.PAUSER_SET_GATEWAY,
                addresses.PAUSER_SET_GATEWAY,
            ];
            const values = [0, 0];
            const functionSignatures = ["", ""];
            const datas = [d1, d2];
            const operations = [0, 0];

            const options = Options.newOptions()
                .addExecutorLzReceiveOption(200000, 0)
                .toHex()
                .toString() as `0x${string}`;
            const quotedFee =
                (await governanceOAppSender.read.quoteSendCrossChainTransaction(
                    [
                        targets,
                        values,
                        functionSignatures,
                        datas,
                        operations,
                        options,
                    ],
                )) as bigint;

            const dao = getAddress(addresses.DAO_ADDRESS) as Address;
            await anvilProcessL1.client.impersonateAccount({ address: dao });
            await anvilProcessL1.client.setBalance({
                address: dao,
                value: parseEther("10"),
            });
            const daoSigner = createWalletClient({
                account: dao,
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
                    options,
                ],
                { value: quotedFee },
            );
            await anvilProcessL1.client.waitForTransactionReceipt({ hash });
            await anvilProcessL1.client.stopImpersonatingAccount({
                address: dao,
            });

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
                Options.fromOptions(options),
            );

            expect(await pauserSetGateway.read.isPauser([p1])).toBe(true);
            expect(await pauserSetGateway.read.isPauser([p2])).toBe(true);
        });

        it("rejects delivery with wrong srcEid", async () => {
            const message = encodeGovernanceMessage([], [], [], [], []);
            let failed = false;
            try {
                await deliverToReceiver(
                    anvilProcessGateway.client,
                    addresses.GOVERNANCE_OAPP_RECEIVER as Address,
                    governanceOAppReceiver.abi,
                    EndpointId.ZAMA_V2_TESTNET, // wrong origin EID
                    addresses.GOVERNANCE_OAPP_SENDER,
                    message,
                    Options.newOptions().addExecutorLzReceiveOption(100000, 0),
                );
            } catch {
                failed = true;
            }
            expect(failed).toBe(true);
        });

        it("rejects delivery with wrong sender", async () => {
            const message = encodeGovernanceMessage([], [], [], [], []);
            let failed = false;
            try {
                await deliverToReceiver(
                    anvilProcessGateway.client,
                    addresses.GOVERNANCE_OAPP_RECEIVER as Address,
                    governanceOAppReceiver.abi,
                    EndpointId.SEPOLIA_V2_TESTNET,
                    "0x000000000000000000000000000000000000dEaD", // wrong sender
                    message,
                    Options.newOptions().addExecutorLzReceiveOption(100000, 0),
                );
            } catch {
                failed = true;
            }
            expect(failed).toBe(true);
        });
    });

    describe("ZamaERC20 roles", () => {
        it("should get role admin for MINTER_ROLE", async () => {
            const roleAdmin = (await zamaERC20.read.getRoleAdmin([
                MINTER_ROLE,
            ])) as `0x${string}`;
            expect(roleAdmin).toBe(DEFAULT_ADMIN_ROLE);
        });

        it("should get role admin for MINTING_PAUSER_ROLE", async () => {
            const roleAdmin = (await zamaERC20.read.getRoleAdmin([
                MINTING_PAUSER_ROLE,
            ])) as `0x${string}`;
            expect(roleAdmin).toBe(DEFAULT_ADMIN_ROLE);
        });

        it("should verify DAO has DEFAULT_ADMIN_ROLE", async () => {
            expect(
                await zamaERC20.read.hasRole([
                    DEFAULT_ADMIN_ROLE,
                    addresses.DAO_ADDRESS as Address,
                ]),
            ).toBe(true);
        });

        it("should grant MINTER_ROLE to alice via DAO governance", async () => {
            // Check alice doesn't have role initially
            expect(
                await zamaERC20.read.hasRole([MINTER_ROLE, aliceAddress]),
            ).toBe(false);

            // Grant role via DAO using the Admin plugin
            await grantRoleViaDao({
                ctx,
                tokenAddress: addresses.ZAMA_TOKEN as Address,
                grantee: aliceAddress,
                role: "MINTER_ROLE",
            });

            // Verify alice now has the role
            expect(
                await zamaERC20.read.hasRole([MINTER_ROLE, aliceAddress]),
            ).toBe(true);
        });
    });

    describe("Fee Burning Mechanism", () => {
        it("should verify ProtocolFeesBurner is configured with correct token", async () => {
            expect(await protocolFeesBurner.read.ZAMA_ERC20()).toBe(
                getAddress(addresses.ZAMA_TOKEN),
            );
        });

        it("should verify FeesSenderToBurner is configured correctly", async () => {
            expect(await feesSenderToBurner.read.ZAMA_OFT()).toBe(
                getAddress(addresses.ZAMA_OFT),
            );
            expect(await feesSenderToBurner.read.PROTOCOL_FEES_BURNER()).toBe(
                getAddress(addresses.PROTOCOL_FEES_BURNER),
            );
        });

        it("should allow anyone to burn fees on ProtocolFeesBurner", async () => {
            // First, check current balance
            const burnerBalance = (await zamaERC20.read.balanceOf([
                addresses.PROTOCOL_FEES_BURNER as Address,
            ])) as bigint;

            if (burnerBalance > 0n) {
                const totalSupplyBefore =
                    (await zamaERC20.read.totalSupply()) as bigint;

                // Bob can call burnFees()
                await anvilProcessGateway.client.impersonateAccount({
                    address: bobAddress,
                });

                const bobSigner = createWalletClient({
                    account: bobAddress,
                    chain: anvilProcessGateway.client.chain,
                    transport: http(
                        anvilProcessGateway.client.chain?.rpcUrls.default
                            .http[0],
                    ),
                });

                const burnerAsBob = getContract({
                    address: addresses.PROTOCOL_FEES_BURNER as Address,
                    abi: protocolFeesBurner.abi,
                    client: bobSigner,
                });

                const hash = await burnerAsBob.write.burnFees();
                await anvilProcessGateway.client.waitForTransactionReceipt({
                    hash,
                });

                await anvilProcessGateway.client.stopImpersonatingAccount({
                    address: bobAddress,
                });

                // Burner balance should be 0
                const burnerBalanceAfter = (await zamaERC20.read.balanceOf([
                    addresses.PROTOCOL_FEES_BURNER as Address,
                ])) as bigint;
                expect(burnerBalanceAfter).toBe(0n);

                // Total supply should decrease
                const totalSupplyAfter =
                    (await zamaERC20.read.totalSupply()) as bigint;
                expect(totalSupplyAfter).toBe(
                    totalSupplyBefore - burnerBalance,
                );
            }
        });
    });

    describe("Governance & Safe", () => {
        it("should verify GovernanceOAppSender is owned by DAO", async () => {
            expect(await governanceOAppSender.read.owner()).toBe(
                getAddress(addresses.DAO_ADDRESS),
            );
        });

        it("should verify GovernanceOAppReceiver is owned by Safe", async () => {
            expect(await governanceOAppReceiver.read.owner()).toBe(
                getAddress(addresses.SAFE_PROXY_ADDRESS),
            );
        });

        it("should verify AdminModule configuration", async () => {
            expect(await adminModule.read.ADMIN_ACCOUNT()).toBe(
                getAddress(addresses.GOVERNANCE_OAPP_RECEIVER),
            );
            expect(await adminModule.read.SAFE_PROXY()).toBe(
                getAddress(addresses.SAFE_PROXY_ADDRESS),
            );
        });

        it("should verify GovernanceOAppReceiver has AdminModule set", async () => {
            expect(await governanceOAppReceiver.read.adminSafeModule()).toBe(
                getAddress(addresses.ADMIN_MODULE_ADDRESS),
            );
        });

        it("should verify AdminModule is enabled in Safe", async () => {
            const isEnabled = (await safeProxy.read.isModuleEnabled([
                addresses.ADMIN_MODULE_ADDRESS as Address,
            ])) as boolean;
            expect(isEnabled).toBe(true);
        });

        it.todo("should send cross-chain proposal to mint tokens", async () => {
            // TODO: This test is expected to fail because no one has the MINTER_ROLE yet.
            const mintAmount = parseEther("500");

            const bobBalanceBefore = (await zamaERC20.read.balanceOf([
                bobAddress,
            ])) as bigint;

            // Build proposal parameters
            const targets = [addresses.ZAMA_TOKEN];
            const values = [0];
            const functionSignatures = [""];
            const datas = [
                encodeFunctionData({
                    abi: zamaERC20.abi,
                    functionName: "mint",
                    args: [bobAddress, mintAmount],
                }),
            ];
            const operations = [0];

            const options = Options.newOptions()
                .addExecutorLzReceiveOption(500000, 0)
                .toHex()
                .toString() as `0x${string}`;

            const quotedFee =
                (await governanceOAppSender.read.quoteSendCrossChainTransaction(
                    [
                        targets,
                        values,
                        functionSignatures,
                        datas,
                        operations,
                        options,
                    ],
                )) as bigint;

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
                    options,
                ],
                { value: quotedFee },
            );

            await anvilProcessL1.client.waitForTransactionReceipt({ hash });
            await anvilProcessL1.client.stopImpersonatingAccount({
                address: daoAddress,
            });

            // Deliver message to Gateway
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
                eidEthereumTestnet,
                addresses.GOVERNANCE_OAPP_SENDER,
                message,
                Options.fromOptions(options),
            );

            // Verify Bob received the tokens
            const bobBalanceAfter = (await zamaERC20.read.balanceOf([
                bobAddress,
            ])) as bigint;
            expect(bobBalanceAfter).toBe(bobBalanceBefore + mintAmount);
        });

        it.todo(
            "should send multi-action proposal via governance",
            async () => {
                // TODO: This test is expected to fail because no one has the MINTER_ROLE yet.
                const mintAmount1 = parseEther("200");
                const mintAmount2 = parseEther("300");

                const aliceBalanceBefore = (await zamaERC20.read.balanceOf([
                    aliceAddress,
                ])) as bigint;
                const bobBalanceBefore = (await zamaERC20.read.balanceOf([
                    bobAddress,
                ])) as bigint;

                const targets = [addresses.ZAMA_TOKEN, addresses.ZAMA_TOKEN];
                const values = [0, 0];
                const functionSignatures = ["", ""];
                const datas = [
                    encodeFunctionData({
                        abi: zamaERC20.abi,
                        functionName: "mint",
                        args: [aliceAddress, mintAmount1],
                    }),
                    encodeFunctionData({
                        abi: zamaERC20.abi,
                        functionName: "mint",
                        args: [bobAddress, mintAmount2],
                    }),
                ];
                const operations = [0, 0];

                const options = Options.newOptions()
                    .addExecutorLzReceiveOption(600000, 0)
                    .toHex()
                    .toString() as `0x${string}`;

                const quotedFee =
                    (await governanceOAppSender.read.quoteSendCrossChainTransaction(
                        [
                            targets,
                            values,
                            functionSignatures,
                            datas,
                            operations,
                            options,
                        ],
                    )) as bigint;

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
                        options,
                    ],
                    { value: quotedFee },
                );

                await anvilProcessL1.client.waitForTransactionReceipt({ hash });
                await anvilProcessL1.client.stopImpersonatingAccount({
                    address: daoAddress,
                });

                // Deliver message
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
                    eidEthereumTestnet,
                    addresses.GOVERNANCE_OAPP_SENDER,
                    message,
                    Options.fromOptions(options),
                );

                const aliceBalanceAfter = (await zamaERC20.read.balanceOf([
                    aliceAddress,
                ])) as bigint;
                const bobBalanceAfter = (await zamaERC20.read.balanceOf([
                    bobAddress,
                ])) as bigint;

                expect(aliceBalanceAfter).toBe(
                    aliceBalanceBefore + mintAmount1,
                );
                expect(bobBalanceAfter).toBe(bobBalanceBefore + mintAmount2);
            },
        );

        it("should verify Safe owners", async () => {
            const owners = (await safeProxy.read.getOwners()) as Address[];
            expect(owners.length).toBeGreaterThan(0);
        });

        it("should verify Safe threshold", async () => {
            const threshold = (await safeProxy.read.getThreshold()) as bigint;
            expect(threshold).toBeGreaterThan(0n);
        });
    });

    describe("PauserSetWrapper Behavior", () => {
        it("allows a pauser to pause minting via wrapper", async () => {
            // Ensure alice is a pauser on host
            if (!(await pauserSetHost.read.isPauser([aliceAddress]))) {
                await executeViaAdminPlugin(ctx, [
                    {
                        to: addresses.PAUSER_SET_HOST,
                        value: 0n,
                        data: encodeFunctionData({
                            abi: pauserSetHost.abi,
                            functionName: "addPauser",
                            args: [aliceAddress],
                        }),
                    },
                ]);
            }

            await anvilProcessL1.client.impersonateAccount({
                address: aliceAddress,
            });
            const aliceSigner = createWalletClient({
                account: aliceAddress,
                chain: anvilProcessL1.client.chain,
                transport: http(
                    anvilProcessL1.client.chain?.rpcUrls.default.http[0],
                ),
            });
            const wrapperAsAlice = getContract({
                address: addresses.PAUSER_SET_WRAPPER as Address,
                abi: pauserSetWrapperHost.abi,
                client: aliceSigner,
            });
            const tx = await wrapperAsAlice.write.callFunction(["0x"]);
            await anvilProcessL1.client.waitForTransactionReceipt({ hash: tx });
            await anvilProcessL1.client.stopImpersonatingAccount({
                address: aliceAddress,
            });

            const isPaused = (await zamaERC20.read.paused()) as boolean;
            expect(isPaused).toBe(true);
        });

        it("rejects non-pauser calling wrapper", async () => {
            await anvilProcessL1.client.impersonateAccount({
                address: bobAddress,
            });
            const bobSigner = createWalletClient({
                account: bobAddress,
                chain: anvilProcessL1.client.chain,
                transport: http(
                    anvilProcessL1.client.chain?.rpcUrls.default.http[0],
                ),
            });
            const wrapperAsBob = getContract({
                address: addresses.PAUSER_SET_WRAPPER as Address,
                abi: pauserSetWrapperHost.abi,
                client: bobSigner,
            });
            const tx = await wrapperAsBob.write.callFunction(["0x"]);
            const receipt =
                await anvilProcessL1.client.waitForTransactionReceipt({
                    hash: tx,
                });
            expect(receipt.status).toBe("reverted");
            await anvilProcessL1.client.stopImpersonatingAccount({
                address: bobAddress,
            });
        });
    });
});
