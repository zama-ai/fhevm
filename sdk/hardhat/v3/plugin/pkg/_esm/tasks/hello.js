// The hello task's action, in its own module: hardhat 3 loads task actions LAZILY (setAction takes an
// importer returning { default }), so an action never pays its import cost until the task actually runs.
const helloAction = () => {
    console.log('Hello from @fhevm/hardhat-plugin — the hardhat v3 cluster is alive.');
    return Promise.resolve();
};
export default helloAction;
//# sourceMappingURL=hello.js.map