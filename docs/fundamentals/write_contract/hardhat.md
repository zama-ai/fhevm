# Using Hardhat
This document explains how to start writing smart contract using [Zama Hardhat template](https://github.com/zama-ai/fhevm-hardhat-template).

The Hardhat template allows you to start a fhEVM docker image and run your smart contract on it. Refer to the [README](https://github.com/zama-ai/fhevm-hardhat-template/blob/main/README.md) for more information.

## Developing and testing

When developing confidential contracts, we recommend to use the first the mocked mode of fhEVM for better developer experience:
-  For faster testing: `pnpm test:mock`   
-  For coverage computation: `pnpm coverage:mock` 

{% hint style="info" %}
Note that the mocked fhEVM has limitations and discrepancies compared to the real fhEVM node. Refer to the warning section below for details.
{% endhint %}

Ensure to run tests of the final contract version using the real fhEVM. To do this, run `pnpm test` before deployment.


## Mocked mode

For faster testing iterations, use a mocked version of the `TFHE.sol` library instead of launching all the tests on the local fhEVM node via `pnpm test`or `npx hardhat test`, which could last several minutes.

The same tests should (almost always) pass without any modification: neither the javascript files neither the solidity files need to be changed between the mocked and the real version. 

The mocked mode does not actually encrypt the encrypted types. It runs the tests on a local hardhat node which is implementing the original EVM (non-fhEVM).

Run the mocked tests using:

```
pnpm test:mock
```

Or equivalently:

```
HARDHAT_NETWORK=hardhat npx hardhat test --network hardhat
```

In mocked mode, all tests should pass in few seconds instead of few minutes, allowing a better developer experience.

Furthermore, getting the coverage of tests is only possible in mocked mode. Use:

```
pnpm coverage:mock
```

Or equivalently:

```
HARDHAT_NETWORK=hardhat npx hardhat coverage-mock --network hardhat
```

Then open the file `coverage/index.html`. This increases security by pointing out missing branches not covered yet by the current test suite.

{% hint style="warning" %} Due to intrinsic limitations of the original EVM, the mocked version differ in few corner cases from the real fhEVM. The most important change is that the `TFHE.isInitialized` method always returns `true` in the mocked version. 

Another big difference in mocked mode, compared to the real fhEVM implementation, is that there is no ciphertext verification, neither the checking if a ciphertext has been honestly obtained (see section `4` of the [whitepaper](../../../fhevm-whitepaper.pdf)).

Before deploying to production, you must run the tests with the original fhEVM node as a final check in non-mocked mode, using `pnpm test` or `npx hardhat test`.{% endhint %}
