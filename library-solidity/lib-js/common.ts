// Re-exports codegen sources for the tests (HH2 could not load them directly; see the migration plan, step 2).
import { ArgumentType, OperatorArguments, ReturnType } from '../codegen/src/common.js';

export type * from '../codegen/src/common.js';

export { ArgumentType, OperatorArguments, ReturnType };
