// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import {ConfidentialVestingWalletCliff} from "../../finance/ConfidentialVestingWalletCliff.sol";
import {ZamaEthereumConfig} from "@fhevm/solidity/config/ZamaConfig.sol";

contract TestConfidentialVestingWalletCliff is ZamaEthereumConfig, ConfidentialVestingWalletCliff {
    constructor(address beneficiary_, uint64 startTimestamp_, uint64 duration_, uint64 cliff_)
        ConfidentialVestingWalletCliff(beneficiary_, startTimestamp_, duration_, cliff_)
    {
        //
    }
}
