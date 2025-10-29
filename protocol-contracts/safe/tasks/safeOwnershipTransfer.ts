import Safe from "@safe-global/protocol-kit";
import MultiSendJson from "@safe-global/safe-contracts/build/artifacts/contracts/libraries/MultiSend.sol/MultiSend.json";
import { task } from "hardhat/config";

import { getRequiredEnvVar } from "./utils/loadVariables";

// Deploy the SafeSmartAccount contract
task("task:transferSafeOwnershipFromDeployer")
  .addParam(
    "newOwners",
    "Addresses of the new owners of the Safe, comma-separated",
  )
  .addParam("newThreshold", "Threshold of the Safe")
  .setAction(async function (
    { newOwners, newThreshold },
    { getNamedAccounts, ethers, network },
  ) {
    // Get the deployer
    const { deployer } = await getNamedAccounts();

    // Get the Safe contract
    const safeAddress = getRequiredEnvVar("SAFE_ADDRESS");
    const safeProxy = await ethers.getContractAt("SafeL2", safeAddress);

    // Make sure the deployer is the only owner of the SafeL2Proxy contract
    // This also means that the threshold is 1
    const safeOwners = await safeProxy.getOwners();
    if (safeOwners.length !== 1 || safeOwners[0] !== deployer) {
      throw new Error(
        `This task can only be used if the deployer is the only owner of the SafeL2Proxy contract. 
        Current owners: ${safeOwners.join(", ")}, expected: ${deployer}`,
      );
    }

    // Parse the newOwners string into an array of strings
    const newOwnersAsArray = newOwners.split(",");
    const newThresholdAsBigInt = BigInt(newThreshold);

    // WARNING: IN the following, it is not possible to use the functions from the Safe kit
    // (`createAddOwnerTx`, `createRemoveOwnerTx`, ...) like found in the tests !
    // This is because this kit has some unavoidable checks that is not very compatible with batching
    // transactions as it seems to be mainly designed for single successive transactions.
    // For example:
    // - current threshold is 1
    // - calling `createAddOwnerTx` works fine (if threshold is kept at 1)
    // - calling `createRemoveOwnerTx` will fail: the kit's `encodeRemoveOwnerData` function only
    // sees that the current threshold is 1, and automatically tries to lower it by 1, which is
    // not allowed since the threshold cannot be 0 (ensured by the kit's `validateThreshold`).
    // This happens purely because the kit is not aware that the transactions are going to be batched
    // and that the number of owners will actually be much higher.
    // Note that for a similar reason, it is not possible to initially call `createAddOwnerTx` with
    // a higher threshold than 2: the kit is only aware of the current owner and the new one when
    // creating any of the transaction, but setting a threshold higher than the total number of owners
    // is not allowed.
    // This is why we need to generate the transactions manually below.

    // 1. Generate the transactions to add the new owners and keep the threshold at 1
    // Here, we use a direct call operation to call the Safe contract
    const txsData = [];
    for (const newOwnerAddress of newOwnersAsArray) {
      const addOwnerData = safeProxy.interface.encodeFunctionData(
        "addOwnerWithThreshold",
        [newOwnerAddress, 1],
      );

      txsData.push({
        to: safeAddress,
        value: "0",
        data: addOwnerData,
        operation: 0,
      });
    }

    // 2. Remove the deployer
    // The Safe contract ues linked list to handle owners. This means it requires to know the
    // previous owner to remove an owner.
    // In this case, only some owners have been added, which means the previous owner of the deployer
    // (the initial owner) is the first owner that has been added from the above list
    const prevOwner = newOwnersAsArray[0];

    // Generate the transaction to remove the deployer from the owners and update the threshold at the same time
    // Here, we use a direct call operation to call the Safe contract
    const removeOwnerData = safeProxy.interface.encodeFunctionData(
      "removeOwner",
      [prevOwner, deployer, newThresholdAsBigInt],
    );

    txsData.push({
      to: safeAddress,
      value: "0",
      data: removeOwnerData,
      operation: 0,
    });

    // Get the MultiSend contract address
    const multiSendAddress = getRequiredEnvVar("MULTISEND_ADDRESS");
    const multiSend = await ethers.getContractAt(
      MultiSendJson.abi,
      multiSendAddress,
    );

    // 3. Generate the encoded transactions to be used in the MultiSend contract
    let encodedTransactions = "";
    for (const tx of txsData) {
      const dataLength = ethers.dataLength(tx.data);
      const encodedTx = ethers.solidityPacked(
        ["uint8", "address", "uint256", "uint256", "bytes"],
        [tx.operation, tx.to, tx.value, dataLength, tx.data],
      );
      // Encode each tx: operation (1 byte) + to (20 bytes) + value (32 bytes) + dataLength (32 bytes) + Data
      // Strip the `0x` prefix from the encoded transaction
      encodedTransactions = encodedTransactions + encodedTx.slice(2);
    }

    // Add the `0x` prefix to the encoded transactions
    encodedTransactions = "0x" + encodedTransactions;

    // Generate the meta transaction data for the `multiSend` function of the MultiSend contract
    // call using a DelegateCall operation
    const multiSendData = multiSend.interface.encodeFunctionData("multiSend", [
      encodedTransactions,
    ]);
    const multiSendTx = {
      to: multiSendAddress,
      value: "0",
      data: multiSendData,
      operation: 1,
    };

    // Define the contract networks
    const chain = await ethers.provider.getNetwork();
    const chainIdKey = chain.chainId.toString();

    const contractNetworks = {
      [chainIdKey]: {
        multiSendAddress,
        multiSendCallOnlyAddress: multiSendAddress,
      },
    };

    // Initialize a SafeKit instance with the deployer
    const safeKitDeployer = await Safe.init({
      provider: network.provider,
      signer: deployer,
      safeAddress,
      contractNetworks,
    });

    // Create, sign and execute the transaction batch to do all of the above in a single transaction,
    // using the deployer (the Safe's current only owner)
    const batch = await safeKitDeployer.createTransaction({
      transactions: [multiSendTx],
    });
    await safeKitDeployer.signTransaction(batch);
    await safeKitDeployer.executeTransaction(batch);
  });
