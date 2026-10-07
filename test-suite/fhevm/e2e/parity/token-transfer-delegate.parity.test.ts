import path from "node:path";

import { parityTests } from "./parity";

await parityTests(path.join(import.meta.dir, "token-transfer-delegate.case.ts"));
