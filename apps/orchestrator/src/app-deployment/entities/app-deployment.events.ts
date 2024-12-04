import { z } from 'zod';

type EventTypes =
  | 'requested'
  | 'sc-discovered'
  | 'sc-discovery-failed'
  | 'sc-confirmed'
  | 'sc-confirmation-failed'
  | 'sc-registered'
  | 'sc-registration-failed'
  | 'completed';

function genSchema<Key extends EventTypes, Payload extends z.ZodRawShape>(
  key: Key,
  payload: Payload,
) {
  const type = `app-deployment.${key}` as `app-deployment.${Key}`;
  return z.object({
    type: z.literal(type),
    payload: z.object({
      applicationId: z.string(),
      deploymentId: z.string(),
      ...payload,
    } as {
      applicationId: z.ZodString;
      deploymentId: z.ZodString;
    } & Payload),
  });
}

const eventMap = {
  requested: genSchema('requested', {
    address: z.string(),
    chainId: z.string(),
  }),
  'sc-discovered': genSchema('sc-discovered', {}),
  'sc-discovery-failed': genSchema('sc-discovery-failed', {}),
  'sc-confirmed': genSchema('sc-confirmed', {}),
  'sc-confirmation-failed': genSchema('sc-confirmation-failed', {}),
  'sc-registered': genSchema('sc-registered', {}),
  'sc-registration-failed': genSchema('sc-registration-failed', {}),
  completed: genSchema('completed', {}),
} as const;
type EventMap = typeof eventMap;

const schema = z
  .discriminatedUnion('type', [
    eventMap['requested'],
    eventMap['sc-discovered'],
  ])
  .and(
    z.object({
      _tag: z.literal('Event'),
      $meta: z.record(z.string(), z.string()).optional(),
    }),
  );
type AppDeploymentEvent = z.infer<typeof schema>;

/**
 * Create a factory to generate a given event
 *
 * @param type The type of the Event to generate
 * @returns the factory function for the selected event
 */
function factory<K extends keyof EventMap>(type: K) {
  return function (payload: z.infer<EventMap[K]>['payload']) {
    return {
      _tag: 'Event',
      type: `app-deployment.${type}`,
      payload,
    };
  };
}

export const requested = factory('requested');
export const completed = factory('completed');
export const scDiscovered = factory('sc-discovered');
export const scDiscoveryFailed = factory('sc-discovery-failed');
export const scConfirmed = factory('sc-confirmed');
export const scConfirmationFailed = factory('sc-confirmation-failed');
export const scRegistered = factory('sc-registered');
export const scRegistrationFailed = factory('sc-registration-failed');

export function isAppDeploymentEvent(
  data: unknown,
): data is AppDeploymentEvent {
  const result = schema.safeParse(data);
  return result.success;
}
