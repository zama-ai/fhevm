import path from "node:path";
import { ValidationError } from "../utils/errors.js";
import { resolveProjectRoot } from "../utils/project-paths.js";
import { withRetry } from "../utils/retry.js";
import { TaskOutputReader } from "../utils/task-output-reader.js";
import {
    BaseStep,
    type DeploymentContext,
    type StepExecutionResult,
} from "./base-step.js";

export class Step06GatewayContracts extends BaseStep {
    public readonly id = "step-06";
    public readonly name = "Deploy Gateway Contracts";
    public readonly description =
        "Deploys gateway contracts, registers host chains, and configures pausers.";
    public readonly dependencies = ["step-04", "step-05"] as const;
    public readonly pkgName = "gateway-contracts" as const;

    protected async validate(ctx: DeploymentContext): Promise<void> {
        if (!ctx.env.getAddress("ZAMA_OFT")) {
            throw new ValidationError(
                "Zama OFT must be deployed before Step 6.",
            );
        }
        if (!ctx.env.getAddress("FEES_SENDER_TO_BURNER")) {
            throw new ValidationError(
                "Fees sender to burner must be deployed before Step 6.",
            );
        }
    }

    protected async execute(
        ctx: DeploymentContext,
    ): Promise<StepExecutionResult> {
        const gateway = ctx.networks.getGateway();
        const deployerPk = ctx.env.resolveWalletPrivateKey("protocol_deployer");

        // Build environment variables from config
        const baseEnvVars: Record<string, string> = {
            DEPLOYER_PRIVATE_KEY: deployerPk,
            RPC_URL: gateway.rpcUrl,
            ZAMA_OFT_ADDRESS: ctx.env.getAddress("ZAMA_OFT"),
            FEES_SENDER_TO_BURNER_ADDRESS: ctx.env.getAddress(
                "FEES_SENDER_TO_BURNER",
            ),
            PROTOCOL_NAME: ctx.config.protocol.name,
            PROTOCOL_WEBSITE: ctx.config.protocol.website,
        };

        // Add thresholds
        const thresholds = ctx.config.protocol.thresholds;
        if (thresholds.mpc !== undefined) {
            baseEnvVars.MPC_THRESHOLD = thresholds.mpc.toString();
        }
        if (thresholds.public_decryption !== undefined) {
            baseEnvVars.PUBLIC_DECRYPTION_THRESHOLD =
                thresholds.public_decryption.toString();
        }
        if (thresholds.user_decryption !== undefined) {
            baseEnvVars.USER_DECRYPTION_THRESHOLD =
                thresholds.user_decryption.toString();
        }
        if (thresholds.kms_generation !== undefined) {
            baseEnvVars.KMS_GENERATION_THRESHOLD =
                thresholds.kms_generation.toString();
        }
        if (thresholds.coprocessor !== undefined) {
            baseEnvVars.COPROCESSOR_THRESHOLD =
                thresholds.coprocessor.toString();
        }

        // Add pricing
        const pricing = ctx.config.protocol.pricing;
        if (pricing.input_verification) {
            baseEnvVars.INPUT_VERIFICATION_PRICE = pricing.input_verification;
        }
        if (pricing.public_decryption) {
            baseEnvVars.PUBLIC_DECRYPTION_PRICE = pricing.public_decryption;
        }
        if (pricing.user_decryption) {
            baseEnvVars.USER_DECRYPTION_PRICE = pricing.user_decryption;
        }

        // Add KMS nodes
        const kmsNodes = ctx.config.protocol.kms_nodes;
        baseEnvVars.NUM_KMS_NODES = kmsNodes.length.toString();
        kmsNodes.forEach((node, idx) => {
            baseEnvVars[`KMS_TX_SENDER_ADDRESS_${idx}`] =
                node.tx_sender_address;
            baseEnvVars[`KMS_SIGNER_ADDRESS_${idx}`] = node.signer_address;
            baseEnvVars[`KMS_NODE_IP_ADDRESS_${idx}`] = node.ip_address;
            baseEnvVars[`KMS_NODE_STORAGE_URL_${idx}`] = node.storage_url;
        });

        // Add coprocessors
        const coprocessors = ctx.config.protocol.coprocessors;
        baseEnvVars.NUM_COPROCESSORS = coprocessors.length.toString();
        coprocessors.forEach((coprocessor, idx) => {
            baseEnvVars[`COPROCESSOR_TX_SENDER_ADDRESS_${idx}`] =
                coprocessor.tx_sender_address;
            baseEnvVars[`COPROCESSOR_SIGNER_ADDRESS_${idx}`] =
                coprocessor.signer_address;
            baseEnvVars[`COPROCESSOR_S3_BUCKET_URL_${idx}`] =
                coprocessor.s3_bucket_url;
        });

        // Add custodians
        const custodians = ctx.config.protocol.custodians;
        baseEnvVars.NUM_CUSTODIANS = custodians.length.toString();
        custodians.forEach((custodian, idx) => {
            baseEnvVars[`CUSTODIAN_TX_SENDER_ADDRESS_${idx}`] =
                custodian.tx_sender_address;
            baseEnvVars[`CUSTODIAN_SIGNER_ADDRESS_${idx}`] =
                custodian.signer_address;
            baseEnvVars[`CUSTODIAN_ENCRYPTION_KEY_${idx}`] =
                custodian.encryption_key;
        });

        const baseEnv = ctx.env.buildTaskEnv(baseEnvVars);

        await ctx.hardhat.runTask({
            pkg: this.pkgName,
            task: "task:deployAllGatewayContracts",
            args: ["--network", gateway.gatewayPkgName],
            env: baseEnv,
        });

        // Extract addresses from .env.gateway file written by deployment task
        const projectRoot = resolveProjectRoot();
        const reader = new TaskOutputReader(projectRoot);
        const gatewayEnvPath = path.join(
            projectRoot,
            this.pkgName,
            "addresses",
            ".env.gateway",
        );

        const addressMap = reader.readEnvFile(gatewayEnvPath, {
            GATEWAY_CONFIG_ADDRESS: "GATEWAY_CONFIG",
            PAUSER_SET_ADDRESS: "PAUSER_SET_GATEWAY",
            DECRYPTION_ADDRESS: "DECRYPTION",
            INPUT_VERIFICATION_ADDRESS: "INPUT_VERIFICATION",
            KMS_GENERATION_ADDRESS: "KMS_GENERATION",
            CIPHERTEXT_COMMITS_ADDRESS: "CIPHERTEXT_COMMITS",
            MULTICHAIN_ACL_ADDRESS: "MULTICHAIN_ACL",
            PROTOCOL_PAYMENT_ADDRESS: "PROTOCOL_PAYMENT",
        });

        const pauserAddresses = ctx.config.operators
            .map((operator) => operator.pauser?.address)
            .filter((value): value is string => Boolean(value));

        if (pauserAddresses.length === 0) {
            throw new ValidationError(
                "At least one pauser address is required for Step 5.",
            );
        }

        const pauserEnv: Record<string, string> = {
            NUM_PAUSERS: pauserAddresses.length.toString(),
        };

        pauserAddresses.forEach((address, idx) => {
            pauserEnv[`PAUSER_ADDRESS_${idx}`] = address;
        });

        await ctx.hardhat.runTask({
            pkg: this.pkgName,
            task: "task:addGatewayPausers",
            args: ["--network", gateway.gatewayPkgName],
            env: {
                ...baseEnv,
                ...pauserEnv,
                PAUSER_SET_ADDRESS: addressMap.PAUSER_SET_GATEWAY,
            },
        });

        for (const [key, value] of Object.entries(addressMap)) {
            ctx.env.recordAddress(key, value, this.id);
        }

        return {
            addresses: addressMap,
        };
    }

    protected async verifyDeployments(ctx: DeploymentContext): Promise<void> {
        const gateway = ctx.networks.getGateway();
        const deployerPk = ctx.env.resolveWalletPrivateKey("protocol_deployer");

        const baseEnvVars: Record<string, string> = {
            DEPLOYER_PRIVATE_KEY: deployerPk,
            RPC_URL: gateway.rpcUrl,
            ZAMA_OFT_ADDRESS: ctx.env.getAddress("ZAMA_OFT"),
            FEES_SENDER_TO_BURNER_ADDRESS: ctx.env.getAddress(
                "FEES_SENDER_TO_BURNER",
            ),
            PROTOCOL_NAME: ctx.config.protocol.name,
            PROTOCOL_WEBSITE: ctx.config.protocol.website,
        };

        const baseEnv = ctx.env.buildTaskEnv(baseEnvVars);
        ctx.logger.info("Verifying gateway contracts...");
        await withRetry(
            () =>
                ctx.hardhat.runTask({
                    pkg: this.pkgName,
                    task: "task:verifyAllGatewayContracts",
                    args: [
                        "--network",
                        gateway.gatewayPkgName,
                        "--use-internal-proxy-address",
                        "true",
                    ],
                    env: baseEnv,
                }),
            {
                maxAttempts: 3,
                initialDelayMs: 10000,
                onRetry: (attempt) => {
                    ctx.logger.warn(
                        `Gateway contracts verification failed, retrying (attempt ${attempt}/3)...`,
                    );
                },
            },
        );
        ctx.logger.success("Gateway contracts verified successfully");
    }
}
