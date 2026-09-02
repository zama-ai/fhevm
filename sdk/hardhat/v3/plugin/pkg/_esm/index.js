// The hardhat v3 fhevm plugin — hello-world skeleton proving the v3 cluster topology end to end.
//
// Hardhat 3 plugins are declarative OBJECTS (no side-effect extendEnvironment): an id, task
// definitions with LAZY action modules, and hook handlers. Verified against hardhat 3.15.
import { task } from 'hardhat/config';
const plugin = {
    id: 'fhevm',
    tasks: [
        task('hello', 'Print a greeting proving the fhevm plugin is wired into hardhat v3')
            .setAction(() => import('./tasks/hello.js'))
            .build(),
    ],
};
export default plugin;
//# sourceMappingURL=index.js.map