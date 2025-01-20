import { z } from 'zod'

export const AccountFormSchema = z.object({
  name: z
    .string()
    .min(2, 'Name must be at least 2 characters long')
    .max(128, 'Name should be at most 128 characters long'),
})

export type AccountFormSchemaType = z.infer<typeof AccountFormSchema>

export const PasswordFormSchema = z
  .object({
    password: z
      .string()
      .min(8, 'Password should be at least 8 characters long')
      .max(64, 'Password should be at most 64 characters long'),
    repeatPassword: z.string(),
  })
  .refine(data => data.password === data.repeatPassword, {
    message: "Passwords don't match",
    path: ['repeatPassword'],
  })

export type PAsswordFormSchemaType = z.infer<typeof PasswordFormSchema>

export const getPasswordStrengthScore = (password: string): number => {
  let score = 0
  if (password.length >= 8) score++
  if (/[a-z]/.test(password) && /[A-Z]/.test(password)) score++
  if (/\d/.test(password)) score++
  if (/[@$!%*?&]/.test(password)) score++
  return Math.min(score, 4)
}
