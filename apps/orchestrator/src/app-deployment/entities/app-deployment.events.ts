/* eslint-disable @typescript-eslint/no-empty-object-type */
import type { ExhaustiveTuple } from '../../utils';

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

type EventTypes = keyof EventMap;

export type AppDeploymentEvent = {
  [Key in EventTypes]: {
    _tag: 'Event';
    type: `app-deployment.${Key}`;
    payload: EventMap[Key] & { applicationId: string };
  };
}[EventTypes];

const _eventTypes = [
  'requested',
  'sc-confirmation-failed',
  'sc-confirmed',
  'sc-discovered',
  'sc-discovery-failed',
  'sc-registered',
  'sc-registration-failed',
  'completed',
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
  // Check _tag
  if (!('_tag' in data) || data._tag !== 'Event') {
    return false;
  }

  // check type
  if (
    !('type' in data) ||
    typeof data.type !== 'string' ||
    !data.type.startsWith('app-deployment.') ||
    !(eventTypes as readonly string[]).includes(data.type.split('.')[1])
  ) {
    return false;
  }

  // checking payload
  if (
    !('payload' in data) ||
    typeof data.payload !== 'object' ||
    data.payload === null ||
    !('applicationId' in data.payload) ||
    typeof data.payload.applicationId !== 'string'
  ) {
    return false;
  }

  return true;
}
