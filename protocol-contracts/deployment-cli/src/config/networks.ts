import { ValidationError } from "../utils/errors.js";
import type { DeploymentConfig } from "./schema.js";

export interface NetworkInfo {
    readonly name: string;
    readonly rpcUrl: string;
    readonly chainId: number;
    readonly explorerUrl?: string;
    readonly explorerApiKey?: string;
    readonly hostPkgName?: string; // Network name used by host-contracts package for Hardhat tasks
    readonly gatewayPkgName?: string; // Network name used by gateway-contracts package for Hardhat tasks
    readonly blockscoutApiUrl?: string; // Blockscout API URL for contract verification
}

interface EnvironmentNetworks {
    ethereum: {
        name: string;
        host_pkg_name: string;
        rpc_url: string;
        chain_id: number;
        explorer_url?: string;
        etherscan_api_key?: string;
    };
    gateway: {
        name: string;
        gateway_pkg_name: string;
        rpc_url: string;
        chain_id: number;
        explorer_url?: string;
        etherscan_api_key?: string;
        blockscout_api_url?: string;
    };
    layerzero_config: string;
}

export class NetworkRegistry {
    private readonly ethereum: NetworkInfo & { hostPkgName: string };
    private readonly gateway: NetworkInfo & { gatewayPkgName: string };
    private readonly layerzeroConfig: string;
    private readonly selectedEnvironment: "testnet" | "mainnet";
    private readonly allEnvironments: readonly string[];

    constructor(
        config: DeploymentConfig,
        selectedEnvironment: "testnet" | "mainnet",
    ) {
        const networksList = Object.entries(config.networks);
        this.allEnvironments = networksList.map(([key]) => key);

        const selectedNetworks = this.findEnvironment(
            config.networks,
            selectedEnvironment,
        );
        if (!selectedNetworks) {
            throw new ValidationError(
                `Network environment '${selectedEnvironment}' not found. Available environments: ${Array.from(this.allEnvironments).join(", ")}`,
            );
        }

        this.selectedEnvironment = selectedEnvironment;

        this.ethereum = {
            name: selectedNetworks.ethereum.name,
            hostPkgName: selectedNetworks.ethereum.host_pkg_name,
            rpcUrl: selectedNetworks.ethereum.rpc_url,
            chainId: selectedNetworks.ethereum.chain_id,
            explorerUrl: selectedNetworks.ethereum.explorer_url,
            explorerApiKey: selectedNetworks.ethereum.etherscan_api_key,
        };

        this.gateway = {
            name: selectedNetworks.gateway.name,
            gatewayPkgName: selectedNetworks.gateway.gateway_pkg_name,
            rpcUrl: selectedNetworks.gateway.rpc_url,
            chainId: selectedNetworks.gateway.chain_id,
            explorerUrl: selectedNetworks.gateway.explorer_url,
            explorerApiKey: selectedNetworks.gateway.etherscan_api_key,
            blockscoutApiUrl: selectedNetworks.gateway.blockscout_api_url,
        };

        this.layerzeroConfig = selectedNetworks.layerzero_config;
    }

    private findEnvironment(
        networks: Record<string, EnvironmentNetworks>,
        name: string,
    ): EnvironmentNetworks | undefined {
        for (const [key, value] of Object.entries(networks)) {
            if (key === name) {
                return value;
            }
        }
        return undefined;
    }

    public getEthereum(): NetworkInfo & { hostPkgName: string } {
        return this.ethereum;
    }

    public getGateway(): NetworkInfo & { gatewayPkgName: string } {
        return this.gateway;
    }

    public getSelectedEnvironment(): "testnet" | "mainnet" {
        return this.selectedEnvironment;
    }

    public listAvailableEnvironments(): readonly string[] {
        return this.allEnvironments;
    }

    public getLayerzeroConfig(): string {
        return this.layerzeroConfig;
    }
}
