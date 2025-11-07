import { ethers } from "ethers";
import { ValidationError } from "../utils/errors.js";
import {
    BaseStep,
    type DeploymentContext,
    type StepExecutionResult,
} from "./base-step.js";

export class Step09GatewayOwnership extends BaseStep {
    public readonly id = "step-09";
    public readonly name = "Transfer Gateway Ownership to Safe";
    public readonly description =
        "Transfers GatewayConfig ownership from deployer to the Safe using the two-step ownership pattern.";
    public readonly dependencies = ["step-06"] as const;
    public readonly pkgName = "gateway-contracts" as const;

    protected async execute(
        ctx: DeploymentContext,
    ): Promise<StepExecutionResult> {
        const gateway = ctx.networks.getGateway();

        const gatewayConfig = ctx.env.getAddress("GATEWAY_CONFIG");
        const safeProxyAddress = ctx.env.getAddress("SAFE_PROXY_ADDRESS");
        const safeAddress = ctx.env.getAddress("SAFE_ADDRESS");
        if (!gatewayConfig) {
            throw new ValidationError(
                "GatewayConfig address not found. Ensure Step 6 completed.",
            );
        }
        if (!safeProxyAddress || !safeAddress) {
            throw new ValidationError(
                "Safe address not found. Ensure Step 2 completed.",
            );
        }

        // Idempotence check: verify if ownership transfer is already complete
        const isOwnedBySafe = await this.isGatewayConfigOwnedBy(
            gateway.rpcUrl,
            gatewayConfig,
            safeProxyAddress,
        );
        if (isOwnedBySafe) {
            ctx.logger.success(
                "GatewayConfig is already owned by the Safe. Skipping ownership transfer.",
            );
            return {
                notes: [
                    "Ownership transfer already completed; GatewayConfig is owned by the Safe.",
                ],
            };
        }

        const deployerPk = ctx.env.resolveWalletPrivateKey("deployer");
        ctx.logger.info(
            `Starting ownership transfer of GatewayConfig ${gatewayConfig} to Safe ${safeProxyAddress}`,
        );

        const offerEnv = ctx.env.buildTaskEnv({
            DEPLOYER_PRIVATE_KEY: deployerPk,
            GATEWAY_CONFIG_ADDRESS: gatewayConfig,
            RPC_URL: gateway.rpcUrl,
        });

        // Step 1: Transfer ownership to Safe
        await ctx.hardhat.runTask({
            pkg: this.pkgName,
            task: "task:transferGatewayOwnership",
            args: [
                "--new-owner-address",
                safeProxyAddress,
                "--network",
                gateway.gatewayPkgName,
            ],
            env: offerEnv,
        });

        // Step 2: Accept ownership. At this point, the Safe is still owned by the deployer private key.
        const protocolDeployerPk =
            ctx.env.resolveWalletPrivateKey("protocol_deployer");
        const safeOwnerPrivateKeysEnv = JSON.stringify([protocolDeployerPk]);
        await ctx.hardhat.runTask({
            pkg: "protocol-contracts/safe",
            task: "task:acceptOwnership",
            args: ["--address", gatewayConfig, "--network", gateway.name],
            env: {
                ...offerEnv,
                PRIVATE_KEY: protocolDeployerPk,
                SAFE_OWNER_PRIVATE_KEYS: safeOwnerPrivateKeysEnv,
                SAFE_PROXY_ADDRESS: safeProxyAddress,
                SAFE_ADDRESS: safeAddress,
                RPC_URL_ZAMA_GATEWAY_TESTNET: gateway.rpcUrl,
            },
        });

        return {};
    }

    /**
     * Check if a GatewayConfig contract is owned by a specific address.
     * Reads the 'owner' public state variable via ethers provider.
     */
    private async isGatewayConfigOwnedBy(
        rpcUrl: string,
        gatewayConfigAddress: string,
        expectedOwner: string,
    ): Promise<boolean> {
        try {
            const provider = new ethers.JsonRpcProvider(rpcUrl);

            const ownerableABI = [
                "function owner() public view returns (address)",
            ];
            const contract = new ethers.Contract(
                gatewayConfigAddress,
                ownerableABI,
                provider,
            );

            const owner = (await contract.owner()) as string;
            return (
                ethers.getAddress(owner) === ethers.getAddress(expectedOwner)
            );
        } catch {
            // If we can't check ownership (contract doesn't exist or lacks owner()), assume not owned yet
            return false;
        }
    }
}
