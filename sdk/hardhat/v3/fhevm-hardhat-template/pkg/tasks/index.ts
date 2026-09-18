import { emptyTask, task } from 'hardhat/config';
import { definePlugin } from 'hardhat/plugins';
import { ArgumentType } from 'hardhat/types/arguments';
import type { HardhatPlugin } from 'hardhat/types/plugins';

const templateTasks: HardhatPlugin = definePlugin({
  id: 'fhevm-hardhat-template',
  tasks: [
    task('accounts', 'Prints the accounts exposed by the selected network')
      .setAction(() => import('./actions/accounts.js'))
      .build(),
    emptyTask(['counter'], 'Interact with a deployed FHECounter contract').build(),
    task(['counter', 'decrypt-count'], 'Decrypts the current counter value')
      .addPositionalArgument({ name: 'address', description: 'The deployed FHECounter address' })
      .setAction(() => import('./actions/decrypt-count.js'))
      .build(),
    task(['counter', 'increment'], 'Increments the encrypted counter')
      .addPositionalArgument({ name: 'address', description: 'The deployed FHECounter address' })
      .addPositionalArgument({
        name: 'value',
        description: 'The non-negative integer increment',
        type: ArgumentType.INT,
      })
      .setAction(() => import('./actions/increment.js'))
      .build(),
    task(['counter', 'decrement'], 'Decrements the encrypted counter')
      .addPositionalArgument({ name: 'address', description: 'The deployed FHECounter address' })
      .addPositionalArgument({
        name: 'value',
        description: 'The non-negative integer decrement',
        type: ArgumentType.INT,
      })
      .setAction(() => import('./actions/decrement.js'))
      .build(),
  ],
});

export default templateTasks;
