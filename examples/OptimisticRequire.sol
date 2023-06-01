// SPDX-License-Identifier: BSD-3-Clause-Clear

pragma solidity >=0.8.13 <0.9.0;

import '../node_modules/@openzeppelin/contracts/utils/cryptography/ECDSA.sol';
import '../node_modules/@openzeppelin/contracts/utils/cryptography/EIP712.sol';
import '../lib/TFHE.sol';

// Example use of optimistic ciphertext requires. Aims to show the different gas usage
// as compared to syncrhonous non-optimistic and plaintext ones.
contract OptimisticRequire {
  euint32 internal ct1;
  euint32 internal ct2;
  uint256 internal value;
  uint256 internal iterations;

  constructor() EIP712('Authorization token', '1') {}

  function init(bytes calldata ctBytes, uint256 _iterations) public {
    ct1 = TFHE.asEuint32(ctBytes);
    ct2 = TFHE.asEuint32(ctBytes);
    iterations = _iterations;
    value = 1;
  }

  function doWorkToPayGas() internal {
    uint256 newValue = value;
    for (uint256 i = 0; i < iterations; i++) {
      newValue += 2;
    }
    value = newValue;
  }

  function getValue() public view returns (uint256) {
    return value;
  }

  // Charge full gas as both requires are true.
  function optimisticRequireCtTrue() public {
    // True.
    TFHE.optimisticRequireCt(TFHE.lte(ct1, ct2));

    // True.
    TFHE.optimisticRequireCt(TFHE.lte(ct1, ct2));

    // Mutate state to pay for gas.
    doWorkToPayGas();
  }

  // Charge full gas as we are using optimistic requires.
  function optimisticRequireCtFalse() public {
    // True.
    TFHE.optimisticRequireCt(TFHE.lte(ct1, ct2));

    // False.
    TFHE.optimisticRequireCt(TFHE.lt(ct1, ct2));

    // Mutate state to pay for gas - we will pay for it, because we are using optimistic requires.
    doWorkToPayGas();
  }

  // Charge less than full gas, because the non-optimistic ciphertext require aborts early.
  function requireCtFalse() public {
    // True.
    TFHE.requireCt(TFHE.lte(ct1, ct2));

    // False.
    TFHE.requireCt(TFHE.lt(ct1, ct2));

    // Try to mutate state to pay for gas - we won't reach that point due to the false require.
    doWorkToPayGas();
  }

  // Must behave as requireCtFalse() in terms of gas.
  // Since gas estimation would always fail, call it without it by providing a gas value and observe transaction gas usage.
  function requireFalse() public {
    // False.
    require(false);

    // Try to mutate state to pay for gas - we won't reach that point due to the false require.
    doWorkToPayGas();
  }
}
