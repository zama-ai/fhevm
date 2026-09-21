import {
    UpdateV1InstructionAccounts,
    UpdateV1InstructionArgs,
    fetchMetadataFromSeeds,
    updateV1,
} from '@metaplex-foundation/mpl-token-metadata'
import { publicKey, transactionBuilder } from '@metaplex-foundation/umi'
import { toWeb3JsTransaction } from '@metaplex-foundation/umi-web3js-adapters'
import bs58 from 'bs58'
import { task } from 'hardhat/config'

import { types as devtoolsTypes } from '@layerzerolabs/devtools-evm-hardhat'
import { EndpointId } from '@layerzerolabs/lz-definitions'

import { deriveConnection, getExplorerTxLink } from './index'

interface UpdateMetadataTaskArgs {
    eid: EndpointId
    name: string
    mint: string
    sellerFeeBasisPoints: number
    symbol: string
    uri: string
    vaultPda: string
}

// note that if URI is specified, then the name and symbol in there would be used and will override the 'outer' name and symbol
// Example:
// pnpm hardhat lz:oft:solana:update-metadata --eid <EID> --mint <MINT_ADDRESS> --name <NEW_TOKEN_NAME>
// If Update Authority is a multisig (Vault PDA):
// // pnpm hardhat lz:oft:solana:update-metadata --eid <EID> --mint <MINT_ADDRESS> --name <NEW_TOKEN_NAME> --vault-pda <VAULT_ADDRESS>
task('lz:oft:solana:update-metadata', 'Updates the metaplex metadata of the SPL Token')
    .addParam('eid', 'Solana mainnet (30168) or testnet (40168)', undefined, devtoolsTypes.eid)
    .addParam('mint', 'The Token mint public key', undefined, devtoolsTypes.string)
    .addOptionalParam('name', 'Token Name', undefined, devtoolsTypes.string)
    .addOptionalParam('symbol', 'Token Symbol', undefined, devtoolsTypes.string)
    .addOptionalParam('sellerFeeBasisPoints', 'Seller fee basis points', undefined, devtoolsTypes.int)
    .addOptionalParam('uri', 'URI for token metadata', undefined, devtoolsTypes.string)
    .addOptionalParam('vaultPda', 'The Vault PDA public key', undefined, devtoolsTypes.string)
    .setAction(
        async ({ eid, name, mint: mintStr, sellerFeeBasisPoints, symbol, uri, vaultPda }: UpdateMetadataTaskArgs) => {
            const { umi, umiWalletSigner } = await deriveConnection(eid, {
                noopSigner: vaultPda ? publicKey(vaultPda) : undefined,
            })

            const mint = publicKey(mintStr)

            const initialMetadata = await fetchMetadataFromSeeds(umi, { mint })

            if (!vaultPda && initialMetadata.updateAuthority !== umiWalletSigner.publicKey.toString()) {
                throw new Error('Only the update authority can update the metadata')
            }

            if (vaultPda && initialMetadata.updateAuthority !== publicKey(vaultPda).toString()) {
                throw new Error('Provided vaultPda is not the current update authority on this metadata')
            }

            if (initialMetadata.isMutable == false) {
                throw new Error('Metadata is not mutable')
            }

            const isTestnet = eid == EndpointId.SOLANA_V2_TESTNET

            const updateV1Args: UpdateV1InstructionAccounts & UpdateV1InstructionArgs = {
                mint,
                authority: umiWalletSigner,
                data: {
                    ...initialMetadata,
                    name: name || initialMetadata.name,
                    symbol: symbol || initialMetadata.symbol,
                    uri: uri || initialMetadata.uri,
                    sellerFeeBasisPoints:
                        sellerFeeBasisPoints != undefined ? sellerFeeBasisPoints : initialMetadata.sellerFeeBasisPoints,
                },
            }
            const updateIxn = updateV1(umi, updateV1Args)
            const txBuilder = transactionBuilder().add(updateIxn)
            if (vaultPda) {
                txBuilder.setFeePayer(umiWalletSigner).useV0()
                // Include a recent blockhash before building
                const web3JsTxn = toWeb3JsTransaction(await txBuilder.buildWithLatestBlockhash(umi))
                const base58 = bs58.encode(new Uint8Array(web3JsTxn.message.serialize()))
                console.log('==== Import the following base58 txn data into the Squads UI ====')
                console.log(base58)
                // output txn data as base58
            } else {
                // submit the txn
                const createTokenTx = await txBuilder.sendAndConfirm(umi)
                console.log(`createTokenTx: ${getExplorerTxLink(bs58.encode(createTokenTx.signature), isTestnet)}`)
            }
        }
    )
