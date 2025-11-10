import type { Options } from "@layerzerolabs/lz-v2-utilities";
import {
    type Abi,
    type Address,
    createWalletClient,
    type GetContractReturnType,
    getContract,
    http,
    parseEther,
    type TransactionReceipt,
} from "viem";
import type { ExtendedTestClient } from "../../tests/types.js";
import {
    deliverToReceiver,
    encodeGovernanceMessage,
} from "../../tests/utils/helpers.js";

export interface ProposalData {
    targets: string[];
    values: number[] | bigint[];
    functionSignatures: string[];
    datas: string[];
    operations: number[];
}

/**
 * Send a proposal from L1 (GovernanceOAppSender)
 */
export async function sendProposal(
    client: ExtendedTestClient,
    senderContract: GetContractReturnType<Abi, ExtendedTestClient, Address>,
    proposal: ProposalData,
    options: Options,
    daoAddress: Address,
): Promise<void> {
    const optionsHex = options.toHex().toString();

    // Quote the fee
    const quotedFee = (await senderContract.read.quoteSendCrossChainTransaction(
        [
            proposal.targets,
            proposal.values,
            proposal.functionSignatures,
            proposal.datas,
            proposal.operations,
            optionsHex,
        ],
    )) as bigint;

    // Impersonate DAO to send proposal
    await client.impersonateAccount({ address: daoAddress });
    await client.setBalance({
        address: daoAddress,
        value: parseEther("10"),
    });

    const daoSigner = createWalletClient({
        account: daoAddress,
        chain: client.chain,
        transport: http(client.chain?.rpcUrls.default.http[0]),
    });

    const senderAsDAO = getContract({
        address: senderContract.address,
        abi: senderContract.abi,
        client: daoSigner,
    });

    const hash = await senderAsDAO.write.sendRemoteProposal(
        [
            proposal.targets,
            proposal.values,
            proposal.functionSignatures,
            proposal.datas,
            proposal.operations,
            optionsHex,
        ],
        { value: quotedFee },
    );

    await client.waitForTransactionReceipt({ hash });
    await client.stopImpersonatingAccount({ address: daoAddress });
}

/**
 * Execute a proposal on L2 (GovernanceOAppReceiver) by simulating LayerZero delivery
 * Returns the transaction receipt to check status and gas used
 */
export async function executeProposal(
    client: ExtendedTestClient,
    receiverContract: GetContractReturnType<Abi, ExtendedTestClient, Address>,
    proposal: ProposalData,
    options: Options,
    originEid: number,
    senderAddress: string,
): Promise<TransactionReceipt> {
    const message = encodeGovernanceMessage(
        proposal.targets,
        proposal.values,
        proposal.functionSignatures,
        proposal.datas,
        proposal.operations,
    );

    const receipt = await deliverToReceiver(
        client,
        receiverContract.address,
        receiverContract.abi,
        originEid,
        senderAddress,
        message,
        options,
    );
    return receipt;
}
