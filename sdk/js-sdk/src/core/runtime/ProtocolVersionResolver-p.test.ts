import { describe, expect, it } from 'vitest';
import { sepolia } from '../chains/definitions/sepolia.js';
import { mainnet } from '../chains/definitions/mainnet.js';
import { pubKeyCrsVersionForChain } from './ProtocolVersionResolver-p.js';

function makeChain(relayerUrl: string): typeof sepolia {
  return {
    ...sepolia,
    fhevm: {
      ...sepolia.fhevm,
      relayerUrl,
    },
  };
}

describe('ProtocolVersionResolver', () => {
  it('resolves the PubKey/CRS version served by known public relayers', () => {
    expect(pubKeyCrsVersionForChain(mainnet)).toEqual({
      version: '1.4.0-alpha.3',
      comparator: 'eq',
    });
    expect(pubKeyCrsVersionForChain(sepolia)).toEqual({
      version: '1.4.0-alpha.3',
      comparator: 'eq',
    });
    // Trailing slashes are normalized before matching relayer URLs.
    expect(pubKeyCrsVersionForChain(makeChain(`${sepolia.fhevm.relayerUrl}/`))).toEqual({
      version: '1.4.0-alpha.3',
      comparator: 'eq',
    });
  });

  it('resolves the generated PubKey/CRS version for unknown relayers', () => {
    expect(pubKeyCrsVersionForChain(makeChain('http://localhost:3000'))).toEqual({
      version: '1.6.1',
      comparator: 'eq',
    });
  });
});
