import { z } from 'zod'

export const CreateApiKeySchema = z.object({
  name: z
    .string()
    .min(3, 'Name must be at least 3 characters long')
    .max(50, 'Name should be at most 50 characters long'),
  description: z
    .string()
    .max(500, 'Description should be at most 500 characters long'),
})

export type CreateApiKeySchemaType = z.infer<typeof CreateApiKeySchema>
