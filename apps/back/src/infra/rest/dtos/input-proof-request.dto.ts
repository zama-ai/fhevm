import { web3Address } from 'messages'
import { z } from 'zod'

export const schema = z.object({
  contractChainId: z
    .string()
    .regex(/^0x[\da-f]+$/i, 'Invalid Chain ID')
    .refine(val => parseInt(val, 16) > 0, 'Invalid Chain ID'),
  contractAddress: web3Address,
  userAddress: web3Address,
  ciphertextWithInputVerification: z
    .string()
    .regex(/^[\da-f]+$/i, 'Invalid Ciphertext'),
})

export type InputProofRequest = z.infer<typeof schema>
