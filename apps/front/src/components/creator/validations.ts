import { z } from 'zod'

export const AddressSchema = z
  .string()
  // example: 0x1b44F3514812d835EB1BDB0acB33d3fA3351Ee43
  .regex(
    /^0x[a-fA-F0-9]{40}$/,
    'Sepolia addesses are 42 characters long and start by 0x',
  )

export const CreatorFormSchema = z.object({
  name: z
    .string()
    .min(2, 'Name must be at least 2 characters long')
    .max(128, 'Name should be at most 128 characters long'),
  address: AddressSchema,
})

export type CreatorFormSchemaType = z.infer<typeof CreatorFormSchema>
