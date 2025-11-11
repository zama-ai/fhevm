import path from "node:path";
import { execa } from "execa";
import { ValidationError } from "../utils/errors.js";
import { resolveProjectRoot } from "../utils/project-paths.js";
import { withRetry } from "../utils/retry.js";
import { TaskOutputReader } from "../utils/task-output-reader.js";
import {
    BaseStep,
    type DeploymentContext,
    type StepExecutionResult,
} from "./base-step.js";

export class Step04TokenDeployment extends BaseStep {
    public readonly id = "step-04";
    public readonly name = "Deploy Zama Token";
    public readonly description =
        "Deploys ZamaERC20, ZamaOFTAdapter, and ZamaOFT contracts and wires LayerZero configuration.";
    public readonly dependencies = ["step-01", "step-02"] as const;
    public readonly pkgName = "protocol-contracts/token" as const;

    protected async validate(ctx: DeploymentContext): Promise<void> {
        const recipients = ctx.config.protocol.token.recipients;
        if (!recipients || recipients.length === 0) {
            throw new ValidationError(
                "At least one token recipient is required before Step 4.",
            );
        }
        const receiver = ctx.env.getAddress("DAO_ADDRESS");
        if (!receiver) {
            throw new ValidationError("DAO address is required before Step 4.");
        }
        const safeProxyAddress = ctx.env.getAddress("SAFE_PROXY_ADDRESS");
        if (!safeProxyAddress) {
            throw new ValidationError(
                "Safe address is required before Step 4.",
            );
        }
    }

    protected async execute(
        ctx: DeploymentContext,
    ): Promise<StepExecutionResult> {
        const ethereum = ctx.networks.getEthereum();
        const gateway = ctx.networks.getGateway();
        const protocolPk = ctx.env.resolveWalletPrivateKey("protocol_deployer");
        const daoAddress = ctx.env.getAddress("DAO_ADDRESS");
        const safeProxyAddress = ctx.env.getAddress("SAFE_PROXY_ADDRESS");

        if (!daoAddress || !safeProxyAddress) {
            throw new ValidationError(
                "Required prerequisite addresses missing before token deployment.",
            );
        }

        const recipients = ctx.config.protocol.token.recipients;
        const envVars: Record<string, string> = {
            PRIVATE_KEY: protocolPk,
            INITIAL_ADMIN: daoAddress,
            SEPOLIA_RPC_URL: ethereum.rpcUrl,
            RPC_URL_ZAMA_GATEWAY_TESTNET: gateway.rpcUrl,
            DAO_ADDRESS: daoAddress,
            SAFE_PROXY_ADDRESS: safeProxyAddress,
            NUM_INITIAL_RECEIVERS: recipients.length.toString(),
        };

        // Add indexed recipient environment variables
        recipients.forEach((recipient, idx) => {
            envVars[`INITIAL_RECEIVER_${idx}`] = recipient.address;
            envVars[`INITIAL_AMOUNT_${idx}`] = recipient.amount;
        });

        const baseEnv = ctx.env.buildTaskEnv(envVars);

        // Determine deployment preset based on network
        // Note: preset is different from hostPkgName; it's a deployment preset for the token package
        const presetName = ctx.networks.getSelectedEnvironment();

        // Run the orchestrated token deployment task
        const args = ["--preset", presetName];
        await ctx.hardhat.runTask({
            pkg: this.pkgName,
            task: "deploy:token",
            args: args,
            env: baseEnv,
        });

        // Extract addresses from deployment artifacts
        const projectRoot = resolveProjectRoot();
        const reader = new TaskOutputReader(projectRoot);

        const zamaToken = reader.readHardhatDeployment(
            this.pkgName,
            ethereum.name,
            "ZamaERC20",
        );
        const zamaOftAdapter = reader.readHardhatDeployment(
            this.pkgName,
            ethereum.name,
            "ZamaOFTAdapter",
        );
        const zamaOft = reader.readHardhatDeployment(
            this.pkgName,
            gateway.name,
            "ZamaOFT",
        );

        // Handle ownership and delegation
        const tokenOwnershipTasks = [
            {
                task: "zama:oftadapter:setDelegate",
                args: [
                    "--address",
                    daoAddress,
                    "--from-deployment",
                    "true",
                    "--network",
                    ethereum.name,
                ],
            },
            {
                task: "zama:oftadapter:transferOwnership",
                args: [
                    "--address",
                    daoAddress,
                    "--from-deployment",
                    "true",
                    "--network",
                    ethereum.name,
                ],
            },
            {
                task: "zama:oft:setDelegate",
                args: [
                    "--address",
                    safeProxyAddress,
                    "--from-deployment",
                    "true",
                    "--network",
                    gateway.name,
                ],
            },
            {
                task: "zama:oft:transferOwnership",
                args: [
                    "--address",
                    safeProxyAddress,
                    "--from-deployment",
                    "true",
                    "--network",
                    gateway.name,
                ],
            },
        ] as const;

        for (const ownershipTask of tokenOwnershipTasks) {
            await ctx.hardhat.runTask({
                pkg: this.pkgName,
                task: ownershipTask.task,
                args: ownershipTask.args as unknown as string[],
                env: baseEnv,
            });
        }

        return {
            addresses: {
                ZAMA_TOKEN: zamaToken,
                ZAMA_OFT: zamaOft,
                ZAMA_OFT_ADAPTER: zamaOftAdapter,
            },
        };
    }

    protected async verifyDeployments(ctx: DeploymentContext): Promise<void> {
        const ethereum = ctx.networks.getEthereum();
        const gateway = ctx.networks.getGateway();
        const protocolPk = ctx.env.resolveWalletPrivateKey("protocol_deployer");
        const daoAddress = ctx.env.getAddress("DAO_ADDRESS");

        const baseEnv = ctx.env.buildTaskEnv({
            PRIVATE_KEY: protocolPk,
            INITIAL_ADMIN: daoAddress,
            SEPOLIA_RPC_URL: ethereum.rpcUrl,
            RPC_URL_ZAMA_GATEWAY_TESTNET: gateway.rpcUrl,
            DAO_ADDRESS: daoAddress,
            ETHERSCAN_API: ethereum.explorerApiKey,
            BLOCKSCOUT_API: gateway.blockscoutApiUrl,
        });

        const networkEnvironment = ctx.networks.getSelectedEnvironment();
        const pkgDir = path.join(resolveProjectRoot(), this.pkgName);
        const pkgManager = this.getPackageManager();

        const ethereumVerifyScript = this.getVerificationScriptName(
            networkEnvironment,
            "ethereum",
        );
        const gatewayVerifyScript = this.getVerificationScriptName(
            networkEnvironment,
            "gateway",
        );

        ctx.logger.info("Verifying token contracts on block explorers...");
        ctx.logger.info(
            `Verifying ZamaERC20 and ZamaOFTAdapter on ${ethereum.name}...`,
        );
        await withRetry(
            async () => {
                await execa(pkgManager, ["run", ethereumVerifyScript], {
                    cwd: pkgDir,
                    env: baseEnv,
                    stdio: ["inherit", "inherit", "inherit"],
                });
            },
            {
                maxAttempts: 3,
                initialDelayMs: 10000,
                onRetry: (attempt) => {
                    ctx.logger.warn(
                        `Token contracts verification on ${ethereum.name} failed, retrying (attempt ${attempt}/3)...`,
                    );
                },
            },
        );
        ctx.logger.success(
            `Verified ZamaERC20 and ZamaOFTAdapter on ${ethereum.name}`,
        );

        // Verify ZamaOFT on Gateway
        ctx.logger.info(`Verifying ZamaOFT on ${gateway.name}...`);
        await withRetry(
            async () => {
                await execa(pkgManager, ["run", gatewayVerifyScript], {
                    cwd: pkgDir,
                    env: baseEnv,
                    stdio: ["inherit", "inherit", "inherit"],
                });
            },
            {
                maxAttempts: 3,
                initialDelayMs: 10000,
                onRetry: (attempt) => {
                    ctx.logger.warn(
                        `ZamaOFT verification on ${gateway.name} failed, retrying (attempt ${attempt}/3)...`,
                    );
                },
            },
        );
        ctx.logger.success(`Verified ZamaOFT on ${gateway.name}`);
    }

    /**
     * Maps network names to verification script names in package.json
     * @param networkEnvironment The network environment from config (e.g., "testnet", "mainnet")
     * @param chainType Either "ethereum" or "gateway"
     * @returns The verification script name
     */
    private getVerificationScriptName(
        networkEnvironment: "testnet" | "mainnet",
        chainType: "ethereum" | "gateway",
    ): string {
        switch (networkEnvironment) {
            case "testnet":
                return chainType === "ethereum"
                    ? "verify:etherscan:ethereum:sepolia"
                    : "verify:etherscan:gateway:testnet";
            case "mainnet":
                return `verify:etherscan:${chainType}:mainnet`;
            default:
                throw new Error(
                    `Unsupported network environment: ${networkEnvironment}`,
                );
        }
    }
}
