import { readFileSync } from "node:fs";
import path from "node:path";
import type { Abi } from "viem";
import { resolveProjectRoot } from "../../src/utils/project-paths.js";

/**
 * Load all contract ABIs needed for E2E tests
 */
export async function loadContractABIs(): Promise<{
    // ACL
    ACLAbi: Abi;

    // Governance
    governanceOAppSenderAbi: Abi;
    governanceOAppReceiverAbi: Abi;

    // Safe + Admin Module
    safeL2Abi: Abi;
    adminModuleAbi: Abi;

    // Token + OFT
    zamaERC20Abi: Abi;
    zamaOFTAdapterAbi: Abi;
    zamaOFTAbi: Abi;

    // Fees Burner
    protocolFeesBurnerAbi: Abi;
    feesSenderToBurnerAbi: Abi;

    // Pauser Set
    pauserSetHostAbi: Abi;
    pauserSetWrapperAbi: Abi;

    // Gateway Config
    gatewayConfigAbi: Abi;
}> {
    const projectRoot = resolveProjectRoot();

    const ACLAbi = JSON.parse(
        readFileSync(
            path.resolve(
                projectRoot,
                "host-contracts/artifacts/contracts/ACL.sol/ACL.json",
            ),
            "utf-8",
        ),
    ).abi;

    const governanceOAppSenderAbi = JSON.parse(
        readFileSync(
            path.resolve(
                projectRoot,
                "protocol-contracts/governance/artifacts/contracts/GovernanceOAppSender.sol/GovernanceOAppSender.json",
            ),
            "utf-8",
        ),
    ).abi;

    const governanceOAppReceiverAbi = JSON.parse(
        readFileSync(
            path.resolve(
                projectRoot,
                "protocol-contracts/governance/artifacts/contracts/GovernanceOAppReceiver.sol/GovernanceOAppReceiver.json",
            ),
            "utf-8",
        ),
    ).abi;

    const safeL2Abi = JSON.parse(
        readFileSync(
            path.resolve(
                projectRoot,
                "protocol-contracts/governance/artifacts/@safe-global/safe-contracts/contracts/SafeL2.sol/SafeL2.json",
            ),
            "utf-8",
        ),
    ).abi;

    const adminModuleAbi = JSON.parse(
        readFileSync(
            path.resolve(
                projectRoot,
                "protocol-contracts/safe/artifacts/contracts/AdminModule.sol/AdminModule.json",
            ),
            "utf-8",
        ),
    ).abi;

    const zamaERC20Abi = JSON.parse(
        readFileSync(
            path.resolve(
                projectRoot,
                "protocol-contracts/token/artifacts/contracts/ZamaERC20.sol/ZamaERC20.json",
            ),
            "utf-8",
        ),
    ).abi;

    const zamaOFTAdapterAbi = JSON.parse(
        readFileSync(
            path.resolve(
                projectRoot,
                "protocol-contracts/token/artifacts/contracts/ZamaOFTAdapter.sol/ZamaOFTAdapter.json",
            ),
            "utf-8",
        ),
    ).abi;

    const zamaOFTAbi = JSON.parse(
        readFileSync(
            path.resolve(
                projectRoot,
                "protocol-contracts/token/artifacts/contracts/ZamaOFT.sol/ZamaOFT.json",
            ),
            "utf-8",
        ),
    ).abi;

    const protocolFeesBurnerAbi = JSON.parse(
        readFileSync(
            path.resolve(
                projectRoot,
                "protocol-contracts/feesBurner/artifacts/contracts/ProtocolFeesBurner.sol/ProtocolFeesBurner.json",
            ),
            "utf-8",
        ),
    ).abi;

    const feesSenderToBurnerAbi = JSON.parse(
        readFileSync(
            path.resolve(
                projectRoot,
                "protocol-contracts/feesBurner/artifacts/contracts/FeesSenderToBurner.sol/FeesSenderToBurner.json",
            ),
            "utf-8",
        ),
    ).abi;

    const pauserSetHostAbi = JSON.parse(
        readFileSync(
            path.resolve(
                projectRoot,
                "host-contracts/artifacts/contracts/immutable/PauserSet.sol/PauserSet.json",
            ),
            "utf-8",
        ),
    ).abi;

    const pauserSetWrapperAbi = JSON.parse(
        readFileSync(
            path.resolve(
                projectRoot,
                "protocol-contracts/pauserSetWrapper/artifacts/contracts/PauserSetWrapper.sol/PauserSetWrapper.json",
            ),
            "utf-8",
        ),
    ).abi;

    const gatewayConfigAbi = JSON.parse(
        readFileSync(
            path.resolve(
                projectRoot,
                "gateway-contracts/artifacts/contracts/GatewayConfig.sol/GatewayConfig.json",
            ),
            "utf-8",
        ),
    ).abi;

    return {
        ACLAbi,
        governanceOAppSenderAbi,
        governanceOAppReceiverAbi,
        safeL2Abi,
        adminModuleAbi,
        zamaERC20Abi,
        zamaOFTAdapterAbi,
        zamaOFTAbi,
        protocolFeesBurnerAbi,
        feesSenderToBurnerAbi,
        pauserSetHostAbi,
        pauserSetWrapperAbi,
        gatewayConfigAbi,
    };
}
