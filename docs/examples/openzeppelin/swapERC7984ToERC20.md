This example demonstrates how to swap between a confidential token - the ERC7984 and the ERC20 tokens using OpenZeppelin's smart contract library powered by ZAMA's FHEVM.


{% hint style="info" %}
To run this example correctly, make sure the files are placed in the following directories:

- `.sol` file → `<your-project-root-dir>/contracts/`
- `.ts` file → `<your-project-root-dir>/test/`

This ensures Hardhat can compile and test your contracts as expected.
{% endhint %}
{% tabs %}

{% tab title="SwapERC7984ToERC20.sol" %}

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {FHE, externalEuint64, euint64} from "@fhevm/solidity/lib/FHE.sol";
import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {IERC7984} from "@openzeppelin/confidential-contracts/interfaces/IERC7984.sol";

contract SwapERC7984ToERC20 {
    error SwapERC7984ToERC20InvalidGatewayRequest(uint256 requestId);

    mapping(uint256 requestId => address) private _receivers;
    IERC7984 private _fromToken;
    IERC20 private _toToken;

    constructor(IERC7984 fromToken, IERC20 toToken) {
        _fromToken = fromToken;
        _toToken = toToken;
    }

    function SwapERC7984ToERC20(externalEuint64 encryptedInput, bytes memory inputProof) public {
        euint64 amount = FHE.fromExternal(encryptedInput, inputProof);
        FHE.allowTransient(amount, address(_fromToken));
        euint64 amountTransferred = _fromToken.confidentialTransferFrom(msg.sender, address(this), amount);

        bytes32[] memory cts = new bytes32[](1);
        cts[0] = euint64.unwrap(amountTransferred);
        uint256 requestID = FHE.requestDecryption(cts, this.finalizeSwap.selector);

        // register who is getting the tokens
        _receivers[requestID] = msg.sender;
    }

    function finalizeSwap(uint256 requestID, uint64 amount, bytes[] memory signatures) public virtual {
        FHE.checkSignatures(requestID, signatures);
        address to = _receivers[requestID];
        require(to != address(0), SwapERC7984ToERC20InvalidGatewayRequest(requestID));
        delete _receivers[requestID];

        if (amount != 0) {
            SafeERC20.safeTransfer(_toToken, to, amount);
        }
    }
}
```

{% endtabs %}
