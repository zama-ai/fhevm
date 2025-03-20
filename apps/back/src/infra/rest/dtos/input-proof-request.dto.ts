import { chainId } from 'messages'
import { z } from 'zod'

export const schema = z.object({
  contractChainId: chainId,
  contractAddress: z.string().regex(/^0x[\da-f]{40}$/i, 'Invalid Address'),
  userAddress: z.string().regex(/^0x[\da-f]{40}$/i, 'Invalid Address'),
  ciphertextWithZkpok: z.string().regex(/^[\da-f]+$/i, 'Invalid Ciphertext'),
})

export type InputProofRequest = z.infer<typeof schema>
