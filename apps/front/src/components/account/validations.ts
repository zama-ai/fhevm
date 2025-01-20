import { z } from 'zod'

export const AccountFormSchema = z.object({
  name: z
    .string()
    .min(2, 'Name must be at least 2 characters long')
    .max(128, 'Name should be at most 128 characters long'),
})

export type AccountFormSchemaType = z.infer<typeof AccountFormSchema>
