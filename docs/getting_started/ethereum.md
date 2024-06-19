# Deploy on Ethereum

You can deploy any fhEVM contract on Ethereum Sepolia. In this scenario, our Coprocessor will handle the homomorphic operations asynchronously.

Ensure you import the appropriate configuration before deploying to Ethereum:

```solidity
import "fhevm/networks/ethereum.sol";
import "fhevm/lib/TFHE.sol";
```
