// Run: npm run generate:templates
//
// Reads the forge artifacts in ./out, so `forge build` has to have run first (`make generate`
// does both in order). Silent on success — the output is the regenerated files, and test/templates.test.ts
// is what checks them.
import { writeTemplates } from '../generateTemplates.ts';

await writeTemplates();
