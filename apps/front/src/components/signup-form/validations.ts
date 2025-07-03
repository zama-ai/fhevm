import { z } from 'zod'

const signupProps = {
  name: z
    .string()
    .min(2, 'Name must be at least 2 characters long')
    .max(128, 'Name should be at most 128 characters long'),
  password: z
    .string()
    .min(8, 'Password should be at least 8 characters long')
    .max(64, 'Password should be at most 64 characters long'),
  repeatPassword: z.string(),
  agree: z.boolean(),
}

export const SignupFormSchema = z
  .object({ ...signupProps, email: z.string().email('Invalid email address') })
  .refine(data => data.password === data.repeatPassword, {
    message: "Passwords don't match",
    path: ['repeatPassword'],
  })
  .refine(data => data.agree, {
    message: 'You must agree to the terms of service',
    path: ['agree'],
  })

export const InvitationFormSchema = z
  .object(signupProps)
  .refine(data => data.password === data.repeatPassword, {
    message: "Passwords don't match",
    path: ['repeatPassword'],
  })
  .refine(data => data.agree, {
    message: 'You must agree to the terms of service',
    path: ['agree'],
  })

export type InvitationFormSchemaType = z.infer<typeof InvitationFormSchema>
export type SignupFormSchemaType = z.infer<typeof SignupFormSchema>

export const getPasswordStrengthScore = (password: string): number => {
  let score = 0
  if (password.length >= 8) score++
  if (/[a-z]/.test(password) && /[A-Z]/.test(password)) score++
  if (/\d/.test(password)) score++
  if (/[@$!%*?&]/.test(password)) score++
  return Math.min(score, 4)
}
