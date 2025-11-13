import crypto from "node:crypto";
import fs from "node:fs";
import dotenv from "dotenv";
import { ethers } from "ethers";
import YAML from "yaml";
import { z } from "zod";
import { ConfigNotFoundError, ValidationError } from "../utils/errors.js";

dotenv.config();

export const ADDRESS_REGEX = /^0x[a-fA-F0-9]{40}$/;
export const PRIVATE_KEY_REGEX = /^0x[a-fA-F0-9]{64}$/;

const AddressSchema = z
    .string()
    .trim()
    .regex(ADDRESS_REGEX, { message: "Expected valid EVM address" })
    .transform((value) => ethers.getAddress(value));

const EthereumNetworkSchema = z
    .object({
        name: z.string().min(1, "Network name is required"),
        host_pkg_name: z
            .string()
            .min(
                1,
                'Host package network name is required (e.g., "sepolia" for host-contracts Hardhat tasks)',
            ),
        rpc_url_env: z
            .string()
            .min(1, "Environment variable name for RPC URL is required"),
        chain_id: z.number().positive(),
        explorer_url: z.string().url().optional(),
        etherscan_api_key: z.string().optional(),
    })
    .transform((value) => {
        const envRpcUrl = process.env[value.rpc_url_env] || "";
        try {
            new URL(envRpcUrl);
        } catch {
            throw new ValidationError(`RPC URL is invalid: ${envRpcUrl}`);
        }
        return {
            name: value.name,
            host_pkg_name: value.host_pkg_name,
            rpc_url: envRpcUrl,
            chain_id: value.chain_id,
            explorer_url: value.explorer_url,
            etherscan_api_key: process.env[value.etherscan_api_key ?? ""],
        };
    });
const GatewayNetworkSchema = z
    .object({
        name: z.string().min(1, "Network name is required"),
        gateway_pkg_name: z
            .string()
            .min(
                1,
                'Gateway package network name is required (e.g., "testnet" for gateway-contracts Hardhat tasks)',
            ),
        rpc_url_env: z
            .string()
            .min(1, "Environment variable name for RPC URL is required"),
        blockscout_api_url: z
            .string()
            .url("Blockscout API URL must be a valid URL"),
        chain_id: z.number().positive(),
        explorer_url: z.string().url().optional(),
        etherscan_api_key: z.string().optional(),
    })
    .transform((value) => {
        const envRpcUrl = process.env[value.rpc_url_env] || "";
        try {
            new URL(envRpcUrl);
        } catch {
            throw new ValidationError(`RPC URL is invalid: ${envRpcUrl}`);
        }
        return {
            name: value.name,
            gateway_pkg_name: value.gateway_pkg_name,
            rpc_url: envRpcUrl,
            blockscout_api_url: value.blockscout_api_url,
            chain_id: value.chain_id,
            explorer_url: value.explorer_url,
            etherscan_api_key: process.env[value.etherscan_api_key ?? ""],
        };
    });

const WalletSchema = z
    .object({
        description: z.string().optional(),
        private_key_env: z
            .string()
            .min(1, "Environment variable name for private key is required"),
    })
    .transform((value) => {
        const envPrivateKey = process.env[value.private_key_env] || "";
        if (!PRIVATE_KEY_REGEX.test(envPrivateKey)) {
            throw new ValidationError(
                `Private key of ${value.description} must be a 32-byte hex string`,
            );
        }
        const address = ethers.computeAddress(envPrivateKey);
        return {
            ...value,
            address,
        };
    });

const OperatorSchema = z.object({
    name: z.string().min(1),
    pauser: z
        .object({
            address: AddressSchema,
        })
        .optional(),
});

const PricingSchema = z
    .object({
        input_verification: z.string().optional(),
        public_decryption: z.string().optional(),
        user_decryption: z.string().optional(),
    })
    .default({});

const KmsNodeSchema = z.object({
    tx_sender_address: AddressSchema,
    signer_address: AddressSchema,
    ip_address: z.string().min(1),
    storage_url: z.string().url(),
});

const CoprocessorSchema = z.object({
    tx_sender_address: AddressSchema,
    signer_address: AddressSchema,
    s3_bucket_url: z.string().url(),
});

const CustodianSchema = z.object({
    tx_sender_address: AddressSchema,
    signer_address: AddressSchema,
    encryption_key: z.string().regex(/^0x[a-fA-F0-9]{128}$/, {
        message:
            "Encryption key must be a valid 64-byte hex string (0x + 128 hex chars)",
    }),
});

const TokenRecipientSchema = z.object({
    address: AddressSchema,
    amount: z.string().min(1, "Amount is required for token recipient"),
});

const ThresholdSchema = z
    .object({
        mpc: z.number().int().nonnegative().optional(),
        public_decryption: z.number().int().nonnegative().optional(),
        user_decryption: z.number().int().nonnegative().optional(),
        kms_generation: z.number().int().nonnegative().optional(),
        coprocessor: z.number().int().nonnegative().optional(),
    })
    .default({});

const DeploymentOptionsSchema = z
    .object({
        auto_verify_contracts: z.boolean().default(true),
    })
    .default({
        auto_verify_contracts: true,
    });

const ProtocolSchema = z.object({
    name: z.string().min(1),
    website: z.string().url(),
    governance: z
        .object({
            dao_multisig_threshold: z.string().optional(),
        })
        .default({}),
    token: z
        .object({
            recipients: z.array(TokenRecipientSchema).default([]),
        })
        .default({}),
    thresholds: ThresholdSchema,
    pricing: PricingSchema,
    kms_nodes: z.array(KmsNodeSchema).default([]),
    coprocessors: z.array(CoprocessorSchema).default([]),
    custodians: z.array(CustodianSchema).default([]),
});

const MetadataSchema = z.object({
    deployment_name: z.string().min(1),
});

const EnvironmentNetworksSchema = z.object({
    ethereum: EthereumNetworkSchema,
    gateway: GatewayNetworkSchema,
    layerzero_config: z
        .string()
        .min(1, "LayerZero config file name is required")
        .default("layerzero.config.testnet.ts"),
});

export const DeploymentConfigSchema = z.object({
    metadata: MetadataSchema,
    networks: z
        .record(
            z.string().min(1, "Environment name is required"),
            EnvironmentNetworksSchema,
        )
        .refine(
            (networks) => Object.keys(networks).length > 0,
            "At least one network environment must be defined",
        ),
    wallets: z.object({
        protocol_deployer: WalletSchema,
        deployer: WalletSchema,
        governance_deployer: WalletSchema,
    }),
    operators: z.array(OperatorSchema).min(1),
    protocol: ProtocolSchema,
    options: DeploymentOptionsSchema,
});

export type DeploymentConfig = z.infer<typeof DeploymentConfigSchema>;
export type DeploymentOptions = z.infer<typeof DeploymentOptionsSchema>;

export function loadDeploymentConfig(configPath: string): {
    config: DeploymentConfig;
    hash: string;
} {
    if (!fs.existsSync(configPath)) {
        throw new ConfigNotFoundError(configPath);
    }

    const raw = fs.readFileSync(configPath, "utf-8");
    const data = YAML.parse(raw, { uniqueKeys: false });

    const parsed = DeploymentConfigSchema.safeParse(data);

    if (!parsed.success) {
        const formatted = parsed.error.errors
            .map((err) => `${err.path.join(".")}: ${err.message}`)
            .join("\n");
        throw new ValidationError(
            `Invalid deployment configuration:\n${formatted}`,
        );
    }

    const normalized = parsed.data;
    const hash = crypto
        .createHash("sha256")
        .update(JSON.stringify(normalized))
        .digest("hex");

    return { config: normalized, hash };
}
