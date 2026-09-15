import { describe, expect, it } from 'vitest';
import { sepolia } from '../chains/definitions/sepolia.js';
import { mainnet } from '../chains/definitions/mainnet.js';
import { protocolContextForChain } from './ProtocolVersionResolver-p.js';

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
  it('always resolves the fixed protocol version', () => {
    for (const chain of [mainnet, sepolia, makeChain('http://localhost:3000')]) {
      expect(protocolContextForChain(chain).protocolVersion).toEqual({ version: '0.15.0', comparator: 'eq' });
    }
  });

  it('resolves the PubKey/CRS version served by known public relayers', () => {
    expect(protocolContextForChain(mainnet).pubKeyCrsVersion).toEqual({
      version: '1.4.0-alpha.3',
      comparator: 'eq',
    });
    expect(protocolContextForChain(sepolia).pubKeyCrsVersion).toEqual({
      version: '1.4.0-alpha.3',
      comparator: 'eq',
    });
    // Trailing slashes are normalized before matching relayer URLs.
    expect(protocolContextForChain(makeChain(`${sepolia.fhevm.relayerUrl}/`)).pubKeyCrsVersion).toEqual({
      version: '1.4.0-alpha.3',
      comparator: 'eq',
    });
  });

  it('resolves the generated PubKey/CRS version for unknown relayers', () => {
    expect(protocolContextForChain(makeChain('http://localhost:3000')).pubKeyCrsVersion).toEqual({
      version: '1.6.1',
      comparator: 'eq',
    });
  });
});
