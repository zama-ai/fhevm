import { z } from 'zod'

export const RegisterFormSchema = z
  .object({
    name: z
      .string()
      .min(2, 'Name must be at least 2 characters long')
      .max(128, 'Name should be at most 128 characters long'),
    password: z
      .string()
      .min(8, 'Password should be at least 8 characters long')
      .max(64, 'Password should be at most 64 characters long'),
    repeatPassword: z.string(),
    invitationKey: z.string(),
    agree: z.boolean(),
  })
  .refine(data => data.password === data.repeatPassword, {
    message: "Passwords don't match",
    path: ['repeatPassword'],
  })
  .refine(data => data.agree, {
    message: 'You must agree to the terms of service',
    path: ['agree'],
  })

export type RegisterFormSchemaType = z.infer<typeof RegisterFormSchema>

export const getPasswordStrengthScore = (password: string): number => {
  let score = 0
  if (password.length >= 8) score++
  if (/[a-z]/.test(password) && /[A-Z]/.test(password)) score++
  if (/\d/.test(password)) score++
  if (/[@$!%*?&]/.test(password)) score++
  return Math.min(score, 4)
}
