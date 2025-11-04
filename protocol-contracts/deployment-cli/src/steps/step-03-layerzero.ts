import path from "node:path";
import { execa } from "execa";
import type { PackageName } from "../tasks/hardhat-runner.js";
import { ValidationError } from "../utils/errors.js";
import { resolveProjectRoot } from "../utils/project-paths.js";
import { withRetry } from "../utils/retry.js";
import { TaskOutputReader } from "../utils/task-output-reader.js";
import {
    BaseStep,
    type DeploymentContext,
    type StepExecutionResult,
} from "./base-step.js";

export class Step03LayerzeroLink extends BaseStep {
    public readonly id = "step-03";
    public readonly name = "Link DAO and Safe via LayerZero";
    public readonly description =
        "Deploys GovernanceOAppSender/Receiver, wires OApps, configures Safe integration, and transfers ownership to DAO/Safe.";
    public readonly dependencies = ["step-01", "step-02"] as const;
    public readonly pkgName: PackageName = "protocol-contracts/governance";

    protected async validate(ctx: DeploymentContext): Promise<void> {
        const dao = ctx.env.getAddress("DAO_ADDRESS");
        const safe = ctx.env.getAddress("SAFE_ADDRESS");

        if (!dao) {
            throw new ValidationError(
                "DAO address missing. Complete Step 1 first.",
            );
        }
        if (!safe) {
            throw new ValidationError(
                "Safe address missing. Complete Step 2 first.",
            );
        }
    }

    protected async execute(
        ctx: DeploymentContext,
    ): Promise<StepExecutionResult> {
        const ethereumNetwork = ctx.networks.getEthereum();
        const gatewayNetwork = ctx.networks.getGateway();
        const protocolPk = ctx.env.resolveWalletPrivateKey("protocol_deployer");
        const daoAddress = ctx.env.getAddress("DAO_ADDRESS");
        const safeAddress = ctx.env.getAddress("SAFE_ADDRESS");

        if (!daoAddress || !safeAddress) {
            throw new ValidationError(
                "DAO and Safe addresses are required before executing Step 3.",
            );
        }

        const baseEnv = ctx.env.buildTaskEnv({
            PRIVATE_KEY: protocolPk,
            SEPOLIA_RPC_URL: ethereumNetwork.rpcUrl,
            RPC_URL_ZAMA_GATEWAY_TESTNET: gatewayNetwork.rpcUrl,
            DAO_ADDRESS: daoAddress,
            SAFE_ADDRESS: safeAddress,
            ETHERSCAN_API: ethereumNetwork.explorerApiKey,
            BLOCKSCOUT_API: gatewayNetwork.blockscoutApiUrl,
        });

        const projectRoot = resolveProjectRoot();
        const reader = new TaskOutputReader(projectRoot);

        // Step 1: Deploy GovernanceOAppSender on Ethereum
        ctx.logger.info("Deploying GovernanceOAppSender on Ethereum...");
        await ctx.hardhat.runTask({
            pkg: this.pkgName,
            task: "lz:deploy",
            args: [
                "--networks",
                ethereumNetwork.name,
                "--ci",
                "--tags",
                "GovernanceOAppSender",
            ],
            env: baseEnv,
        });

        const senderAddress = reader.readHardhatDeployment(
            this.pkgName,
            ethereumNetwork.name,
            "GovernanceOAppSender",
        );
        ctx.env.recordAddress("GOVERNANCE_OAPP_SENDER", senderAddress, this.id);
        ctx.logger.success(`Deployed GovernanceOAppSender at ${senderAddress}`);

        // Step 2: Deploy GovernanceOAppReceiver on Gateway
        ctx.logger.info("Deploying GovernanceOAppReceiver on Gateway...");
        await ctx.hardhat.runTask({
            pkg: this.pkgName,
            task: "lz:deploy",
            args: [
                "--networks",
                gatewayNetwork.name,
                "--ci",
                "--tags",
                "GovernanceOAppReceiver",
            ],
            env: baseEnv,
        });

        const receiverAddress = reader.readHardhatDeployment(
            this.pkgName,
            gatewayNetwork.name,
            "GovernanceOAppReceiver",
        );
        ctx.env.recordAddress(
            "GOVERNANCE_OAPP_RECEIVER",
            receiverAddress,
            this.id,
        );
        ctx.logger.success(
            `Deployed GovernanceOAppReceiver at ${receiverAddress}`,
        );

        // Step 3: Wire OApps together (configure LayerZero messaging between chains)
        // This also sets the delegate according to the layerzero config file.
        const layerzeroConfig = ctx.networks.getLayerzeroConfig();
        ctx.logger.info("Wiring OApps together via LayerZero...");
        await ctx.hardhat.runTask({
            pkg: this.pkgName,
            task: "lz:oapp:wire",
            args: ["--oapp-config", layerzeroConfig, "--ci"],
            env: baseEnv,
        });
        ctx.logger.success(
            "Wired GovernanceOAppSender and GovernanceOAppReceiver",
        );

        // Step 4: Transfer ownership of GovernanceOAppSender and GovernanceOAppReceiver to DAO and Safe
        ctx.logger.info(
            `Transferring GovernanceOAppSender ownership to DAO (${daoAddress}) and...`,
        );
        ctx.logger.info(
            `Transferring GovernanceOAppReceiver ownership to Safe (${safeAddress})...`,
        );
        await ctx.hardhat.runTask({
            pkg: this.pkgName,
            task: "lz:ownable:transfer-ownership",
            args: ["--oapp-config", layerzeroConfig, "--ci"],
            env: baseEnv,
        });
        ctx.logger.success("Transferred GovernanceOAppSender ownership to DAO");

        return {
            addresses: {
                GOVERNANCE_OAPP_SENDER: senderAddress,
                GOVERNANCE_OAPP_RECEIVER: receiverAddress,
            },
            notes: [
                "Safe module integration and delegation transfers handled in governance contract deployment",
                `Run E2E tests manually: cd ${this.pkgName} && npx hardhat test`,
            ],
        };
    }

    protected async verifyDeployments(ctx: DeploymentContext): Promise<void> {
        const ethereumNetwork = ctx.networks.getEthereum();
        const gatewayNetwork = ctx.networks.getGateway();
        const networkEnvironment = ctx.networks.getSelectedEnvironment();
        const protocolPk = ctx.env.resolveWalletPrivateKey("protocol_deployer");
        const daoAddress = ctx.env.getAddress("DAO_ADDRESS");
        const safeAddress = ctx.env.getAddress("SAFE_ADDRESS");

        const baseEnv = ctx.env.buildTaskEnv({
            PRIVATE_KEY: protocolPk,
            SEPOLIA_RPC_URL: ethereumNetwork.rpcUrl,
            RPC_URL_ZAMA_GATEWAY_TESTNET: gatewayNetwork.rpcUrl,
            DAO_ADDRESS: daoAddress,
            SAFE_ADDRESS: safeAddress,
            ETHERSCAN_API: ethereumNetwork.explorerApiKey,
            BLOCKSCOUT_API: gatewayNetwork.blockscoutApiUrl,
        });

        ctx.logger.info("Verifying LayerZero contracts on block explorers...");

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

        if (ethereumVerifyScript) {
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
                            `GovernanceOAppSender verification failed, retrying (attempt ${attempt}/3)...`,
                        );
                    },
                },
            );
            ctx.logger.success(
                `Verified GovernanceOAppSender on ${ethereumNetwork.name}`,
            );
        } else {
            ctx.logger.warn(
                `No verification script found for Ethereum network: ${ethereumNetwork.name}`,
            );
        }

        if (gatewayVerifyScript) {
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
                            `GovernanceOAppReceiver verification failed, retrying (attempt ${attempt}/3)...`,
                        );
                    },
                },
            );
            ctx.logger.success(
                `Verified GovernanceOAppReceiver on ${gatewayNetwork.name}`,
            );
        } else {
            ctx.logger.warn(
                `No verification script found for Gateway network: ${gatewayNetwork.name}`,
            );
        }
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
                return `verify:etherscan:${chainType}:testnet`;
            case "mainnet":
                return `verify:etherscan:${chainType}:mainnet`;
            default:
                throw new Error(
                    `Unsupported network environment: ${networkEnvironment}`,
                );
        }
    }
}
