import type { Options } from "@layerzerolabs/lz-v2-utilities";
import type { Abi, Address } from "viem";
import {
    createWalletClient,
    encodeAbiParameters,
    getAddress,
    getContract,
    http,
    pad,
    parseEther,
} from "viem";
import type { ExtendedTestClient } from "../types.js";

// Helper function to encode governance messages
export function encodeGovernanceMessage(
    targets: string[],
    values: number[] | bigint[],
    functionSignatures: string[],
    datas: string[],
    operations: number[],
): `0x${string}` {
    return encodeAbiParameters(
        [
            { type: "address[]" },
            { type: "uint256[]" },
            { type: "string[]" },
            { type: "bytes[]" },
            { type: "uint8[]" },
        ],
        [
            targets as Address[],
            values.map((v) => BigInt(v)),
            functionSignatures,
            datas as `0x${string}`[],
            operations.map((o) => o),
        ],
    );
}

// Helper function to deliver messages to receiver via endpoint impersonation
// Simulates LayerZero Executor by enforcing gas limits from options
export async function deliverToReceiver(
    client: ExtendedTestClient,
    receiverAddress: Address,
    receiverAbi: Abi,
    originSrcEid: number,
    senderAddress: string,
    message: `0x${string}`,
    options: Options,
): Promise<void> {
    const receiver = getContract({
        address: receiverAddress,
        abi: receiverAbi,
        client,
    });

    const endpointAddress = (await receiver.read.endpoint()) as Address;
    const senderBytes32 = pad(getAddress(senderAddress), { size: 32 });
    const nextNonce = (await receiver.read.nextNonce([
        originSrcEid,
        senderBytes32,
    ])) as bigint;

    // Impersonate the endpoint
    await client.impersonateAccount({ address: endpointAddress });
    await client.setBalance({
        address: endpointAddress,
        value: parseEther("10"),
    });

    const endpointSigner = createWalletClient({
        account: endpointAddress,
        chain: client.chain,
        transport: http(client.chain?.rpcUrls.default.http[0]),
    });

    try {
        const receiverWithSigner = getContract({
            address: receiverAddress,
            abi: receiverAbi,
            client: endpointSigner,
        });

        const txOptions: { gas?: bigint; value?: bigint } = {
            gas: options.decodeExecutorLzReceiveOption()?.gas,
            value: options.decodeExecutorLzReceiveOption()?.value,
        };
        const mockGuid =
            "0x0000000000000000000000000000000000000000000000000000000000000000";
        const hash = await receiverWithSigner.write.lzReceive(
            [
                {
                    srcEid: originSrcEid,
                    sender: senderBytes32,
                    nonce: nextNonce,
                },
                mockGuid,
                message,
                endpointAddress,
                "0x" as `0x${string}`,
            ],
            txOptions,
        );
        await client.waitForTransactionReceipt({ hash });
    } finally {
        await client.stopImpersonatingAccount({ address: endpointAddress });
    }
}
