import { ValidationError } from "../utils/errors.js";
import { resolveProjectRoot } from "../utils/project-paths.js";
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
        const safeAddress = ctx.env.getAddress("SAFE_ADDRESS");
        if (!safeAddress) {
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
        const safeAddress = ctx.env.getAddress("SAFE_ADDRESS");

        if (!daoAddress || !safeAddress) {
            throw new ValidationError(
                "Required prerequisite addresses missing before token deployment.",
            );
        }

        const recipients = ctx.config.protocol.token.recipients;
        const envVars: Record<string, string> = {
            PRIVATE_KEY: protocolPk,
            INITIAL_ADMIN: ctx.config.wallets.protocol_deployer.address,
            SEPOLIA_RPC_URL: ethereum.rpcUrl,
            RPC_URL_ZAMA_GATEWAY_TESTNET: gateway.rpcUrl,
            DAO_ADDRESS: daoAddress,
            SAFE_ADDRESS: safeAddress,
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
        const args = [
            "--preset",
            presetName,
            ...(ctx.config.options.auto_verify_contracts
                ? ["--verify", "true"]
                : []),
        ];
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
                    safeAddress,
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
                    safeAddress,
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
}
