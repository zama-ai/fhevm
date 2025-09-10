import { z } from 'zod'

const CustomAttributeSchema = z.object({
  Identifier: z.string(),
  Value: z.string(),
})

const UserRegisteredSchema = z.object({
  Event: z.literal('UserRegistered'),
  Message: z.object({
    ID: z.number().int().positive(),
    Email: z.string().email(),
    First: z.string(),
    Last: z.string(),
    OrgID: z.number().int().positive(),
    Provider: z.string(),
    CreatedAt: z.string().datetime(),
    CustomAttributes: z.array(CustomAttributeSchema),
  }),
})

const ApplicationRegistered = z.object({
  Event: z.literal('ApplicationRegistered'),
  Message: z.object({
    ID: z.number().int().positive(),
    Name: z.string(),
    UserID: z.number().int().positive(),
    CreatedAt: z.string().datetime(),
  }),
})

export const WebhookPayloadSchema = z.discriminatedUnion('Event', [
  UserRegisteredSchema,
  ApplicationRegistered,
])

export type WebhookPayload = z.infer<typeof WebhookPayloadSchema>

export type UserRegistered = Extract<
  WebhookPayload,
  { Event: 'UserRegistered' }
>['Message']
export type ApplicationRegistered = Extract<
  WebhookPayload,
  { Event: 'ApplicationRegistered' }
>['Message']
