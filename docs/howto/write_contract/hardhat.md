# Using Hardhat

The best way to start writing smart contracts with fhEVM is to use our [Hardhat template](https://github.com/zama-ai/fhevm-hardhat-template).

It allows you to start a fhEVM docker image and run your smart contract on it. Read the [README](https://github.com/zama-ai/fhevm-hardhat-template/blob/main/README.md) for more information.

For faster testing iterations, instead of launching all the tests on the local fhEVM node via `pnpm test`or `npx hardhat test` which could last several minutes, you could use instead a mocked version of the `TFHE.sol` library.
The same tests should pass, as is (almost always), without any modification, neither the javascript files neither the solidity files need to be changed between the mocked and the real version.

To run the mocked tests use either:

```
pnpm test:mock
```

Or equivalently:

```
HARDHAT_NETWORK=hardhat npx hardhat test --network hardhat
```

In mocked mode, all tests should pass in few seconds instead of few minutes, allowing a better developer experience.

Furthermore, getting the coverage of tests is only possible in mocked mode. Just use the following command:

```
pnpm coverage:mock
```

Or equivalently:

```
HARDHAT_NETWORK=hardhat npx hardhat coverage-mock --network hardhat
```

then open the file `coverage/index.html`. This will allow increased security by pointing out missing branches not covered yet by the current test suite.

Notice that, due to intrinsic limitations of the original EVM, the mocked version differ in few corner cases from the real fhevm, the most important change is the TFHE.isInitialized method which will always return true in the mocked version. This means that before deploying to production, developpers still need to run the tests with the original fhevm node, as a final check in non-mocked mode, with `pnpm test` or `npx hardhat test`.
