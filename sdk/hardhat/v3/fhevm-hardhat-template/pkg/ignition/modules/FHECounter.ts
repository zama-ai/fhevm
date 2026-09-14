import { buildModule } from '@nomicfoundation/hardhat-ignition/modules';

export default buildModule('FHECounterModule', (m) => {
  const fheCounter = m.contract('FHECounter');

  return { fheCounter };
});
