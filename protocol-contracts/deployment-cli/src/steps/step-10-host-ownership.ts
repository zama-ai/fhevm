import { ethers } from "ethers";
import { ValidationError } from "../utils/errors.js";
import {
    BaseStep,
    type DeploymentContext,
    type StepExecutionResult,
} from "./base-step.js";

export class Step10HostOwnership extends BaseStep {
    public readonly id = "step-10";
    public readonly name = "Transfer Host Ownership to DAO";
    public readonly description =
        "Transfers ACL ownership from deployer to DAO and waits for DAO acceptance.";
    public readonly dependencies = ["step-07", "step-01"] as const;
    public readonly pkgName = "host-contracts" as const;

    protected async execute(
        ctx: DeploymentContext,
    ): Promise<StepExecutionResult> {
        const ethereum = ctx.networks.getEthereum();
        const daoAddress = ctx.env.getAddress("DAO_ADDRESS");
        const aclAddress = ctx.env.getAddress("ACL_ADDRESS");

        const isOwnedByDao = await this.isACLOwnedBy(
            ethereum.rpcUrl,
            aclAddress,
            daoAddress,
        );
        if (isOwnedByDao) {
            ctx.logger.success(
                "ACL is already owned by the DAO. Skipping ownership transfer.",
            );
            return {
                notes: [
                    "Ownership transfer already completed; ACL is owned by the DAO.",
                ],
            };
        }

        if (!daoAddress) {
            throw new ValidationError(
                "DAO address missing. Complete Step 1 first.",
            );
        }
        if (!aclAddress) {
            throw new ValidationError(
                "ACL address missing. Complete Step 7 first.",
            );
        }

        const deployerPk = ctx.env.resolveWalletPrivateKey("deployer");
        ctx.logger.info(
            `Starting ownership transfer of ACL ${aclAddress} to DAO ${daoAddress}`,
        );
        const offerEnv = ctx.env.buildTaskEnv({
            DEPLOYER_PRIVATE_KEY: deployerPk,
            ACL_CONTRACT_ADDRESS: aclAddress,
            RPC_URL: ethereum.rpcUrl,
        });
        await ctx.hardhat.runTask({
            pkg: this.pkgName,
            task: "task:transferHostOwnership",
            args: [
                "--new-owner-address",
                daoAddress,
                "--network",
                ethereum.hostPkgName,
            ],
            env: offerEnv,
        });

        ctx.logger.pending(`Waiting for DAO to accept ACL ownership...`);
        ctx.logger.info(`ACL Address: ${aclAddress}`);
        ctx.logger.info(`DAO Address: ${daoAddress}`);
        ctx.logger.info(
            `Function selector for acceptOwnership(): ${ethers.id("acceptOwnership()").slice(0, 10)}`,
        );

        const confirmed = await ctx.prompt.confirm(
            "Has the DAO executed acceptOwnership() on the ACL contract?",
            false,
        );

        if (!confirmed) {
            throw new ValidationError(
                "Ownership transfer incomplete. DAO must accept ownership to finish Step 10.",
            );
        }

        // Verify that the ACL is now owned by the DAO
        const verified = await this.isACLOwnedBy(
            ethereum.rpcUrl,
            aclAddress,
            daoAddress,
        );
        if (!verified) {
            ctx.logger.error("Could not verify that ACL is owned by DAO.");
            process.exit(1);
        } else {
            ctx.logger.success("Verified: ACL is now owned by the DAO.");
        }

        return {
            notes: ["DAO confirmed to have accepted ACL ownership."],
        };
    }

    /**
     * Check if an ACL contract is owned by a specific address.
     * Reads the 'owner' public state variable via ethers provider.
     */
    private async isACLOwnedBy(
        rpcUrl: string,
        aclAddress: string,
        expectedOwner: string,
    ): Promise<boolean> {
        try {
            const provider = new ethers.JsonRpcProvider(rpcUrl);

            // Create a minimal Ownable contract interface to call owner()
            const ownerableABI = [
                "function owner() public view returns (address)",
            ];
            const contract = new ethers.Contract(
                aclAddress,
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
