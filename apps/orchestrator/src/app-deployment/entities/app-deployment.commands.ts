/* eslint-disable @typescript-eslint/no-empty-object-type */
export type AppDeploymentCommands = DiscoverSM | ConfirmSM | RegisterSM;

type Command<Key extends string, Payload extends object> = {
  _tag: 'Command';
  type: `app-deployment.${Key}`;
  payload: Payload & { applicationId: string };
};

type DiscoverSM = Command<'discover-sm', { address: string; chainId: string }>;
type ConfirmSM = Command<'confirm-sm', {}>;
type RegisterSM = Command<'register-sm', {}>;

/**
 * Create a factory to generate a given command
 *
 * @param type The type of the Command to generate
 * @returns the factory function for the selected command
 */
function factory<K extends AppDeploymentCommands['type']>(type: K) {
  return function <Command extends Extract<AppDeploymentCommands, { type: K }>>(
    payload: Command['payload'],
  ) {
    // TODO: find a better way to solve this
    return { _tag: 'Command', type, payload } as Command;
  };
}

export const discoverSM = factory('app-deployment.discover-sm');
export const confirmSM = factory('app-deployment.confirm-sm');
export const registerSM = factory('app-deployment.register-sm');
