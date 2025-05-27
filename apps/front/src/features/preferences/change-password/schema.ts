import { z } from 'zod'

export const ChangePasswordSchema = z
  .object({
    oldPassword: z
      .string()
      .min(1, { message: 'Old password is required' })
      .optional(),
    newPassword: z
      .string()
      .min(8, { message: 'Password should be at least 8 characters long' })
      .regex(/[a-z]/, {
        message: 'Password should contain at least a lowercase character',
      })
      .regex(/[A-Z]/, {
        message: 'Password should contain at least an uppercase character',
      })
      .regex(/[[ !"#$%&'()*+,-./:;<=>?@[\\\]^_`{|}~]/, {
        message: 'Password should contain at least a special character',
      }),
    repeatPassword: z.string(),
  })
  .refine(data => data.newPassword === data.repeatPassword, {
    message: 'Password does not match',
    path: ['repeatPassword'],
  })

export type ChangePasswordValues = z.infer<typeof ChangePasswordSchema>
