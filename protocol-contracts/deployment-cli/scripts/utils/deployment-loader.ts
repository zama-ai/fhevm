import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import type { Abi } from "viem";
import { resolveProjectRoot } from "../../src/utils/project-paths.js";

export interface DeploymentAddresses {
    readonly [key: string]: string;
    DAO_ADDRESS: string;
    GOVERNANCE_OAPP_SENDER: string;
    GOVERNANCE_OAPP_RECEIVER: string;
    ADMIN_MODULE_ADDRESS: string;
    PAUSER_SET_GATEWAY: string;
    PAUSER_SET_WRAPPER: string;
    SAFE_ADDRESS: string;
}

export function loadDeploymentAddresses(
    stateFileName: string,
): DeploymentAddresses {
    const addressesPath = path.resolve(
        resolveProjectRoot(),
        "protocol-contracts/deployment-cli/deployment-state",
        stateFileName,
    );

    if (!existsSync(addressesPath)) {
        throw new Error(
            `Deployment addresses file not found at ${addressesPath}. Make sure you ran the deployment CLI first: bun run start deploy --network testnet`,
        );
    }

    const addresses = JSON.parse(
        readFileSync(addressesPath, "utf-8"),
    ) as DeploymentAddresses;

    return addresses;
}

export function validateDeployment(addresses: DeploymentAddresses) {
    const requiredFields = [
        "DAO_ADDRESS",
        "GOVERNANCE_OAPP_SENDER",
        "GOVERNANCE_OAPP_RECEIVER",
        "ADMIN_MODULE_ADDRESS",
        "PAUSER_SET_GATEWAY",
        "PAUSER_SET_WRAPPER",
        "SAFE_ADDRESS",
    ];

    const missingFields = requiredFields.filter(
        (field) => !addresses[field] || addresses[field] === "",
    );

    if (missingFields.length > 0) {
        throw new Error(
            `Missing required deployment addresses: ${missingFields.join(", ")}`,
        );
    }
}

export interface ContractABIs {
    governanceOAppSenderAbi: Abi;
    governanceOAppReceiverAbi: Abi;
    pauserSetAbi: Abi;
}

export async function loadContractABIs(): Promise<ContractABIs> {
    const senderArtifactPath = path.resolve(
        resolveProjectRoot(),
        "protocol-contracts/governance/artifacts/contracts/GovernanceOAppSender.sol/GovernanceOAppSender.json",
    );
    const governanceOAppSenderAbi = JSON.parse(
        readFileSync(senderArtifactPath, "utf-8"),
    ).abi;

    const receiverArtifactPath = path.resolve(
        resolveProjectRoot(),
        "protocol-contracts/governance/artifacts/contracts/GovernanceOAppReceiver.sol/GovernanceOAppReceiver.json",
    );
    const governanceOAppReceiverAbi = JSON.parse(
        readFileSync(receiverArtifactPath, "utf-8"),
    ).abi;

    const pauserSetArtifactPath = path.resolve(
        resolveProjectRoot(),
        "host-contracts/artifacts/contracts/immutable/PauserSet.sol/PauserSet.json",
    );
    const pauserSetAbi = JSON.parse(
        readFileSync(pauserSetArtifactPath, "utf-8"),
    ).abi;

    return { governanceOAppSenderAbi, governanceOAppReceiverAbi, pauserSetAbi };
}
