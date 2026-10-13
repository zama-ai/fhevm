import { expect } from 'chai';
import fs from 'fs';

import { VECTORS_PATH, serializeVectors } from '../../scripts/generateProtocolConfigVectors';

describe('ProtocolConfig EIP-712 test vectors', function () {
  it('match the checked-in JSON (run scripts/generateProtocolConfigVectors.ts to update)', function () {
    expect(fs.readFileSync(VECTORS_PATH, 'utf8')).to.equal(serializeVectors());
  });
});
