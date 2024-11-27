/* eslint-disable @typescript-eslint/no-empty-object-type */
export type AppDeploymentEvents =
  | DeploymentRequested
  | DeploymentCompleted
  | SmartContractDiscovered
  | SmartContractDiscoveryFailed
  | SmartContractConfirmed
  | SmartContractConfirmationFailed
  | SmartContractRegistered
  | SmartContractRegistrationFailed;

type Event<Key extends string, Payload extends object> = {
  _tag: 'Event';
  type: `app-deployment.${Key}`;
  payload: Payload & { applicationId: string };
};

type DeploymentRequested = Event<
  'requested',
  { address: string; chainId: string }
>;

type SmartContractDiscovered = Event<'sc-discovered', {}>;
type SmartContractDiscoveryFailed = Event<'sc-discovery-failed', {}>;
type SmartContractConfirmed = Event<'sc-confirmed', {}>;
type SmartContractConfirmationFailed = Event<'sc-confirmation-failed', {}>;
type SmartContractRegistered = Event<'sc-registered', {}>;
type SmartContractRegistrationFailed = Event<'sc-registration-failed', {}>;
type DeploymentCompleted = Event<'completed', {}>;

// type KeyWithPrfix

/**
 * Create a factory to generate a given event
 *
 * @param type The type of the Event to generate
 * @returns the factory function for the selected event
 */
function factory<K extends AppDeploymentEvents['type']>(type: K) {
  return function <Event extends Extract<AppDeploymentEvents, { type: K }>>(
    payload: Event['payload'],
  ) {
    // TODO: find a better way to solve this
    return { _tag: 'Event', type, payload } as Event;
  };
}

export const requested = factory('app-deployment.requested');
export const completed = factory('app-deployment.completed');
export const scDiscovered = factory('app-deployment.sc-discovered');
export const scDiscoveryFailed = factory('app-deployment.sc-discovery-failed');
export const scConfirmed = factory('app-deployment.sc-confirmed');
export const scConfirmationFailed = factory(
  'app-deployment.sc-confirmation-failed',
);
export const scRegistered = factory('app-deployment.sc-registered');
export const scRegistrationFailed = factory(
  'app-deployment.sc-registration-failed',
);
