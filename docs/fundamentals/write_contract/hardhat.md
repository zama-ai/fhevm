# Using Hardhat

The best way to start writing smart contracts with fhEVM is to use our [Hardhat template](https://github.com/zama-ai/fhevm-hardhat-template).

It allows you to start a fhEVM docker image and run your smart contract on it. Read the [README](https://github.com/zama-ai/fhevm-hardhat-template/blob/main/README.md) for more information.

When developing confidential contracts, we recommend to use first the mocked version of fhEVM for faster testing with `pnpm test:mock` and coverage computation via `pnpm coverage:mock`, this will lead to a better developer experience. However, keep in mind that the mocked fhEVM has some limitations and discrepencies compared to the real fhEVM node, as explained in the warning section at the end of this section.

It's essential to run tests of the final contract version using the real fhEVM. You can do this by running `pnpm test` before deployment.

## Mocked mode

For faster testing iterations, instead of launching all the tests on the local fhEVM node via `pnpm test`or `npx hardhat test` which could last several minutes, you could use instead a mocked version of the `TFHE.sol` library.
The same tests should (almost always) pass, as is, without any modification: neither the javascript files neither the solidity files need to be changed between the mocked and the real version. The mocked mode does not actually use encryption for encrypted types and runs the tests on a local hardhat node which is implementing the original EVM (i.e non-fhEVM).

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

Then open the file `coverage/index.html`. This will allow increased security by pointing out missing branches not covered yet by the current test suite.

⚠️ **Warning :** Notice that, due to intrinsic limitations of the original EVM, the mocked version differ in few corner cases from the real fhEVM, the most important change is the `TFHE.isInitialized` method which will always return `true` in the mocked version. Another big difference in mocked mode, compared to the real fhEVM implementation, is that there is no ciphertext verification neither checking that a ciphertext has been honestly obtained in the mocked version (see section `4` of the [whitepaper](../../../fhevm-whitepaper.pdf)).
This means that before deploying to production, developers still need to run the tests with the original fhEVM node, as a final check in non-mocked mode, with `pnpm test` or `npx hardhat test`.
