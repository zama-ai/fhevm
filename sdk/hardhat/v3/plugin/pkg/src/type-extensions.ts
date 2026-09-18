// Type-level face of the runtime attachments — no runtime code in this file, per the hardhat 3
// type-extension rules; the matching hook handlers are what make these properties real.
//
// The augmentation must repeat hardhat's own type-parameter list VERBATIM (`ChainTypeT extends
// ChainType | string`), or TypeScript refuses to merge the interfaces — the two rules below cannot
// see that requirement.
/* eslint-disable @typescript-eslint/no-redundant-type-constituents, @typescript-eslint/no-unused-vars */

import type { HardhatFhevmRuntimeEnvironment } from './types.js';

declare module 'hardhat/types/network' {
  interface NetworkConnection<ChainTypeT extends ChainType | string> {
    fhevm: HardhatFhevmRuntimeEnvironment;
  }
}
