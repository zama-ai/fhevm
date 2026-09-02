import {
  type RegenerateTestConsumerPackageLocksOptions,
  regenerateTestConsumerPackageLocks,
} from '../base/test-consumer.ts';

export function testConsumerRegeneratePackageLock(options: RegenerateTestConsumerPackageLocksOptions): void {
  regenerateTestConsumerPackageLocks(options);
}
