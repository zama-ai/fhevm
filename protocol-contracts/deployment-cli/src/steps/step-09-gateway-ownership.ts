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
        const safeAddress = ctx.env.getAddress("SAFE_ADDRESS");

        if (!gatewayConfig) {
            throw new ValidationError(
                "GatewayConfig address not found. Ensure Step 6 completed.",
            );
        }
        if (!safeAddress) {
            throw new ValidationError(
                "Safe address not found. Ensure Step 2 completed.",
            );
        }

        // Idempotence check: verify if ownership transfer is already complete
        const isOwnedBySafe = await this.isGatewayConfigOwnedBy(
            gateway.rpcUrl,
            gatewayConfig,
            safeAddress,
        );
        if (isOwnedBySafe) {
            ctx.logger.success(
                "GatewayConfig is already owned by the Safe. Skipping ownership transfer.",
            );
            return {
                status: "completed",
                notes: [
                    "Ownership transfer already completed; GatewayConfig is owned by the Safe.",
                ],
            };
        }

        const deployerPk = ctx.env.resolveWalletPrivateKey("deployer");
        ctx.logger.info(
            `Starting ownership transfer of GatewayConfig ${gatewayConfig} to Safe ${safeAddress}`,
        );

        const offerEnv = ctx.env.buildTaskEnv({
            DEPLOYER_PRIVATE_KEY: deployerPk,
            GATEWAY_CONFIG_ADDRESS: gatewayConfig,
            OWNER_SAFE_SMART_ACCOUNT_PROXY_ADDRESS: safeAddress,
            RPC_URL: gateway.rpcUrl,
        });

        const fixedGatewayNetwork = gateway.gatewayPkgName!;

        // Step 1: Transfer ownership to Safe
        await ctx.hardhat.runTask({
            pkg: this.pkgName,
            task: "task:transferGatewayOwnership",
            args: [
                "--new-owner-address",
                safeAddress,
                "--network",
                fixedGatewayNetwork,
            ],
            env: offerEnv,
        });

        // Step 2: Check Safe balance
        const safeBalance = parseFloat(
            await this.checkSafeBalance(gateway.rpcUrl, safeAddress),
        );
        ctx.logger.info(
            `Safe Balance Check - Address: ${safeAddress}, Balance: ${safeBalance} ETH`,
        );

        if (safeBalance === 0) {
            ctx.logger.warn(
                `Safe is not funded yet! Please fund it on the Gateway network.`,
            );
            const proceed = await ctx.prompt.confirm(
                "Safe has no balance. Please fund it first on the Gateway network. Continue when funded? (y/n)",
            );
            if (!proceed) {
                return {
                    status: "pending",
                    notes: [
                        "Waiting for Safe to be funded before accepting ownership.",
                    ],
                };
            }

            // Re-check balance after user confirms funding
            const updatedBalance = await this.checkSafeBalance(
                gateway.rpcUrl,
                safeAddress,
            );
            ctx.logger.info(`Updated Safe Balance: ${updatedBalance} ETH`);
        }

        // Step 3: Accept ownership. At this point, the Safe is still owned by the deployer private key.
        await ctx.hardhat.runTask({
            pkg: this.pkgName,
            task: "task:acceptGatewayOwnershipFromSafeSmartAccount",
            args: [
                "--owner-private-keys",
                JSON.stringify([deployerPk]),
                "--network",
                fixedGatewayNetwork,
            ],
            env: offerEnv,
        });

        return {
            status: "completed",
        };
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

            // Create a minimal Ownable contract interface to call owner()
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

    private async checkSafeBalance(
        rpcUrl: string,
        safeAddress: string,
    ): Promise<string> {
        try {
            const response = await fetch(rpcUrl, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
                    jsonrpc: "2.0",
                    method: "eth_getBalance",
                    params: [safeAddress, "latest"],
                    id: 1,
                }),
            });

            const data = (await response.json()) as {
                result?: string;
                error?: unknown;
            };
            if (data.error) {
                throw new ValidationError(
                    `RPC error checking balance: ${JSON.stringify(data.error)}`,
                );
            }

            if (!data.result) {
                throw new ValidationError("No balance data returned from RPC");
            }

            const balanceWei = BigInt(data.result);
            const balanceEth = Number(balanceWei) / 1e18;
            return balanceEth.toFixed(4);
        } catch (error) {
            throw new ValidationError(
                `Failed to check Safe balance at ${safeAddress}: ${error instanceof Error ? error.message : String(error)}`,
            );
        }
    }
}
