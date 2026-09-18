import { fetchMetadataFromSeeds, updateV1 } from '@metaplex-foundation/mpl-token-metadata'
import { publicKey } from '@metaplex-foundation/umi'
import { SystemProgram } from '@solana/web3.js'
import bs58 from 'bs58'
import { task } from 'hardhat/config'

import { types as devtoolsTypes } from '@layerzerolabs/devtools-evm-hardhat'
import { promptToContinue } from '@layerzerolabs/io-devtools'
import { EndpointId } from '@layerzerolabs/lz-definitions'

import { deriveConnection, getExplorerTxLink } from '.'

interface Args {
    mint: string
    newUpdateAuthority?: string
    renounceUpdateAuthority?: boolean
    eid: EndpointId
}

// sets the update authority via Metaplex
// Example (set new update authority):
// pnpm hardhat lz:oft:solana:set-update-authority --eid <EID> --mint <MINT_ADDRESS> --new-update-authority <NEW_UPDATE_AUTHORITY>
// Example (renounce update authority):
// pnpm hardhat lz:oft:solana:set-update-authority --eid <EID> --mint <MINT_ADDRESS> --renounce-update-authority true
task('lz:oft:solana:set-update-authority', 'Updates the metaplex update authority of the SPL Token')
    .addParam('eid', 'Solana mainnet (30168) or testnet (40168)', undefined, devtoolsTypes.eid)
    .addParam('mint', 'The Token mint public key', undefined, devtoolsTypes.string)
    .addOptionalParam('newUpdateAuthority', 'The new update authority', undefined, devtoolsTypes.string)
    .addOptionalParam('renounceUpdateAuthority', 'Renounce update authority', false, devtoolsTypes.boolean)
    .setAction(
        async ({ eid, mint: mintStr, newUpdateAuthority: newUpdateAuthorityStr, renounceUpdateAuthority }: Args) => {
            // if not renouncing, must provide new update authority
            if (!renounceUpdateAuthority && !newUpdateAuthorityStr) {
                throw new Error(
                    'Either specify the new update authority via --new-update-authority <NEW_UPDATE_AUTHORITY> or renounce via --renounce-update-authority true'
                )
            }

            // if renouncing, must not provide new update authority
            if (renounceUpdateAuthority && newUpdateAuthorityStr) {
                throw new Error('Cannot provide new update authority if renouncing')
            }

            /*
             * On why the update authority is set to SystemProgram.programId ("11111111111111111111111111111111") when renouncing:
             * The Metaplex Token Metadata program defines the update_authority strictly as a Pubkey:
             * https://github.com/metaplex-foundation/mpl-token-metadata/blob/23aee718e723578ee5df411f045184e0ac9a9e63/programs/token-metadata/program/src/state/metadata.rs#L73
             * Hence, the value must always be a Pubkey
             * To renounce the update authority, we can to set its value to SystemProgram ID ("11111111111111111111111111111111")
             * This is done on top of setting `isMutable` to false
             */

            const updateAuthority = renounceUpdateAuthority
                ? publicKey(SystemProgram.programId)
                : // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
                  publicKey(newUpdateAuthorityStr!) // we already checked that this is defined

            const { umi, umiWalletSigner } = await deriveConnection(eid)

            const mint = publicKey(mintStr)
            const initialMetadata = await fetchMetadataFromSeeds(umi, { mint })

            if (initialMetadata.updateAuthority === SystemProgram.programId.toString()) {
                console.log('\nThe update authority has already been renounced\n')
                return
            }

            if (initialMetadata.updateAuthority !== umiWalletSigner.publicKey.toString()) {
                throw new Error('Only the update authority can update the metadata')
            }

            console.log(`\nMint Address: ${mintStr}\n`)
            console.log(`\nCurrent update authority: ${initialMetadata.updateAuthority}\n`)
            console.log(`\nNew update authority: ${updateAuthority.toString()}\n`)

            if (renounceUpdateAuthority) {
                const doContinue = await promptToContinue(
                    'You have chosen `--renounce-update-authority true`. This means that the Update Authority will be immediately renounced. This is irreversible.  Continue?'
                )
                if (!doContinue) {
                    return
                }
            }

            const isMutable = renounceUpdateAuthority ? false : initialMetadata.isMutable

            // Verify that isMutable is true when not renouncing, can't be too safe.
            if (!renounceUpdateAuthority && !isMutable) {
                throw new Error('When not renouncing, `isMutable` must be true')
            }

            const txn = await updateV1(umi, {
                mint,
                newUpdateAuthority: updateAuthority,
                authority: umiWalletSigner,
                isMutable: renounceUpdateAuthority ? false : isMutable,
            }).sendAndConfirm(umi)

            const isTestnet = eid == EndpointId.SOLANA_V2_TESTNET

            console.log(`Txn link: ${getExplorerTxLink(bs58.encode(txn.signature), isTestnet)}`)
        }
    )
