# Architecture

Use the existing boundaries when adding behavior.

- `src/cli`: command registration, argument parsing, stdout/stderr behavior.
- `src/flows`: orchestration only. Compose config, SDK, contract, ACL, transaction, and value helpers here. Group command families into subfolders such as `fhe-test`, `public-decrypt`, `user-decrypt`, and `delegated-user-decrypt`.
- `src/fhevm`: `@fhevm/sdk` adapters and SDK response normalization.
- `src/fhe-test`: FHETest ABI and contract reads/writes.
- `src/acl`: ACL delegation reads/writes.
- `src/config`: network registry, runtime config, account loading, and client contexts.
- `src/values`: clear-value parsing, random values, and JSON serialization helpers.
- `src/shared`: cross-cutting helpers like progress and transaction waiting.

When adding a command, prefer:

1. CLI module in `src/cli/commands`.
2. Flow module in the matching `src/flows/<command-family>` folder.
3. Adapter changes only at the boundary being crossed.

Keep raw SDK response casts and raw contract calls out of CLI and flow code when practical.
