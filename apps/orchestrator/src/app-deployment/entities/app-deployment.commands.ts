/* eslint-disable @typescript-eslint/no-empty-object-type */
// export type AppDeploymentCommand = DiscoverSM | ConfirmSM | RegisterSM;

import { ExhaustiveTuple } from '../utils';

type CommandMap = {
  'discover-sm': { address: string; chainId: string };
  'confirm-sm': {};
  'register-sm': {};
};

export type AppDeploymentCommand = {
  [Key in CommandTypes]: {
    _tag: 'Command';
    type: `app-deployment.${Key}`;
    payload: CommandMap[Key] & { applicationId: string };
  };
}[CommandTypes];

type CommandTypes = keyof CommandMap;

const _cmdTypes = ['confirm-sm', 'discover-sm', 'register-sm'] as const;
const cmdTypes: ExhaustiveTuple<CommandTypes, typeof _cmdTypes> = _cmdTypes;
/**
 * Create a factory to generate a given command
 *
 * @param type The type of the Command to generate
 * @returns the factory function for the selected command
 */
function factory<K extends CommandTypes>(type: K) {
  return function (payload: CommandMap[K] & { applicationId: string }) {
    // TODO: find a better way to solve this
    return {
      _tag: 'Command',
      type: `app-deployment.${type}`,
      payload,
    } as AppDeploymentCommand;
  };
}

export const discoverSM = factory('discover-sm');
export const confirmSM = factory('confirm-sm');
export const registerSM = factory('register-sm');

export function isAppDeploymentCommand(
  data: unknown,
): data is AppDeploymentCommand {
  if (typeof data !== 'object' || data === null) {
    return false;
  }
  if (!('_tag' in data) || !('type' in data) || !('payload' in data)) {
    return false;
  }

  if (
    data._tag !== 'Command' ||
    typeof data.type !== 'string' ||
    data.type.startsWith('app-deployment.') ||
    typeof data.payload !== 'object' ||
    data.payload === null
  ) {
    return false;
  }

  return (
    (cmdTypes as readonly string[]).includes(data.type.split('.')[1]) &&
    'applicationId' in data.payload &&
    data.payload.applicationId === 'string'
  );
}
