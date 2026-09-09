import { HardhatPluginError } from 'hardhat/plugins';

import constants from './internal/constants';

export class HardhatFhevmError extends HardhatPluginError {
  constructor(message: string, parent?: Error) {
    super(constants.HARDHAT_PLUGIN_NAME, message, parent);
  }
}
