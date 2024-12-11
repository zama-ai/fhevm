import { z } from 'zod'

export const CreatorNameFormSchema = z.object({
  name: z
    .string()
    .min(2, 'Name must be at least 2 characters long')
    .max(128, 'Name should be at most 128 characters long'),
})

export const CreatorAddressFormSchema = z.object({
  address: z
    .string()
    // 0x1b44F3514812d835EB1BDB0acB33d3fA3351Ee43
    .length(42, 'Sepolia addesses are 42 characters long and start by 0x')
    .regex(/^0x[a-z0-9]{39}/i, 'Invalid sepolia address address'),
})

export type CreatorNameFormSchemaType = z.infer<typeof CreatorNameFormSchema>
export type CreatorAddressFormSchemaType = z.infer<
  typeof CreatorAddressFormSchema
>
