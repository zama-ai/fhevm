import { web3Address } from 'messages'
import { z } from 'zod'

const ctHandleContractPairs = z.object({
  ctHandle: z.string(),
  contractAddress: z.string(),
})

export const requestValidity = z.object({
  startTimestamp: z.string(),
  durationDays: z.string(),
})

export const schema = z.object({
  contractsChainId: z
    .string()
    .regex(/^0x[\da-f]+$/i, 'Invalid Chain ID')
    .refine(val => parseInt(val, 16) > 0, 'Invalid Chain ID'),
  ctHandleContractPairs: z.array(ctHandleContractPairs),
  requestValidity: requestValidity,
  contractsAddresses: z.array(web3Address),
  userAddress: web3Address,
  signature: z.string(),
  publicKey: z.string(),
})

export type PrivateDecryptRequest = z.infer<typeof schema>
