import { z } from 'zod'

export const meta = z.record(z.string(), z.union([z.string(), z.number()])).and(
  z.object({
    correlationId: z.string().uuid(),
  }),
)

export type Meta = z.infer<typeof meta>

export const chainId = z.string().refine(
  v => {
    const n = Number(v)
    return !isNaN(n) && n > 0
  },
  { message: 'Invalid Chain Id' },
)

export const web3Address = z
  .string()
  .length(42, 'blockchain address must be exactly 42 charaxters long')
  .startsWith('0x', 'sepolia address must start with 0x')
