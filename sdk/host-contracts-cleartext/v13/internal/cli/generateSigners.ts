// Run: npm run generate:signers
//
// Silent on success: the output is the generated modules, and their content is asserted by
// test/signers.test.ts rather than eyeballed here.

import { writeSigners } from '../generateSigners.ts';

await writeSigners();
