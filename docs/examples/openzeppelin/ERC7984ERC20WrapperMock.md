This example demonstrates how to wrap between the ERC20 token into a ERC7984 token using OpenZeppelin's smart contract library powered by ZAMA's FHEVM.

{% hint style="info" %}
To run this example correctly, make sure the files are placed in the following directories:

- `.sol` file → `<your-project-root-dir>/contracts/`
- `.ts` file → `<your-project-root-dir>/test/`

This ensures Hardhat can compile and test your contracts as expected.
{% endhint %}
{% tabs %}

{% tab title="ERC7984ERC20WrapperExample.sol" %}

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {SepoliaConfig} from "@fhevm/solidity/config/ZamaConfig.sol";
import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {ERC7984ERC20Wrapper, ERC7984} from "@openzeppelin/confidential-contracts/token/ERC7984/extensions/ERC7984ERC20Wrapper.sol";

contract ERC7984ERC20WrapperExample is ERC7984ERC20Wrapper, SepoliaConfig {
    constructor(
        IERC20 token,
        string memory name,
        string memory symbol,
        string memory uri
    ) ERC7984ERC20Wrapper(token) ERC7984(name, symbol, uri) {}
}
```

{% endtabs %}
