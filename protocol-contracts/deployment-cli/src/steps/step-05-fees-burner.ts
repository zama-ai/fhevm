import { ValidationError } from "../utils/errors.js";
import { resolveProjectRoot } from "../utils/project-paths.js";
import { withRetry } from "../utils/retry.js";
import { TaskOutputReader } from "../utils/task-output-reader.js";
import {
    BaseStep,
    type DeploymentContext,
    type StepExecutionResult,
} from "./base-step.js";

export class Step05FeesBurner extends BaseStep {
    public readonly id = "step-05";
    public readonly name = "Deploy Fee Burning Contracts";
    public readonly description =
        "Deploys ProtocolFeesBurner on Ethereum and FeesSenderToBurner on Gateway, optionally verifying both.";
    public readonly dependencies = ["step-04"] as const;
    public readonly pkgName = "protocol-contracts/feesBurner" as const;

    protected async validate(ctx: DeploymentContext): Promise<void> {
        if (!ctx.env.getAddress("ZAMA_TOKEN")) {
            throw new ValidationError(
                "Zama token must be deployed before Step 5.",
            );
        }
        if (!ctx.env.getAddress("ZAMA_OFT")) {
            throw new ValidationError(
                "Zama OFT must be deployed before Step 5.",
            );
        }
    }

    protected async execute(
        ctx: DeploymentContext,
    ): Promise<StepExecutionResult> {
        const ethereum = ctx.networks.getEthereum();
        const gateway = ctx.networks.getGateway();
        const protocolPk = ctx.env.resolveWalletPrivateKey("protocol_deployer");
        const tokenAddress = ctx.env.getAddress("ZAMA_TOKEN");
        const oftAddress = ctx.env.getAddress("ZAMA_OFT");

        let protocolFeesBurner = ctx.env.getAddress("PROTOCOL_FEES_BURNER");
        let feesSenderToBurner = ctx.env.getAddress("FEES_SENDER_TO_BURNER");

        const baseEnv = ctx.env.buildTaskEnv({
            PRIVATE_KEY: protocolPk,
            SEPOLIA_RPC_URL: ethereum.rpcUrl,
            RPC_URL_ZAMA_GATEWAY_TESTNET: gateway.rpcUrl,
            ZAMA_ERC20_ADDRESS: tokenAddress,
            ZAMA_OFT_ADDRESS: oftAddress,
        });

        if (!protocolFeesBurner) {
            await ctx.hardhat.runTask({
                pkg: this.pkgName,
                task: "deploy",
                args: [
                    "--tags",
                    "ProtocolFeesBurner",
                    "--network",
                    ethereum.name,
                ],
                env: baseEnv,
            });

            const projectRoot = resolveProjectRoot();
            const reader = new TaskOutputReader(projectRoot);
            protocolFeesBurner = reader.readHardhatDeployment(
                this.pkgName,
                ethereum.name,
                "ProtocolFeesBurner",
            );
        }

        if (!feesSenderToBurner) {
            const envWithBurner = ctx.env.buildTaskEnv(baseEnv, {
                PROTOCOL_FEES_BURNER_ADDRESS: protocolFeesBurner,
            });

            await ctx.hardhat.runTask({
                pkg: this.pkgName,
                task: "deploy",
                args: [
                    "--tags",
                    "FeesSenderToBurner",
                    "--network",
                    gateway.name,
                ],
                env: envWithBurner,
            });

            const projectRoot = resolveProjectRoot();
            const reader = new TaskOutputReader(projectRoot);
            feesSenderToBurner = reader.readHardhatDeployment(
                this.pkgName,
                gateway.name,
                "FeesSenderToBurner",
            );
        }

        if (ctx.config.options.auto_verify_contracts) {
            ctx.logger.info("Verifying contracts on block explorers...");

            await withRetry(
                () =>
                    ctx.hardhat.runTask({
                        pkg: this.pkgName,
                        task: "task:verifyProtocolFeesBurner",
                        args: [
                            "--protocol-fees-burner",
                            protocolFeesBurner,
                            "--network",
                            ethereum.name,
                        ],
                        env: baseEnv,
                    }),
                {
                    maxAttempts: 3,
                    initialDelayMs: 10000,
                    onRetry: (attempt) => {
                        ctx.logger.warn(
                            `ProtocolFeesBurner verification failed, retrying (attempt ${attempt}/3)...`,
                        );
                    },
                },
            );
            ctx.logger.info(`Verified ProtocolFeesBurner on ${ethereum.name}`);

            // Reset ETHERSCAN_API_KEY to force usage of BlockScout API
            baseEnv.ETHERSCAN_API_KEY = "";
            await withRetry(
                () =>
                    ctx.hardhat.runTask({
                        pkg: this.pkgName,
                        task: "task:verifyFeesSenderToBurner",
                        args: [
                            "--fees-sender-to-burner",
                            feesSenderToBurner,
                            "--network",
                            gateway.name,
                        ],
                        env: {
                            ...baseEnv,
                            PROTOCOL_FEES_BURNER_ADDRESS: protocolFeesBurner,
                        },
                    }),
                {
                    maxAttempts: 3,
                    initialDelayMs: 10000,
                    onRetry: (attempt) => {
                        ctx.logger.warn(
                            `FeesSenderToBurner verification failed, retrying (attempt ${attempt}/3)...`,
                        );
                    },
                },
            );
            ctx.logger.info(`Verified FeesSenderToBurner on ${gateway.name}`);
        }

        return {
            addresses: {
                PROTOCOL_FEES_BURNER: protocolFeesBurner,
                FEES_SENDER_TO_BURNER: feesSenderToBurner,
            },
        };
    }
}
