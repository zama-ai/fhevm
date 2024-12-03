/* eslint-disable @typescript-eslint/no-empty-object-type */
// export type AppDeploymentCommand = DiscoverSM | ConfirmSM | RegisterSM;

import { ExhaustiveTuple } from '../../utils';

type CommandMap = {
  'discover-sm': { address: string; chainId: string };
  'confirm-sm': {};
  'register-sm': {};
};

type CommandTypes = keyof CommandMap;

export type AppDeploymentCommand = {
  [Key in CommandTypes]: {
    _tag: 'Command';
    type: `app-deployment.${Key}`;
    payload: CommandMap[Key] & { applicationId: string };
  };
}[CommandTypes];

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
  // Check _tag
  if (!('_tag' in data) || data._tag !== 'Command') {
    return false;
  }

  // check type
  if (
    !('type' in data) ||
    typeof data.type !== 'string' ||
    !data.type.startsWith('app-deployment.') ||
    !(cmdTypes as readonly string[]).includes(data.type.split('.')[1])
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
