Problem:
I must find the right folder skeleton + npm workspace organisation given the constraints of the fhevm mono repo
It's quite challenging

i want an autonomous npm workspace
it should be out of the root workspace
it should contain the following sub packages:

- host-contracts-cleartext (a package similar to /Users/alex/src/me/zama-ai/fhevm/sdk/host-contracts-cleartext-v13)
  it should follow the fhevm repo versioning - the workspace version should be the same as the root fhevm version
- a new package containing a future hardhat plugin for hardhat v2 (maybe hardhat-plugin/v2)
- a new package containing a future hardhat plugin for hardhat v3 (maybe hardhat-plugin/v3)
- host-contracts-cleartext should have no dependency within the workspace
- hardhat-plugin should have host-contracts-cleartext as a dependency and fhevm/sdk as a dependency as well
- for the moment, since fhevm/sdk is still part of the root workspace, we must find a way to solve this issue.
- in a short future, fhevm/sdk will be part of the same workspace
- fhevm/sdk (fhevm/sdk/js-sdk) is valid, the only problem is that it should be out of the root workspace and be moved to the new workspace
- hardhat-plugin will be published on npm
- host-contracts-cleartext will be published on npm
- one potential challenge that we will have to address after the skeleton is ready is the following:
  - the host-contracts-cleartext has a TS update function.
  - to test it, we must deploy host-contracts-cleartext v(N-1) first
  - where to pick host-contracts-cleartext v(N-1) ??
- rule: every subpackage must have the same version number equal to the fhevm version number (hardhat-plugin/v2, hardhat-plugin/v3, host-contracts-cleartext)
- rule: the package.json setup MUST follow the technique in fhevm/sdk/js-sdk. fhevm/sdk/js-sdk has 2 package.json files: /Users/alex/src/me/zama-ai/fhevm/sdk/js-sdk/package.json (internal) and /Users/alex/src/me/zama-ai/fhevm/sdk/js-sdk/src/package.json (published on npmjs). This allows a very clean and minimal package.json in npmjs
- rule: host-contracts-cleartext MUST be a Solidity package first! That can be imported in any Solidity project foundry or hardhat. The TS tools are secondary, there are optional features for deploy in TS (used by hardhat plugin for example)
- objective: in the future, fhevm/sdk/js-sdk should have host-contracts-cleartext as devDeps for testing.
  and the existing embeded host-contracts-cleartext in fhevm/sdk/js-sdk should disappear
- goal: hardhat-plugin/v2 package name could be "name": "@fhevm/hardhat-plugin". and "version": "2.x.y"
- goal: hardhat-plugin/v3 package name could be "name": "@fhevm/hardhat-plugin". and "version": "3.x.y"
- goal: maintain 2 versions of the hardhat-plugin one for each major version of HH
- rule: hardhat-plugin/v2 is a commonjs package
- rule: hardhat-plugin/v3 is an esm package
- rule: /Users/alex/experiment-skeleton is the working draft of this skeleton; iterate there, outside the fhevm repo
- rule: fhevm/sdk is not yet movable to the new workspace,
- because fhevm/sdk is not yet movable to the new workspace, i need a method to solve the problem temporarily
- rule: eslint config and prettier config must be the same as the existing ones in fhevm/sdk/js-sdk
- rule: tsconfig compile options must be the same as the existing ones in fhevm/sdk/js-sdk (i want to compile with the same checks). There is only one exception to this rule: 'module' and 'moduleResolution' because of the type of each package (esm or commonjs)

folder proposal with associated deps + devDeps:
<root>/fhevm/sdk/host-contracts-cleartext - dep: NO
<root>/fhevm/sdk/host-contracts-cleartext/pkg - dep: NO
<root>/fhevm/sdk/js-sdk - dep: NO - devDep: (tests only) host-contracts-cleartext
<root>/fhevm/sdk/hardhat-plugin/v2 - dep: js-sdk (or decouple: devDep + dep) - dep: host-contracts-cleartext (or decouple: devDep + dep)
<root>/fhevm/sdk/hardhat-plugin/v3 - dep: js-sdk (or decouple: devDep + dep) - dep: host-contracts-cleartext (or decouple: devDep + dep)
<root>/fhevm/sdk/js-sdk-cli - dep: js-sdk (or decouple: devDep + dep) - devDep: (tests only) host-contracts-cleartext
<root>/fhevm/sdk/js-solana-sdk - dep: NO
<root>/fhevm/sdk/package.json (minimal)
<root>/fhevm/sdk/prettierrc.json
<root>/fhevm/sdk/eslint.json
<root>/fhevm/sdk/tsconfig.json
<root>/fhevm/sdk/tsconfig.build.json
<root>/fhevm/sdk/tsconfig.base.json

planning:
Step 0: the workspace root, and nothing else

- Deliverable: `sdk/package.json` only -- no config files, no subpackage changes, no install.
- Workspace name `fhevm-sdk-workspace`, `private: true`, never published.
- It sits at `sdk/`, outside the root `fhevm` workspace, which must never glob `sdk/*` or a member would belong to two workspaces and hoist out of this one's toolchain.
- `js-sdk` is excluded from the member list: it is still a root-workspace member, cannot belong to two, and anything needing it consumes a published tarball until Step 2.
- `hardhat-plugin/v2` and `v3` are excluded for the same reason -- they depend on `js-sdk`.
- Members are listed by explicit path, not by glob. Superseded in one detail by what was built: a version is _not_ always a pair. Every `pkg/` shares the one published name `@fhevm/host-contracts-cleartext`, and npm rejects two workspace members with the same name (`EDUPLICATEWORKSPACE`), so only the CURRENT generation lists its `pkg/`. Today that is `v12`, `v13`, `v13/pkg` — `v12/pkg` is deliberately absent, and v12's payload stays reachable through its harness because harness manifests declare no `exports` (ARCHITECTURE.md I2, I3).
- One `tsconfig.base.json` at the root holds the compiler options every subpackage extends, `module`/`moduleResolution` excepted (rule 38).
- One eslint config at the root, extended by every subpackage.
- One prettier config at the root, extended by every subpackage.
- TypeScript >= 6 everywhere.
- Shared toolchain (compiler, linter, formatter, test runner) is declared once in the root `devDependencies`; package-specific deps stay in their package.
- Root tool versions match `sdk/js-sdk` exactly, because a workspace hoists one copy and a higher root pin silently upgrades every member -- prettier reformats between minor versions and every package runs `prettier:check` in its build.
- No `type` field at the root: `hardhat-plugin/v2` is commonjs and `v3` is esm, and any subpackage omitting its own would inherit it.
- A subpackage may override eslint/prettier rules locally, as an exception that states its reason in the file.

- i want to create a real TS project with multiple sub TS projects. ('composite: true' i believe ?)
- i want to create a real TS dependencees tree. for example <root>/fhevm/sdk/host-contracts-cleartext/v12 should be compiled first, then <root>/fhevm/sdk/host-contracts-cleartext/v13, then js-sdk (when available), then HH plugin v2/v3

Step 0-bis:

- i want to be able to test TS compiler 7 without impacting the TS 6 config

Step 1:

- specs: npm packages architecture definition (names and versions)
- specs: how to incorporate fhevm/sdk into this new workspace ?
- dev: npm packages architecture skeleton
- dev: npm packages dev dependencies setup
- dev: npm packages prettier + eslint complete setup (use the same as fhevm/sdk! this is mandatory)
- dev: npm packages test setup (vitest etc.)
- dev: npm packages CI setup + publish phase (not-activated yet)

Step 1 must be executed with great care as it is the fundation for the next step. No need to spend too much time, but we must be extremly focused otherwise we will run into troubles later on and it will be nightmareish

Step 2:

- Make fhevm/sdk independant from fhevm root workspace
- Incorporate fhevm/sdk in the new workspace
- Define a development roadmap and strategy for host-contracts-cleartext v11,v12,v13,v14,v15 in the current constrained fhevm repo (how to do that ??) - maybe in mutliple phases ? first with all versions in parallel, later migrate them one by one in each release branches ?

Step 3:

- develop host-contracts-cleartext v11 (deploy only)
- develop host-contracts-cleartext v12 (deploy + update from v11)
- develop host-contracts-cleartext v13 (deploy + update from v12)
- develop host-contracts-cleartext v14 (deploy + update from v13)
- develop host-contracts-cleartext v15 (deploy + update from v14)

Step 4:

- remove existing host-contracts-cleartext from fhevm/sdk, and use new host-contracts-cleartext/vxxx instead

Once host-contracts-cleartext is pixel-perfect, move to the hardhat plugin phase

Step 5:

- based on the existing hardhat-plugin (v2) write hardhat-plugin/v2 only
- port all existing tests in https://github.com/zama-ai/fhevm-mocks
- setup CI stuff (npm publish as well)

Step 6:

- publish on npmjs the next version of hardhat-plugin v2 ONLY

Step 7:

- based on hardhat-plugin/v2, write hardhat-plugin/v3
- port all existing tests in https://github.com/zama-ai/fhevm-mocks
- setup CI stuff (npm publish as well)

Step 8:

- Find a solution to share the tests between hh v2 and v3 (2000+ tests duplicated)
