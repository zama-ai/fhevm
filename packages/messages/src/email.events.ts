import { z } from 'zod'
import { meta, metaFactory, requestId } from './shared.js'

type EventTypes =
  | 'password-reset:requested'
  | 'password-reset:completed'
  | 'user:created'

function genSchema<Key extends EventTypes, Payload extends z.ZodRawShape>(
  key: Key,
  payload: Payload,
) {
  const type = `email:${key}` as `email:${Key}`
  return z.object({
    type: z.literal(type),
    payload: z.object({
      requestId,
      ...payload,
    }),
  })
}

const schemas = [
  genSchema('password-reset:requested', {
    email: z.string().email(),
    token: z.string(),
  }),
  genSchema('password-reset:completed', {
    email: z.string().email(),
  }),
  genSchema('user:created', {
    email: z.string().email(),
    token: z.string(),
  }),
] as const

export const schema = z.discriminatedUnion('type', [...schemas]).and(
  z
    .object({
      meta: meta.optional(),
    })
    .optional(),
)

export type EmailEvent = z.infer<typeof schema>

const factory = metaFactory<EmailEvent>('email')

export const passwordResetRequested = factory('password-reset:requested')
export const passwordResetCompleted = factory('password-reset:completed')
export const userCreated = factory('user:created')

export function isEmailEvent(data: unknown): data is EmailEvent {
  return schema.safeParse(data).success
}
