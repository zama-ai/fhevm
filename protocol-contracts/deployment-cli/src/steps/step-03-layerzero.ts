import type { PackageName } from "../tasks/hardhat-runner.js";
import { ValidationError } from "../utils/errors.js";
import { resolveProjectRoot } from "../utils/project-paths.js";
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
        const existingSender = ctx.env.getAddress("GOVERNANCE_OAPP_SENDER");
        const existingReceiver = ctx.env.getAddress("GOVERNANCE_OAPP_RECEIVER");

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
        });

        const projectRoot = resolveProjectRoot();
        const reader = new TaskOutputReader(projectRoot);

        // Step 1: Deploy GovernanceOAppSender on Ethereum
        let senderAddress = existingSender;
        if (!senderAddress) {
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

            senderAddress = reader.readHardhatDeployment(
                this.pkgName,
                ethereumNetwork.name,
                "GovernanceOAppSender",
            );
            ctx.env.recordAddress(
                "GOVERNANCE_OAPP_SENDER",
                senderAddress,
                this.id,
            );
            ctx.logger.success(
                `Deployed GovernanceOAppSender at ${senderAddress}`,
            );
        } else {
            ctx.logger.info(
                `Using existing GovernanceOAppSender at ${senderAddress}`,
            );
        }

        // Step 2: Deploy GovernanceOAppReceiver on Gateway
        let receiverAddress = existingReceiver;
        if (!receiverAddress) {
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

            receiverAddress = reader.readHardhatDeployment(
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
        } else {
            ctx.logger.info(
                `Using existing GovernanceOAppReceiver at ${receiverAddress}`,
            );
        }

        // Step 3: Wire OApps together (configure LayerZero messaging between chains)
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

        // Note: Safe module integration (AdminModuleMock) and delegation transfers are handled
        // in the governance contracts deployment scripts, not in this CLI step.
        // E2E tests can be run separately via: npx hardhat test (in protocol-contracts/governance)

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
}
