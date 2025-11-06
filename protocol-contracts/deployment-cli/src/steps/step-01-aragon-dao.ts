import { getAddress } from "ethers";
import { ADDRESS_REGEX } from "../config/schema.js";
import { ValidationError } from "../utils/errors.js";
import {
    BaseStep,
    type DeploymentContext,
    type StepExecutionResult,
} from "./base-step.js";

export class Step01AragonDao extends BaseStep {
    public readonly id = "step-01";
    public readonly name = "Deploy Aragon DAO on Ethereum";
    public readonly description =
        "Creates the governance DAO and records its address.";
    public readonly dependencies = [] as const;
    // No pkgName needed for this manual step

    protected async execute(
        ctx: DeploymentContext,
    ): Promise<StepExecutionResult> {
        const infoLines = [
            "Manual action required using the Aragon App:",
            "1. Navigate to https://app.aragon.org/ with the governance deployer wallet.",
            "2. Create a new DAO and install the native multisig plugin.",
            "3. Add governance members and set threshold as needed.",
            "4. Copy the DAO address once deployment is confirmed.",
        ];

        for (const line of infoLines) {
            ctx.logger.info(line);
        }

        const confirmed = await ctx.prompt.confirm(
            "Have you completed the DAO deployment steps?",
            true,
        );
        if (!confirmed) {
            throw new ValidationError(
                "DAO deployment must be completed to continue.",
            );
        }

        const daoAddress = await ctx.prompt.input(
            "Enter the deployed DAO address",
        );
        if (!ADDRESS_REGEX.test(daoAddress)) {
            throw new ValidationError("Invalid DAO address provided.");
        }

        return {
            addresses: {
                DAO_ADDRESS: getAddress(daoAddress),
            },
        };
    }
}
