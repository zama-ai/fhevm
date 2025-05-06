import { chainId, web3Address } from 'messages'
import { z } from 'zod'

export const schema = z.object({
  contractChainId: chainId,
  contractAddress: web3Address,
  userAddress: web3Address,
  ciphertextWithInputVerification: z.string().regex(/^[\da-f]+$/i, 'Invalid Ciphertext'),
})

export type InputProofRequest = z.infer<typeof schema>
