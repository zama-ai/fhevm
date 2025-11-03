import fs from "node:fs";
import path from "node:path";
import { ValidationError } from "../utils/errors.js";
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
        "Deploys Safe multisig wallet (SafeL2Proxy) and AdminModule on the Gateway network.";
    public readonly dependencies = [] as const;
    public readonly pkgName = "protocol-contracts/safe" as const;

    protected async execute(
        ctx: DeploymentContext,
    ): Promise<StepExecutionResult> {
        const gateway = ctx.networks.getGateway();
        const deployerAddress = ctx.config.wallets.protocol_deployer.address;
        const deployerPk = ctx.env.resolveWalletPrivateKey("protocol_deployer");
        const projectRoot = resolveProjectRoot();
        const reader = new TaskOutputReader(projectRoot);

        const baseEnv = ctx.env.buildTaskEnv({
            PRIVATE_KEY: deployerPk,
            RPC_URL_ZAMA_GATEWAY_TESTNET: gateway.rpcUrl,
        });

        // Check if Safe is already deployed
        const taskOutput = new TaskOutputReader(projectRoot);
        let safeAddress: string | undefined;
        try {
            safeAddress = taskOutput.readHardhatDeployment(
                this.pkgName,
                gateway.name,
                "SafeL2Proxy",
            );
        } catch (error) {
            safeAddress = undefined;
        }

        if (safeAddress) {
            ctx.logger.info(
                "Safe artifact found, reading existing deployment...",
            );
            ctx.logger.success(`Using existing Safe proxy at ${safeAddress}`);
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

            safeAddress = reader.readHardhatDeployment(
                this.pkgName,
                gateway.name,
                "SafeL2Proxy",
            );
            ctx.logger.success(`Deployed Safe proxy at ${safeAddress}`);
        }

        // Check if AdminModule is already deployed
        let adminModuleAddress: string | undefined;
        try {
            adminModuleAddress = taskOutput.readHardhatDeployment(
                this.pkgName,
                gateway.name,
                "AdminModule",
            );
        } catch (error) {
            adminModuleAddress = undefined;
        }

        if (adminModuleAddress) {
            ctx.logger.info(
                "AdminModule artifact found, reading existing deployment...",
            );
            ctx.logger.success(
                `Using existing AdminModule at ${adminModuleAddress}`,
            );
        } else {
            // Step 2: Deploy AdminModule (allows governance to execute Safe transactions)
            ctx.logger.info(
                "Deploying AdminModule to enable governance control...",
            );
            const adminEnv = ctx.env.buildTaskEnv({
                ...baseEnv,
                ADMIN_ADDRESS: deployerAddress,
                SAFE_ADDRESS: safeAddress,
            });

            await ctx.hardhat.runTask({
                pkg: this.pkgName,
                task: "task:deployAdminModule",
                args: ["--network", gateway.name],
                env: adminEnv,
            });

            adminModuleAddress = reader.readHardhatDeployment(
                this.pkgName,
                gateway.name,
                "AdminModule",
            );
            ctx.logger.success(`Deployed AdminModule at ${adminModuleAddress}`);
        }

        // Step 3: Verify contracts if auto verification is enabled
        if (ctx.config.options.auto_verify_contracts) {
            ctx.logger.info("Verifying Safe contracts...");
            try {
                await ctx.hardhat.runTask({
                    pkg: this.pkgName,
                    task: "task:verifySafe",
                    args: ["--network", gateway.name],
                    env: baseEnv,
                });
                ctx.logger.success("Safe contracts verified successfully");
            } catch (error) {
                ctx.logger.warn(
                    "Safe verification failed (this may be acceptable if already verified)",
                );
            }

            ctx.logger.info("Verifying AdminModule...");
            try {
                const verifyEnv = ctx.env.buildTaskEnv({
                    ...baseEnv,
                    ADMIN_ADDRESS: deployerAddress,
                    SAFE_ADDRESS: safeAddress,
                });
                await ctx.hardhat.runTask({
                    pkg: this.pkgName,
                    task: "task:verifyAdminModule",
                    args: ["--network", gateway.name],
                    env: verifyEnv,
                });
                ctx.logger.success("AdminModule verified successfully");
            } catch (error) {
                ctx.logger.warn(
                    "AdminModule verification failed (this may be acceptable if already verified)",
                );
            }
        }

        return {
            status: "completed",
            addresses: {
                SAFE_ADDRESS: safeAddress,
                ADMIN_MODULE_ADDRESS: adminModuleAddress,
            },
            notes: [
                "Safe proxy deployed as the governance multisig wallet",
                "AdminModule deployed to enable cross-chain proposal execution",
                "Next: EnableAdminModule in Safe (handled in subsequent governance setup)",
            ],
        };
    }
}
