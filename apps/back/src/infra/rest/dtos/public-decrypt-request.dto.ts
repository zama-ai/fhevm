import { z } from 'zod'

export const publicDecryptSchema = z.object({
  ciphertextHandles: z.array(z.string().startsWith('0x').length(66)).min(1),
})

export type PublicDecryptRequest = z.infer<typeof publicDecryptSchema>
