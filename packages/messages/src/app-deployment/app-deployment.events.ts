import { z } from 'zod'

type EventTypes =
  | 'requested'
  | 'sc-discovered'
  | 'sc-discovery-failed'
  | 'sc-confirmed'
  | 'sc-confirmation-failed'
  | 'sc-registered'
  | 'sc-registration-failed'
  | 'completed'
  | 'failed'

function genSchema<Key extends EventTypes, Payload extends z.ZodRawShape>(
  key: Key,
  payload: Payload,
) {
  const type = `app-deployment.${key}` as `app-deployment.${Key}`
  return z.object({
    type: z.literal(type),
    payload: z.object({
      applicationId: z.string(),
      deploymentId: z.string(),
      ...payload,
    }),
  })
}

const eventMap = {
  requested: genSchema('requested', {
    address: z.string(),
    chainId: z.string(),
  }),
  'sc-discovered': genSchema('sc-discovered', {
    contractAddress: z.string(),
    creatorAddress: z.string(),
  }),
  'sc-discovery-failed': genSchema('sc-discovery-failed', {}),
  'sc-confirmed': genSchema('sc-confirmed', {}),
  'sc-confirmation-failed': genSchema('sc-confirmation-failed', {}),
  'sc-registered': genSchema('sc-registered', {}),
  'sc-registration-failed': genSchema('sc-registration-failed', {}),
  completed: genSchema('completed', {}),
  failed: genSchema('failed', {}),
} as const
type EventMap = typeof eventMap

const schema = z
  .discriminatedUnion('type', [
    eventMap['requested'],
    eventMap['sc-discovered'],
    eventMap['sc-discovery-failed'],
    eventMap['sc-confirmed'],
    eventMap['sc-confirmation-failed'],
    eventMap['sc-registered'],
    eventMap['sc-registration-failed'],
    eventMap['completed'],
    eventMap['failed'],
  ])
  .and(
    z.object({
      _tag: z.literal('Event'),
      meta: z.record(z.string(), z.union([z.string(), z.number()])).optional(),
    }),
  )
export type AppDeploymentEvent = z.infer<typeof schema>

/**
 * Create a factory to generate a given event
 *
 * @param type The type of the Event to generate
 * @returns the factory function for the selected event
 */
function factory<K extends keyof EventMap>(type: K) {
  return function (
    payload: z.infer<EventMap[K]>['payload'],
    meta?: Record<string, string | number>,
  ) {
    return {
      _tag: 'Event',
      type: `app-deployment.${type}`,
      payload,
      meta,
    } as AppDeploymentEvent
  }
}

export const requested = factory('requested')
export const completed = factory('completed')
export const scDiscovered = factory('sc-discovered')
export const scDiscoveryFailed = factory('sc-discovery-failed')
export const scConfirmed = factory('sc-confirmed')
export const scConfirmationFailed = factory('sc-confirmation-failed')
export const scRegistered = factory('sc-registered')
export const scRegistrationFailed = factory('sc-registration-failed')
export const failed = factory('failed')

export function isAppDeploymentEvent(
  data: unknown,
): data is AppDeploymentEvent {
  return schema.safeParse(data).success
}
