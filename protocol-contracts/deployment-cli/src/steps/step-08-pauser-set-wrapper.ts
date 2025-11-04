import { ValidationError } from "../utils/errors.js";
import { resolveProjectRoot } from "../utils/project-paths.js";
import { withRetry } from "../utils/retry.js";
import { TaskOutputReader } from "../utils/task-output-reader.js";
import {
    BaseStep,
    type DeploymentContext,
    type StepExecutionResult,
} from "./base-step.js";

export class Step08PauserSetWrapper extends BaseStep {
    public readonly id = "step-08";
    public readonly name = "Deploy PauserSetWrapper";
    public readonly description =
        "Deploys PauserSetWrapper on Ethereum and grants pauser role via DAO governance.";
    public readonly dependencies = ["step-04", "step-07"] as const;
    public readonly pkgName = "protocol-contracts/pauserSetWrapper" as const;

    protected async validate(ctx: DeploymentContext): Promise<void> {
        const pauserSetAddress = ctx.env.getAddress("PAUSER_SET_HOST");
        const zamaToken = ctx.env.getAddress("ZAMA_TOKEN");
        if (!pauserSetAddress || !zamaToken) {
            throw new ValidationError(
                "Pauser set address and Zama token must be available before Step 8.",
            );
        }
    }

    protected async execute(
        ctx: DeploymentContext,
    ): Promise<StepExecutionResult> {
        const ethereum = ctx.networks.getEthereum();
        const protocolPk = ctx.env.resolveWalletPrivateKey("protocol_deployer");

        const zamaToken = ctx.env.getAddress("ZAMA_TOKEN");
        const pauserSetAddress = ctx.env.getAddress("PAUSER_SET_HOST");

        const baseEnv = ctx.env.buildTaskEnv({
            PRIVATE_KEY: protocolPk,
            SEPOLIA_RPC_URL: ethereum.rpcUrl,
            CONTRACT_TARGET: zamaToken,
            FUNCTION_SIGNATURE: "pauseMinting()",
            PAUSER_SET: pauserSetAddress,
        });

        await ctx.hardhat.runTask({
            pkg: this.pkgName,
            task: "deploy",
            args: ["--network", ethereum.name],
            env: baseEnv,
        });

        const projectRoot = resolveProjectRoot();
        const reader = new TaskOutputReader(projectRoot);
        const wrapperAddress = reader.readHardhatDeployment(
            this.pkgName,
            ethereum.name,
            "PauserSetWrapper",
        );

        // Grant PAUSING_MINTER_ROLE to PAUSER_SET_WRAPPER on ZAMA_TOKEN
        try {
            await ctx.hardhat.runTask({
                pkg: "protocol-contracts/token",
                task: "zama:erc20:grant:minter_role",
                args: [
                    "--address",
                    wrapperAddress,
                    "--contract-address",
                    zamaToken,
                    "--network",
                    ethereum.name,
                ],
                env: baseEnv,
            });
        } catch (error: unknown) {
            const stderr = ((error as Record<string, unknown>)?.stderr ||
                (error as Record<string, unknown>)?.message ||
                "") as string;
            if (
                String(stderr).includes("already has MINTER_ROLE on contract")
            ) {
            } else {
                throw error;
            }
        }

        return {
            addresses: {
                PAUSER_SET_WRAPPER: wrapperAddress,
            },
        };
    }

    protected async verifyDeployments(
        ctx: DeploymentContext,
        result: StepExecutionResult & {
            addresses: { PAUSER_SET_WRAPPER: string };
        },
    ): Promise<void> {
        const ethereum = ctx.networks.getEthereum();
        const protocolPk = ctx.env.resolveWalletPrivateKey("protocol_deployer");
        const zamaToken = ctx.env.getAddress("ZAMA_TOKEN");
        const pauserSetAddress = ctx.env.getAddress("PAUSER_SET_HOST");
        const wrapperAddress = result.addresses.PAUSER_SET_WRAPPER;

        const baseEnv = ctx.env.buildTaskEnv({
            PRIVATE_KEY: protocolPk,
            SEPOLIA_RPC_URL: ethereum.rpcUrl,
            CONTRACT_TARGET: zamaToken,
            FUNCTION_SIGNATURE: "pauseMinting()",
            PAUSER_SET: pauserSetAddress,
        });

        await withRetry(
            () =>
                ctx.hardhat.runTask({
                    pkg: this.pkgName,
                    task: "task:verifyPauserSetWrapper",
                    args: [
                        "--address",
                        wrapperAddress,
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
                        `PauserSetWrapper verification failed, retrying (attempt ${attempt}/3)...`,
                    );
                },
            },
        );
        ctx.logger.success(`Verified PauserSetWrapper on ${ethereum.name}`);
    }
}
