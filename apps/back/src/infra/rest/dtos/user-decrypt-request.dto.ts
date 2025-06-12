import { web3Address } from 'messages'
import { z } from 'zod'

const ctHandleContractPairs = z.object({
  handle: z.string(),
  contractAddress: web3Address,
})

export const requestValidity = z.object({
  startTimestamp: z
    .string()
    .refine(val => parseInt(val, 10) > 0, 'Invalid Timestamp'),
  durationDays: z
    .string()
    .refine(val => parseInt(val, 10) > 0, 'Invalid Duration'),
})

export const userDecryptSchema = z.object({
  handleContractPairs: z.array(ctHandleContractPairs),
  requestValidity: requestValidity,
  contractsChainId: z.union([
    z.number().int().positive(),
    z
      .string()
      .refine(
        val => parseInt(val, 10) > 0,
        'Chain ID should be a numeric string',
      ),
    z.string().regex(/^0x[\da-f]+$/i, 'Invalid Chain ID'),
  ]),
  contractAddresses: z.array(web3Address),
  userAddress: web3Address,
  signature: z.string(),
  publicKey: z.string(),
})

export type UserDecryptRequest = z.infer<typeof userDecryptSchema>
