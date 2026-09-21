import assert from 'assert'
import fs from 'fs'
import path from 'path'

import { Keypair } from '@solana/web3.js'
import bs58 from 'bs58'
import { task } from 'hardhat/config'

import { types as devtoolsTypes } from '@layerzerolabs/devtools-evm-hardhat'

interface Base58FeesTaskArgs {
    /**
     * The path to the keypair file to be used.
     */
    keypairFile: string
}

assert(process.env.HOME != undefined, 'process.env.HOME needs to be defined')

const defaultKeypairFile = path.resolve(process.env.HOME, '.config/solana/id.json')

task('lz:solana:base-58', 'Outputs the base58 string for a keypair')
    .addParam(
        'keypairFile',
        'The path to the keypair file to be used. Defaults to ~/.config/solana/id.json',
        defaultKeypairFile,
        devtoolsTypes.string
    )
    .setAction(async ({ keypairFile }: Base58FeesTaskArgs) => {
        assert(fs.existsSync(keypairFile), `Keypair file not found: ${keypairFile}`)
        const data = fs.readFileSync(keypairFile, 'utf8')
        const keypairJson = JSON.parse(data)
        const keypair = Keypair.fromSecretKey(Uint8Array.from(keypairJson))
        const base58EncodedPrivateKey = bs58.encode(keypair.secretKey)
        console.log(`Base58 encoded private key: ${base58EncodedPrivateKey}`)
    })
