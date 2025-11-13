import { resolveProjectRoot } from "../utils/project-paths.js";
import { TaskOutputReader } from "../utils/task-output-reader.js";
import {
    BaseStep,
    type DeploymentContext,
    type StepExecutionResult,
} from "./base-step.js";

export class Step02Safe extends BaseStep {
    public readonly id = "step-02";
    public readonly name = "Deploy Governance Safe on Gateway";
    public readonly description =
        "Deploys Safe multisig wallet (SafeL2Proxy) on the Gateway network. AdminModule is deployed and wired in Step 03 once the Receiver exists.";
    public readonly dependencies = [] as const;
    public readonly pkgName = "protocol-contracts/safe" as const;

    protected async execute(
        ctx: DeploymentContext,
    ): Promise<StepExecutionResult> {
        const gateway = ctx.networks.getGateway();
        const protocolDeployerPk =
            ctx.env.resolveWalletPrivateKey("protocol_deployer");
        const projectRoot = resolveProjectRoot();
        const reader = new TaskOutputReader(projectRoot);

        const baseEnv = ctx.env.buildTaskEnv({
            PRIVATE_KEY: protocolDeployerPk,
            RPC_URL_ZAMA_GATEWAY_TESTNET: gateway.rpcUrl,
        });

        // Check if Safe is already deployed
        const taskOutput = new TaskOutputReader(projectRoot);
        let safeProxyAddress: `0x${string}` | undefined;
        try {
            safeProxyAddress = taskOutput.readHardhatDeployment(
                this.pkgName,
                gateway.name,
                "SafeL2Proxy",
            );
        } catch (_error) {
            safeProxyAddress = undefined;
        }

        if (safeProxyAddress) {
            ctx.logger.info(
                "Safe artifact found, reading existing deployment...",
            );
            ctx.logger.success(
                `Using existing Safe proxy at ${safeProxyAddress}`,
            );
        } else {
            // Compile before deploying
            ctx.logger.info("Compiling Safe contracts...");
            await ctx.hardhat.runTask({
                pkg: this.pkgName,
                task: "compile",
            });

            // Step 1: Deploy Safe (SafeL2, SafeProxyFactory, SafeL2Proxy)
            ctx.logger.info(
                "Deploying Safe multisig wallet (SafeL2Proxy) on Gateway...",
            );
            await ctx.hardhat.runTask({
                pkg: this.pkgName,
                task: "task:deploySafe",
                args: ["--network", gateway.name],
                env: baseEnv,
            });

            safeProxyAddress = reader.readHardhatDeployment(
                this.pkgName,
                gateway.name,
                "SafeL2Proxy",
            );
            ctx.logger.success(`Deployed Safe proxy at ${safeProxyAddress}`);
        }
        const safeAddress = reader.readHardhatDeployment(
            this.pkgName,
            gateway.name,
            "SafeL2",
        );
        ctx.env.recordAddress("SAFE_ADDRESS", safeAddress, this.id);
        return {
            addresses: {
                SAFE_PROXY_ADDRESS: safeProxyAddress,
                SAFE_ADDRESS: safeAddress,
            },
            notes: [
                "Safe proxy deployed as the governance multisig wallet",
                "AdminModule will be deployed and enabled in Step 03 after the GovernanceOAppReceiver is deployed (it must use the receiver as ADMIN_ACCOUNT)",
            ],
        };
    }

    protected async verifyDeployments(
        ctx: DeploymentContext,
        _result: StepExecutionResult,
    ): Promise<void> {
        const gateway = ctx.networks.getGateway();
        const protocolDeployerPk =
            ctx.env.resolveWalletPrivateKey("protocol_deployer");
        const baseEnv = ctx.env.buildTaskEnv({
            PRIVATE_KEY: protocolDeployerPk,
            RPC_URL_ZAMA_GATEWAY_TESTNET: gateway.rpcUrl,
        });

        ctx.logger.info("Verifying Safe contracts...");
        try {
            await ctx.hardhat.runTask({
                pkg: this.pkgName,
                task: "task:verifySafe",
                args: ["--network", gateway.name],
                env: baseEnv,
            });
            ctx.logger.success("Safe contracts verified successfully");
        } catch (_error) {
            ctx.logger.warn(
                "Safe verification failed (this may be acceptable if already verified)",
            );
            ctx.logger.error(
                _error instanceof Error ? _error.message : String(_error),
            );
        }

        // AdminModule verification is performed in Step 03 after it is deployed with the correct ADMIN_ACCOUNT (receiver)
    }
}
