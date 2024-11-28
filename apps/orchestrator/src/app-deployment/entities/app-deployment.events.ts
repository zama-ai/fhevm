/* eslint-disable @typescript-eslint/no-empty-object-type */
import type { ExhaustiveTuple } from '../utils';

type EventMap = {
  requested: { address: string; chainId: string };
  'sc-discovered': {};
  'sc-discovery-failed': {};
  'sc-confirmed': {};
  'sc-confirmation-failed': {};
  'sc-registered': {};
  'sc-registration-failed': {};
  completed: {};
};

export type AppDeploymentEvent = {
  [Key in EventTypes]: {
    _tag: 'Event';
    type: `app-deployment.${Key}`;
    payload: EventMap[Key] & { applicationId: string };
  };
}[EventTypes];

type EventTypes = keyof EventMap;
const _eventTypes = [
  'completed',
  'requested',
  'sc-confirmation-failed',
  'sc-confirmed',
  'sc-discovered',
  'sc-discovery-failed',
  'sc-registered',
  'sc-registration-failed',
] as const;
const eventTypes: ExhaustiveTuple<EventTypes, typeof _eventTypes> = _eventTypes;

/**
 * Create a factory to generate a given event
 *
 * @param type The type of the Event to generate
 * @returns the factory function for the selected event
 */
function factory<K extends EventTypes>(type: K) {
  // TODO: find a better way to solve this
  return function (payload: EventMap[K] & { applicationId: string }) {
    return {
      _tag: 'Event',
      type: `app-deployment.${type}`,
      payload,
    } as AppDeploymentEvent;
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
  if (typeof data !== 'object' || data === null) {
    return false;
  }
  if (!('_tag' in data) || !('type' in data) || !('payload' in data)) {
    return false;
  }

  if (
    data._tag !== 'Event' ||
    typeof data.type !== 'string' ||
    data.type.startsWith('app-deployment.') ||
    typeof data.payload !== 'object' ||
    data.payload === null
  ) {
    return false;
  }

  return (
    (eventTypes as readonly string[]).includes(data.type.split('.')[1]) &&
    'applicationId' in data.payload &&
    data.payload.applicationId === 'string'
  );
}
