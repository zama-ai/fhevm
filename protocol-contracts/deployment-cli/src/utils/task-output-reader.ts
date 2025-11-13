import fs from "node:fs";
import path from "node:path";
import dotenv from "dotenv";
import { ethers } from "ethers";
import { ValidationError } from "./errors.js";

interface DeployedContractArtifact {
    address: string;
    [key: string]: unknown;
}

/**
 * Utility for reading deployment artifacts and environment files.
 * Provides standardized address discovery from Hardhat deployments and .env files.
 * Expects the project root (repository root) as the base directory.
 */
export class TaskOutputReader {
    private readonly projectRoot: string;

    /**
     * @param projectRoot - The repository root directory (contains gateway-contracts and host-contracts)
     */
    constructor(projectRoot: string) {
        this.projectRoot = projectRoot;
    }

    /**
     * Read a contract address from a Hardhat deployment artifact.
     * @param pkgPath - Package path relative to project root (e.g., 'protocol-contracts/token')
     * @param network - Network name as it appears in the deployments directory
     * @param contractName - Contract name (without .json extension)
     * @returns The contract address in lowercase
     * @throws ValidationError if artifact not found or contains no address
     *
     * @example
     * const addr = reader.readHardhatDeployment('protocol-contracts/token', 'ethereum-testnet', 'ZamaERC20');
     */
    public readHardhatDeployment(
        pkgPath: string,
        network: string,
        contractName: string,
    ): `0x${string}` {
        const artifactPath = path.join(
            this.projectRoot,
            pkgPath,
            "deployments",
            network,
            `${contractName}.json`,
        );

        try {
            const raw = fs.readFileSync(artifactPath, "utf8");
            const data = JSON.parse(raw) as DeployedContractArtifact;

            if (!data.address) {
                throw new ValidationError(
                    `No address field found in ${artifactPath}`,
                );
            }

            return ethers.getAddress(data.address) as `0x${string}`;
        } catch (error) {
            if (error instanceof ValidationError) {
                throw error;
            }
            if (error instanceof SyntaxError) {
                throw new ValidationError(
                    `Invalid JSON in ${artifactPath}: ${error.message}`,
                );
            }
            throw new ValidationError(
                `Failed to read deployment artifact for ${contractName} on ${network} at ${artifactPath}: ${error}`,
            );
        }
    }

    /**
     * Read addresses from a .env file.
     * @param envPath - Absolute path to the .env file
     * @param fieldMapping - Optional map of env variable names to address keys
     *                      If provided, only listed fields are returned with mapped names.
     *                      If omitted, all fields are returned as-is.
     * @returns Object with normalized address keys and values (addresses lowercased)
     * @throws ValidationError if file not found or parsing fails
     *
     * @example
     * const addrs = reader.readEnvFile('/path/.env.host', {
     *   'ACL_CONTRACT_ADDRESS': 'ACL_ADDRESS',
     *   'FHEVM_EXECUTOR_CONTRACT_ADDRESS': 'FHEVM_EXECUTOR_ADDRESS'
     * });
     */
    public readEnvFile(
        envPath: string,
        fieldMapping?: Record<string, string>,
    ): Record<string, `0x${string}`> {
        if (!fs.existsSync(envPath)) {
            throw new ValidationError(`Environment file not found: ${envPath}`);
        }

        try {
            const raw = fs.readFileSync(envPath, "utf8");
            const parsed = dotenv.parse(raw);

            const result: Record<string, `0x${string}`> = {};

            if (fieldMapping) {
                // Use the provided mapping
                for (const [envKey, normalizedKey] of Object.entries(
                    fieldMapping,
                )) {
                    const value = parsed[envKey];
                    if (value) {
                        result[normalizedKey] =
                            value.toLowerCase() as `0x${string}`;
                    }
                }
            } else {
                // Return all fields as-is
                for (const [key, value] of Object.entries(parsed)) {
                    result[key] =
                        typeof value === "string"
                            ? (value.toLowerCase() as `0x${string}`)
                            : value;
                }
            }

            return result;
        } catch (error) {
            if (error instanceof ValidationError) {
                throw error;
            }
            throw new ValidationError(
                `Failed to read environment file ${envPath}: ${error}`,
            );
        }
    }

    /**
     * Resolve a Hardhat artifact path relative to this reader's project root.
     * Useful for debugging or logging the actual paths being used.
     *
     * @example
     * const artifactPath = reader.resolveArtifactPath(
     *   'protocol-contracts/token',
     *   'ethereum-testnet',
     *   'ZamaERC20'
     * );
     * console.log(`Reading from ${artifactPath}`);
     */
    public resolveArtifactPath(
        pkgPath: string,
        network: string,
        contractName: string,
    ): string {
        return path.join(
            this.projectRoot,
            pkgPath,
            "deployments",
            network,
            `${contractName}.json`,
        );
    }
}
