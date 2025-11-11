import { AdminABI } from "@aragon/admin-plugin-artifacts";
import {
    type Address,
    createPublicClient,
    createWalletClient,
    defineChain,
    encodeFunctionData,
    getAddress,
    http,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { loadContractABIs } from "../../tests/utils/contract-loader.js";
import type { DeploymentContext } from "../steps/base-step.js";

export type DaoAction = {
    to: `0x${string}`;
    value: bigint;
    data: `0x${string}`;
};

export async function executeViaAdminPlugin(
    ctx: DeploymentContext,
    actions: DaoAction[],
): Promise<`0x${string}`> {
    const ethereum = ctx.networks.getEthereum();
    const rpcUrl = ethereum.rpcUrl;
    const chain = defineChain({
        id: ethereum.chainId,
        name: ethereum.name,
        rpcUrls: { default: { http: [rpcUrl] }, public: { http: [rpcUrl] } },
        nativeCurrency: { name: "Ether", symbol: "ETH", decimals: 18 },
    });

    const adminPluginAddress = process.env.DAO_ADMIN_PLUGIN;
    if (!adminPluginAddress) {
        throw new Error(
            "DAO_ADMIN_PLUGIN env var is required to execute via Admin plugin.",
        );
    }

    const pk = ctx.env.resolveWalletPrivateKey("governance_deployer");
    const account = privateKeyToAccount(pk as `0x${string}`);
    const wallet = createWalletClient({
        account,
        chain,
        transport: http(rpcUrl),
    });
    const publicClient = createPublicClient({ chain, transport: http(rpcUrl) });

    // Write executeProposal on Admin plugin
    const { writeContract, waitForTransactionReceipt } = {
        writeContract: wallet.writeContract.bind(wallet),
        waitForTransactionReceipt:
            publicClient.waitForTransactionReceipt.bind(publicClient),
    };

    ctx.logger.info(
        `Executing ${actions.length} action(s) via Admin plugin ${getAddress(adminPluginAddress)}`,
    );

    const hash = await writeContract({
        address: getAddress(adminPluginAddress),
        abi: AdminABI,
        functionName: "executeProposal",
        args: [
            "0x", // proposalId left empty for direct execution
            actions.map((a) => ({
                to: getAddress(a.to),
                value: a.value,
                data: a.data,
            })),
            0n, // allowFailureMap
        ],
    });

    await waitForTransactionReceipt({ hash });
    ctx.logger.success(`Admin plugin execution tx: ${hash}`);
    return hash;
}

export async function grantRoleViaDao(options: {
    ctx: DeploymentContext;
    tokenAddress: Address;
    grantee: Address;
    role: "MINTING_PAUSER_ROLE" | "MINTER_ROLE";
}): Promise<`0x${string}` | null> {
    const { ctx, tokenAddress, grantee, role } = options;
    const ethereum = ctx.networks.getEthereum();
    const rpcUrl = ethereum.rpcUrl;
    const chain = defineChain({
        id: ethereum.chainId,
        name: ethereum.name,
        rpcUrls: { default: { http: [rpcUrl] }, public: { http: [rpcUrl] } },
        nativeCurrency: { name: "Ether", symbol: "ETH", decimals: 18 },
    });

    const publicClient = createPublicClient({ chain, transport: http(rpcUrl) });

    const { readContract } = {
        readContract: publicClient.readContract.bind(publicClient),
    };

    const token = getAddress(tokenAddress);
    const granteeAddr = getAddress(grantee);

    // Resolve role value from contract
    const { zamaERC20Abi } = await loadContractABIs();
    const roleValue = (await readContract({
        address: token,
        abi: zamaERC20Abi,
        functionName: role,
        args: [],
    })) as `0x${string}`;

    // Short-circuit if grantee already has the role
    const alreadyHas = (await readContract({
        address: token,
        abi: zamaERC20Abi,
        functionName: "hasRole",
        args: [roleValue, granteeAddr],
    })) as boolean;

    if (alreadyHas) {
        ctx.logger.info(
            `${granteeAddr} already has ${role} on ${token}; skipping DAO action`,
        );
        return null;
    }

    const data = encodeFunctionData({
        abi: zamaERC20Abi,
        functionName: "grantRole",
        args: [roleValue, granteeAddr],
    });

    return executeViaAdminPlugin(ctx, [
        {
            to: token,
            value: 0n,
            data,
        },
    ]);
}
