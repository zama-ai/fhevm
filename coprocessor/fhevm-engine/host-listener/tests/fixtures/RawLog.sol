// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

/// Test fixture: emit an already-encoded EVM log. Not a host-contract stand-in.
contract RawLog {
    function emitLog(bytes32[] calldata topics, bytes calldata data) external {
        uint256 n = topics.length;
        require(n <= 4, "too many topics");
        bytes32 t0;
        bytes32 t1;
        bytes32 t2;
        bytes32 t3;
        if (n > 0) t0 = topics[0];
        if (n > 1) t1 = topics[1];
        if (n > 2) t2 = topics[2];
        if (n > 3) t3 = topics[3];
        bytes memory payload = data;
        assembly {
            let size := mload(payload)
            let ptr := add(payload, 32)
            switch n
            case 0 { log0(ptr, size) }
            case 1 { log1(ptr, size, t0) }
            case 2 { log2(ptr, size, t0, t1) }
            case 3 { log3(ptr, size, t0, t1, t2) }
            case 4 { log4(ptr, size, t0, t1, t2, t3) }
        }
    }
}
