Swapping from a confidential ERC-7984 to another confidential ERC-7984 is a bit more complex although quite simple given the usage of the `FHE` library. This example demonstrates a **1:1 exchange ratio** swap between two confidential ERC-7984 tokens using OpenZeppelin's smart contract library powered by ZAMA's FHEVM.

{% hint style="info" %}
This is a simplified example using a 1:1 exchange ratio. In production, token swaps with proper pricing would typically happen through a DEX contract such as an AMM or intent-based routing.
{% endhint %}

{% hint style="info" %}
To run this example correctly, make sure the files are placed in the following directories:

- `.sol` file → `<your-project-root-dir>/contracts/`
- `.ts` file → `<your-project-root-dir>/test/`

This ensures Hardhat can compile and test your contracts as expected.
{% endhint %}

{% tabs %}

{% tab title="SwapERC7984ToERC7984.sol" %}

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.27;

import {FHE, externalEuint64, euint64} from "@fhevm/solidity/lib/FHE.sol";
import {IERC7984} from "@openzeppelin/confidential-contracts/interfaces/IERC7984.sol";

contract SwapERC7984ToERC7984 {
    function swapConfidentialForConfidential(
        IERC7984 fromToken,
        IERC7984 toToken,
        externalEuint64 amountInput,
        bytes calldata inputProof
    ) public virtual {
        require(fromToken.isOperator(msg.sender, address(this)));

        euint64 amount = FHE.fromExternal(amountInput, inputProof);

        FHE.allowTransient(amount, address(fromToken));
        euint64 amountTransferred = fromToken.confidentialTransferFrom(msg.sender, address(this), amount);

        FHE.allowTransient(amountTransferred, address(toToken));
        toToken.confidentialTransfer(msg.sender, amountTransferred);
    }
}

```

{% endtab %}

{% tab title="ERC7984Wrapper.test.ts" %}

```typescript
import { expect } from 'chai';
import { ethers, fhevm } from 'hardhat';

describe('ERC7984ERC20WrapperExample', function () {
  let wrapper: any;
  let erc20: any;
  let owner: any;
  let user: any;

  const WRAP_AMOUNT = 1000;

  beforeEach(async function () {
    [owner, user] = await ethers.getSigners();

    // Deploy a mock ERC20 token (OZ ERC20Mock takes name, symbol, decimals)
    erc20 = await ethers.deployContract('ERC20Mock', ['Test ERC20', 'TERC', 18]);

    // Deploy the wrapper
    wrapper = await ethers.deployContract('ERC7984ERC20WrapperExample', [
      await erc20.getAddress(),
      'Confidential Token',
      'cTKN',
      'https://example.com/wrapped'
    ]);
  });

  describe('Initialization', function () {
    it('should set the correct name', async function () {
      expect(await wrapper.name()).to.equal('Confidential Token');
    });

    it('should set the correct symbol', async function () {
      expect(await wrapper.symbol()).to.equal('cTKN');
    });

    it('should reference the correct underlying token', async function () {
      expect(await wrapper.underlying()).to.equal(await erc20.getAddress());
    });
  });
});

```

{% endtab %}

{% endtabs %}
