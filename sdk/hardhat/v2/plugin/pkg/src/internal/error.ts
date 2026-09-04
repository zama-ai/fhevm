import { HardhatFhevmError } from '../error';

export function assertHHFhevm(cond: boolean, message?: string): asserts cond {
  if (!cond) {
    throw new HardhatFhevmError(message ?? 'Fhevm assertion failed.');
  }
}
