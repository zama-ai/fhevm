import { z } from 'zod'

export const CreateApiKeySchema = z.object({
  name: z
    .string()
    .min(2, 'Name must be at least 2 characters long')
    .max(50, 'Name should be at most 50 characters long'),
  description: z
    .string()
    .max(200, 'Description should be at most 200 characters long'),
})

export type CreateApiKeySchemaType = z.infer<typeof CreateApiKeySchema>
