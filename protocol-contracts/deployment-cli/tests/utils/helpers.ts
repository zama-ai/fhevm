import { Options } from "@layerzerolabs/lz-v2-utilities";
import type { Abi, Address, TransactionReceipt } from "viem";
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
): Promise<TransactionReceipt> {
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
        const receipt = await client.waitForTransactionReceipt({ hash });
        if (receipt.status === "reverted") {
            throw new Error(`Transaction reverted: ${receipt.transactionHash}`);
        }
        return receipt;
    } finally {
        await client.stopImpersonatingAccount({ address: endpointAddress });
    }
}

export function buildLzOptions(
    gas: number = 200_000,
    value: bigint = 0n,
): { hex: `0x${string}`; obj: Options } {
    const hex = Options.newOptions()
        .addExecutorLzReceiveOption(gas, Number(value))
        .toHex()
        .toString() as `0x${string}`;
    return { hex, obj: Options.fromOptions(hex) };
}

// Send tokens from an OFT Adapter (L1) to OFT (L2) and deliver the message
export async function sendOFTFromAdapterAndDeliver(params: {
    srcClient: ExtendedTestClient;
    dstClient: ExtendedTestClient;
    srcEid: number;
    dstEid: number;
    sender: Address;
    amountLD: bigint; // token amount in local decimals (LD)
    erc20Address: Address;
    erc20Abi: Abi;
    oftAdapterAddress: Address;
    oftAdapterAbi: Abi;
    oftOnDstAddress: Address;
    oftOnDstAbi: Abi;
    gas?: number;
}): Promise<void> {
    const {
        srcClient,
        dstClient,
        srcEid,
        dstEid,
        sender,
        amountLD,
        erc20Address,
        erc20Abi,
        oftAdapterAddress,
        oftAdapterAbi,
        oftOnDstAddress,
        oftOnDstAbi,
        gas = 200_000,
    } = params;

    // Build options used for both send and delivery
    const { hex: optionsHex, obj: optionsObj } = buildLzOptions(gas);

    // Impersonate sender on L1 and construct signer
    await srcClient.impersonateAccount({ address: sender });
    const l1Signer = createWalletClient({
        account: sender,
        chain: srcClient.chain,
        transport: http(srcClient.chain?.rpcUrls.default.http[0]),
    });

    // Approve adapter to pull ERC20
    const erc20AsSender = getContract({
        address: erc20Address,
        abi: erc20Abi,
        client: l1Signer,
    });
    let tx = await erc20AsSender.write.approve([oftAdapterAddress, amountLD]);
    await srcClient.waitForTransactionReceipt({ hash: tx });

    // Quote and send from adapter
    const adapterAsSender = getContract({
        address: oftAdapterAddress,
        abi: oftAdapterAbi,
        client: l1Signer,
    });
    const sendParam = [
        dstEid,
        pad(sender, { size: 32 }),
        amountLD,
        amountLD,
        optionsHex,
        "0x" as `0x${string}`,
        "0x" as `0x${string}`,
    ] as const;

    const quote = (await adapterAsSender.read.quoteSend([
        sendParam,
        false,
    ])) as { nativeFee: bigint; lzTokenFee: bigint };

    const { result } = await adapterAsSender.simulate.send(
        [sendParam, [quote.nativeFee, quote.lzTokenFee], sender],
        { value: quote.nativeFee },
    );
    tx = await adapterAsSender.write.send(
        [sendParam, [quote.nativeFee, quote.lzTokenFee], sender],
        { value: quote.nativeFee },
    );
    await srcClient.waitForTransactionReceipt({ hash: tx });
    await srcClient.stopImpersonatingAccount({ address: sender });

    // Compute amount in shared decimals (amountSD) for message payload
    const oftDst = getContract({
        address: oftOnDstAddress,
        abi: oftOnDstAbi,
        client: dstClient,
    });
    const decimalConversionRate =
        (await oftDst.read.decimalConversionRate()) as bigint;
    const amountReceivedLD = (result[1] as { amountReceivedLD: bigint })
        .amountReceivedLD;
    const amountSD = amountReceivedLD / decimalConversionRate;

    // Encode OFT message: recipient (bytes32) + amountSD (uint64)
    const message = (await import("viem")).encodePacked(
        ["bytes32", "uint64"],
        [pad(sender, { size: 32 }), amountSD],
    );

    // Deliver to destination OFT (simulating executor)
    await deliverToReceiver(
        dstClient,
        oftOnDstAddress,
        oftOnDstAbi,
        srcEid,
        oftAdapterAddress,
        message,
        optionsObj,
    );
}

// Send tokens from OFT (L2) back to Adapter (L1) and deliver the message
export async function sendOFTFromOFTAndDeliver(params: {
    srcClient: ExtendedTestClient;
    dstClient: ExtendedTestClient;
    srcEid: number;
    dstEid: number;
    sender: Address;
    amountLD: bigint; // token amount in local decimals (LD)
    // contracts
    oftOnSrcAddress: Address;
    oftOnSrcAbi: Abi;
    oftAdapterOnDstAddress: Address;
    oftAdapterOnDstAbi: Abi;
    gas?: number;
}): Promise<void> {
    const {
        srcClient,
        dstClient,
        srcEid,
        dstEid,
        sender,
        amountLD,
        oftOnSrcAddress,
        oftOnSrcAbi,
        oftAdapterOnDstAddress,
        oftAdapterOnDstAbi,
        gas = 200_000,
    } = params;

    const { hex: optionsHex, obj: optionsObj } = buildLzOptions(gas);

    await srcClient.impersonateAccount({ address: sender });
    const l2Signer = createWalletClient({
        account: sender,
        chain: srcClient.chain,
        transport: http(srcClient.chain?.rpcUrls.default.http[0]),
    });

    const oftAsSender = getContract({
        address: oftOnSrcAddress,
        abi: oftOnSrcAbi,
        client: l2Signer,
    });

    const sendParam = [
        dstEid,
        pad(sender, { size: 32 }),
        amountLD,
        amountLD,
        optionsHex,
        "0x" as `0x${string}`,
        "0x" as `0x${string}`,
    ] as const;

    const quote = (await oftAsSender.read.quoteSend([sendParam, false])) as {
        nativeFee: bigint;
        lzTokenFee: bigint;
    };

    const { result } = await oftAsSender.simulate.send(
        [sendParam, [quote.nativeFee, quote.lzTokenFee], sender],
        { value: quote.nativeFee },
    );
    const tx = await oftAsSender.write.send(
        [sendParam, [quote.nativeFee, quote.lzTokenFee], sender],
        { value: quote.nativeFee },
    );
    await srcClient.waitForTransactionReceipt({ hash: tx });
    await srcClient.stopImpersonatingAccount({ address: sender });

    // Compute amount in shared decimals
    const oftSrc = getContract({
        address: oftOnSrcAddress,
        abi: oftOnSrcAbi,
        client: srcClient,
    });
    const decimalConversionRate =
        (await oftSrc.read.decimalConversionRate()) as bigint;
    const amountReceivedLD = (result[1] as { amountReceivedLD: bigint })
        .amountReceivedLD;
    const amountSD = amountReceivedLD / decimalConversionRate;

    const message = (await import("viem")).encodePacked(
        ["bytes32", "uint64"],
        [pad(sender, { size: 32 }), amountSD],
    );

    await deliverToReceiver(
        dstClient,
        oftAdapterOnDstAddress,
        oftAdapterOnDstAbi,
        srcEid,
        oftOnSrcAddress,
        message,
        optionsObj,
    );
}
