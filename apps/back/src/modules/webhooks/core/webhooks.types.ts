import { z } from 'zod'

const CustomAttributeSchema = z
  .object({
    Identifier: z.string(),
    Value: z.string(),
  })
  .transform(({ Identifier, Value }) => ({
    identifier: Identifier,
    value: Value,
  }))

const UserRegistredSchema = z.object({
  Event: z.literal('UserRegistered'),
  Message: z
    .object({
      ID: z.number().int().positive(),
      Email: z.string().email(),
      First: z.string(),
      Last: z.string(),
      OrgID: z.number().int().positive(),
      Provider: z.string(),
      CreatedAt: z.string().datetime(),
      CustomAttributes: z.array(CustomAttributeSchema),
    })
    .transform(
      ({
        ID,
        Email,
        First,
        Last,
        OrgID,
        Provider,
        CreatedAt,
        CustomAttributes,
      }) => ({
        id: ID,
        email: Email,
        first: First,
        last: Last,
        orgId: OrgID,
        provider: Provider,
        createdAt: CreatedAt,
        customAttributes: CustomAttributes,
      }),
    ),
})

const ApplicationRequested = z.object({
  Event: z.literal('ApplicationRequested'),
  Message: z
    .object({
      ID: z.number().int().positive(),
      Name: z.string(),
      UserID: z.number().int().positive(),
      CreatedAt: z.string().datetime(),
    })
    .transform(({ ID, Name, UserID, CreatedAt }) => ({
      id: ID,
      name: Name,
      userId: UserID,
      createdAt: CreatedAt,
    })),
})

export const WebhookPayloadSchema = z.discriminatedUnion('Event', [
  UserRegistredSchema,
  ApplicationRequested,
])

export type WebhookPayload = z.infer<typeof WebhookPayloadSchema>

export type UserRegistered = Extract<
  WebhookPayload,
  { Event: 'UserRegistered' }
>['Message']
export type ApplicationRequested = Extract<
  WebhookPayload,
  { Event: 'ApplicationRequested' }
>['Message']
