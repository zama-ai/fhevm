import type { HardhatFhevmRuntimeEnvironment } from './types';

declare module 'hardhat/types/runtime' {
  export interface HardhatRuntimeEnvironment {
    fhevm: HardhatFhevmRuntimeEnvironment;
  }
}
