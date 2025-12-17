import { TransactionReceipt } from "ethers";
import { ethers, fhevm } from "hardhat";
import { BurnableRegulatedERC7984Upgradeable, DeploymentCoordinator, FeeManager, RegulatedERC7984Upgradeable, TestERC20, WrapperUpgradeable } from "../types";
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";
import { FhevmType } from "@fhevm/hardhat-plugin";
import { expect } from "chai";
import { ClearValueType } from "@zama-fhe/relayer-sdk";

export const getEventFromABI = (receipt: TransactionReceipt | null, ABI: string) => {
  if (receipt === null) {
    throw "receipt is null";
  }
  const iface = new ethers.Interface([ABI]);

  const events = [];

  for (const log of receipt.logs) {
    try {
      const parsed = iface.parseLog(log);
      if (parsed) {
        events.push(parsed);
      }
    } catch {
      // not your event, ignore
    }
  }

  return events;
};

export const getDebugEvent = (receipt: TransactionReceipt | null) =>
  getEventFromABI(receipt, "event Debug(string varType, string varName, bytes value)");

export const debug = async (receipt: TransactionReceipt | null) => {
  const abiCoder = new ethers.AbiCoder();
  const events = getDebugEvent(receipt);

  for (const event of events) {
    const varType = event.args[0];
    const decrypt = async (value: string) => {
      if (varType == "euint64") {
        return await fhevm.publicDecryptEuint(FhevmType.euint64, abiCoder.decode(["bytes32"], value)[0]);
      } else if (varType == "address") {
        return abiCoder.decode(["address"], value)[0];
      }
    }
    const varName = event.args[1];
    const value = await decrypt(event.args[2]);
    console.log(`[debug] ${varType} ${varName} = ${value}`);
  }
}

export const getTransferInfoEvent = (receipt: TransactionReceipt | null) =>
  getEventFromABI(receipt, "event TransferInfo(address indexed from, address indexed to, bytes32 encryptedAmount, uint256 txId)");

export const getTransferFeeInfoEvent = (receipt: TransactionReceipt | null) =>
  getEventFromABI(receipt, "event TransferFeeInfo(address indexed from, address indexed to, uint256 txId, uint64 feeAmount)");

export const getBurnEvent = (receipt: TransactionReceipt | null) =>
  getEventFromABI(receipt, "event BurnInfo(address indexed from, bytes32 amount, uint256 txId)");

export const getMintEvent = (receipt: TransactionReceipt | null) =>
  getEventFromABI(receipt, "event MintInfo(address indexed to, uint64 amount, uint256 txId)");

export const getTokenRegulatorUpdatedEvent = (receipt: TransactionReceipt | null) =>
  getEventFromABI(receipt, "event TokenRegulatorUpdated(address indexed oldRegulator, address indexed newRegulator)");

export const getWrappedEvent = (receipt: TransactionReceipt | null) =>
  getEventFromABI(receipt, "event Wrapped(uint64 mintAmount, uint256 amountIn, uint256 feeAmount, address indexed to_, uint256 indexed mintTxId)");

export const getWrapDeployedEvent = (receipt: TransactionReceipt | null) =>
  getEventFromABI(
    receipt,
    "event WrapperPairDeployed(address indexed originalToken, address indexed wrapper, address indexed confidentialToken, string originalName, string originalSymbol, uint8 originalDecimals, address deployer)",
  );

export const getBatchTransferEvent = (receipt: TransactionReceipt | null) =>
  getEventFromABI(receipt, "event BatchTransfer(address indexed cToken, address indexed sender, uint256 startTxId, uint256 endTxId, uint256 fee)");

export const getRetryTransferEvent = (receipt: TransactionReceipt | null) =>
  getEventFromABI(receipt, "event RetryTransfer(address indexed cToken, address indexed sender, uint256 originalTxId, uint256 retryTxId)");

export const getUnwrapStartedEvent = (receipt: TransactionReceipt | null) =>
  getEventFromABI(receipt, "event UnwrappedStarted(bool returnVal, uint256 indexed requestId, uint256 indexed txId, address indexed to, address refund, bytes32 requestedAmount, bytes32 burnAmount)");

export const getUnwrapFinalizedEvent = (receipt: TransactionReceipt | null) =>
  getEventFromABI(receipt, "event UnwrappedFinalized(uint256 indexed requestId, bool finalizeSuccess, bool feeTransferSuccess, uint64 burnAmount, uint256 unwrapAmount, uint256 feeAmount, uint256 indexed nextTxId)");

export const getSwapStartedEvent = (receipt: TransactionReceipt | null) =>
  getEventFromABI(receipt, "event SwapStarted(bool success, string errorString, bytes errorLowLevelData)");

export const getSwapEvent = (receipt: TransactionReceipt | null) =>
  getEventFromABI(receipt, "event Swap(bool indexed success, address[] path, uint256 indexed unwrapTxId, uint256 wrapTxId, string errorReasonString, bytes errorLowLevelData)");

export const getERC20TransferEvent = (receipt: TransactionReceipt | null) =>
  getEventFromABI(receipt, "event Transfer(address indexed from, address indexed to, uint256 value)");

export async function getTxHashes(blockNumber: number) {
  const block = await ethers.provider.getBlock(blockNumber);
  if (block === null) {
    throw new Error("block is null");
  }
  return block.transactions;
}

export async function getConfidentialBalance(
  confidentialToken: RegulatedERC7984Upgradeable | BurnableRegulatedERC7984Upgradeable,
  user: HardhatEthersSigner,
  signer?: HardhatEthersSigner,
) {
  if (!signer) {
    signer = user;
  }
  const balanceHandleBob = await confidentialToken.confidentialBalanceOf(user);
  const confidentialTokenAddress = await confidentialToken.getAddress();
  return await fhevm.userDecryptEuint(
    FhevmType.euint64,
    balanceHandleBob,
    confidentialTokenAddress,
    signer,
  );
}

export async function getWrapFeeBasisPoints(wrapper: WrapperUpgradeable) {
  const adminProviderAddress = await wrapper.adminProvider();
  const adminProvider = await ethers.getContractAt("AdminProvider", adminProviderAddress);
  const feeManagerAddress = await adminProvider.feeManager();
  const feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
  return feeManager.wrapFeeBasisPoints();
}

export async function getUnwrapFeeBasisPoints(wrapper: WrapperUpgradeable) {
  const adminProviderAddress = await wrapper.adminProvider();
  const adminProvider = await ethers.getContractAt("AdminProvider", adminProviderAddress);
  const feeManagerAddress = await adminProvider.feeManager();
  const feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
  return feeManager.unwrapFeeBasisPoints();
}

export async function getUnwrapFee(wrapper: WrapperUpgradeable, amount: bigint) {
  const unwrapFeeBasisPoints = await getUnwrapFeeBasisPoints(wrapper);
  return (amount * unwrapFeeBasisPoints + 10_000n - 1n) / BigInt(10_000);
}

export const getDeployConfidentialTokenTx = async (coordinator: DeploymentCoordinator, erc20: TestERC20, signer: HardhatEthersSigner) => {
  // Get deploy fee from the coordinator's admin provider
  const adminProviderAddress = await coordinator.adminProvider();
  const adminProvider = await ethers.getContractAt("AdminProvider", adminProviderAddress);
  const feeManagerAddress = await adminProvider.feeManager();
  const feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
  const deployTokenFee = await feeManager.getDeployFee(ethers.ZeroAddress);

  return coordinator.connect(signer).deploy(await erc20.getAddress(), {
    value: deployTokenFee,
  });
};

export const deployConfidentialToken = async (coordinator: DeploymentCoordinator, erc20: TestERC20, signer: HardhatEthersSigner) => {
  // Get deploy fee from the coordinator's admin provider
  const adminProviderAddress = await coordinator.adminProvider();
  const adminProvider = await ethers.getContractAt("AdminProvider", adminProviderAddress);
  const feeManagerAddress = await adminProvider.feeManager();
  const feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
  const deployTokenFee = await feeManager.getDeployFee(ethers.ZeroAddress);

  const deployToken = await coordinator.connect(signer).deploy(erc20, { value: deployTokenFee });
  const receipt = await deployToken.wait();

  const cTokenAddress = await coordinator.getConfidentialToken(erc20);
  const wrapperAddress = await coordinator.getWrapper(erc20);
  const wrapper = await ethers.getContractAt("WrapperUpgradeable", wrapperAddress);

  return {
    cTokenAddress,
    cToken: await ethers.getContractAt("RegulatedERC7984Upgradeable", cTokenAddress),
    wrapperAddress,
    wrapper,
    receipt,
  };
};

export const deployConfidentialETH = async (coordinator: DeploymentCoordinator, signer: HardhatEthersSigner) => {
  // Get deploy fee from the coordinator's admin provider
  const adminProviderAddress = await coordinator.adminProvider();
  const adminProvider = await ethers.getContractAt("AdminProvider", adminProviderAddress);
  const feeManagerAddress = await adminProvider.feeManager();
  const feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
  const deployTokenFee = await feeManager.getDeployFee(ethers.ZeroAddress);

  const deployToken = await coordinator.connect(signer).deploy(ethers.ZeroAddress, { value: deployTokenFee });
  const receipt = await deployToken.wait();

  const cEthAddress = await coordinator.getConfidentialToken(ethers.ZeroAddress);
  const wrapperAddress = await coordinator.getWrapper(ethers.ZeroAddress);
  const wrapper = await ethers.getContractAt("WrapperUpgradeable", wrapperAddress);

  return {
    cEthAddress,
    cEth: await ethers.getContractAt("RegulatedERC7984Upgradeable", cEthAddress),
    wrapperAddress,
    wrapper,
    receipt,
  };
};

export const wrapETH = async (coordinator: DeploymentCoordinator, amount: bigint, recipient: string, signer: HardhatEthersSigner) => {
  // Get the deployed wrapper for ETH
  const wrapperAddress = await coordinator.getWrapper(ethers.ZeroAddress);
  const wrapper = await ethers.getContractAt("WrapperUpgradeable", wrapperAddress);

  // Get the confidential token to access its rate
  const cTokenAddress = await coordinator.getConfidentialToken(ethers.ZeroAddress);
  const cToken = await ethers.getContractAt("RegulatedERC7984Upgradeable", cTokenAddress);
  const rate = await cToken.rate();

  // Calculate scaled amount and fee based on scaled amount
  const scaledAmount = amount / rate;
  const adminProviderAddress = await coordinator.adminProvider();
  const adminProvider = await ethers.getContractAt("AdminProvider", adminProviderAddress);
  const feeManagerAddress = await adminProvider.feeManager();
  const feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
  const wrapFee = await feeManager.getWrapFee(scaledAmount, signer.address, recipient);

  const wrap = await wrapper.connect(signer).wrap(recipient, amount, { value: amount });
  const wrapReceipt = await wrap.wait();

  return { wrapReceipt, wrapFee };
};

export const wrapERC20 = async (
  coordinator: DeploymentCoordinator,
  erc20: TestERC20,
  amount: bigint,
  recipient: string,
  signer: HardhatEthersSigner,
) => {
  // Get the deployed wrapper for this ERC20
  const wrapperAddress = await coordinator.getWrapper(await erc20.getAddress());
  const wrapper = await ethers.getContractAt("WrapperUpgradeable", wrapperAddress);

  // Get the confidential token to access its rate
  const cTokenAddress = await coordinator.getConfidentialToken(await erc20.getAddress());
  const cToken = await ethers.getContractAt("RegulatedERC7984Upgradeable", cTokenAddress);
  const rate = await cToken.rate();

  // Calculate scaled amount and fee based on scaled amount
  const scaledAmount = amount / rate;
  const adminProviderAddress = await coordinator.adminProvider();
  const adminProvider = await ethers.getContractAt("AdminProvider", adminProviderAddress);
  const feeManagerAddress = await adminProvider.feeManager();
  const feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
  const wrapFee = await feeManager.getWrapFee(scaledAmount, signer.address, recipient);

  // Calculate actual transfer amount (rounded down to nearest multiple of rate)
  const actualTransferAmount = (amount / rate) * rate;

  const approve = await erc20.connect(signer).approve(wrapperAddress, actualTransferAmount);
  await approve.wait();

  const wrap = await wrapper.connect(signer).wrap(recipient, amount);
  const wrapReceipt = await wrap.wait();

  return { wrapReceipt, wrapFee };
};

export const confidentialApprove = async (
  cToken: RegulatedERC7984Upgradeable,
  spender: string,
  amount: bigint,
  signer: HardhatEthersSigner,
) => {
  const encryptedAmount = await fhevm
    .createEncryptedInput(await cToken.getAddress(), signer.address)
    .add64(amount)
    .encrypt();

  const confidentialApprove = await cToken
    .connect(signer)
    ["confidentialApprove(address,bytes32,bytes)"](spender, encryptedAmount.handles[0], encryptedAmount.inputProof);
  await confidentialApprove.wait();
};

export const unwrapToken = async (
  wrapper: WrapperUpgradeable,
  recipient: string,
  amount: bigint,
  signer: HardhatEthersSigner,
  refund?: string,
  callbackData?: string,
) => {
  const cTokenAddress = await wrapper.confidentialToken();
  const cToken = await ethers.getContractAt("RegulatedERC7984Upgradeable", cTokenAddress);

  const unwrapFee = await getUnwrapFee(wrapper, amount);

  const encryptedUnwrapAmount = await fhevm
    .createEncryptedInput(await cToken.getAddress(), signer.address)
    .add64(amount)
    .encrypt();

  const abiCoder = new ethers.AbiCoder();
  const data = abiCoder.encode(
    ["address", "address", "bytes"],
    [recipient, refund || recipient, callbackData || "0x"],
  )

  const unwrapTx = await cToken.connect(signer)["confidentialTransferAndCall(address,bytes32,bytes,bytes)"](
    await wrapper.getAddress(),
    encryptedUnwrapAmount.handles[0],
    encryptedUnwrapAmount.inputProof,
    data,
  );
  const unwrapReceipt = await unwrapTx.wait();

  const unwrapStartedEvents = getUnwrapStartedEvent(unwrapReceipt);
  const unwrapStartedEvent = unwrapStartedEvents[0];
  const publicDecryptResults = await fhevm.publicDecrypt([
    unwrapStartedEvent.args[5],
    unwrapStartedEvent.args[6],
  ]);

  const abiEncodedClearBurnResults = publicDecryptResults.abiEncodedClearValues;
  const decryptionProof = publicDecryptResults.decryptionProof;

  const unwrapFinalizedTx = await wrapper.connect(signer).finalizeUnwrap(
    unwrapStartedEvent.args.requestId,
    abiEncodedClearBurnResults,
    decryptionProof,
  );
  const unwrapFinalizedReceipt = await unwrapFinalizedTx.wait();

  return { unwrapReceipt, unwrapFinalizedReceipt, unwrapFee };
};

export const unwrapTokenOld = async (
  coordinator: DeploymentCoordinator,
  tokenAddress: string,
  recipient: string,
  amount: bigint,
  signer: HardhatEthersSigner,
  data?: string,
) => {
  if (data == undefined) {
    data = "0x";
  }
  // Get the deployed wrapper for this token
  const wrapperAddress = await coordinator.getWrapper(tokenAddress);
  const wrapper = await ethers.getContractAt("WrapperUpgradeable", wrapperAddress);

  const unwrapFee = await getUnwrapFee(wrapper, amount);

  const unwrapTx = await wrapper.connect(signer).unwrap(recipient, amount, data);
  const unwrapReceipt = await unwrapTx.wait();

  await fhevm.awaitDecryptionOracle();

  const txHashes = await getTxHashes(await ethers.provider.getBlockNumber());
  expect(txHashes.length).to.be.equal(1);
  const txHash = txHashes[0];
  const unwrapFinalizedReceipt = await ethers.provider.getTransactionReceipt(txHash);

  const cTokenAddress = await wrapper.confidentialToken();
  const cToken = await ethers.getContractAt("RegulatedERC7984Upgradeable", cTokenAddress);

  return { unwrapReceipt, unwrapFinalizedReceipt, unwrapFee };
};

export const getEncryptedBalance = async (cToken: RegulatedERC7984Upgradeable, account: HardhatEthersSigner, tokenAddress: string) => {
  const balanceHandle = await cToken.confidentialBalanceOf(account);
  return await fhevm.userDecryptEuint(FhevmType.euint64, balanceHandle, tokenAddress, account);
};

export const getFeeManager = async (coordinator: DeploymentCoordinator) => {
  const adminProviderAddress = await coordinator.adminProvider();
  const adminProvider = await ethers.getContractAt("AdminProvider", adminProviderAddress);
  const feeManagerAddress = await adminProvider.feeManager();
  return await ethers.getContractAt("FeeManager", feeManagerAddress);
}

export const getWrapFee = async (feeManager: FeeManager, amount: bigint, _addressFrom?: string, _addressTo?: string) => {

  return await feeManager.getWrapFee(amount, ethers.ZeroAddress, ethers.ZeroAddress)
}

export const getBatchTransferFee = async (adminProvider: AdminProvider) => {
  const feeManagerAddress = await adminProvider.feeManager();
  const feeManager = await ethers.getContractAt("FeeManager", feeManagerAddress);
  return await feeManager.getBatchTransferFee();
}

/**
 * Verifies the wrapper backing invariant: wrapper's underlying balance equals cToken total supply * rate
 * This ensures all confidential tokens are properly backed by underlying tokens in the wrapper
 *
 * @param wrapper - Wrapper contract instance
 * @param provider - Ethers provider (optional, defaults to ethers.provider)
 */
export const verifyWrapperBacking = async (
  wrapper: any,
  provider?: any
) => {
  const ethersProvider = provider || ethers.provider;

  // Get addresses from wrapper
  const wrapperAddress = await wrapper.getAddress();
  const underlyingTokenAddress = await wrapper.originalToken();
  const cTokenAddress = await wrapper.confidentialToken();

  // Get cToken contract
  const cToken = await ethers.getContractAt("RegulatedERC7984Upgradeable", cTokenAddress);

  // Get wrapper's balance of underlying token
  let wrapperBalance: bigint;
  if (underlyingTokenAddress === ethers.ZeroAddress) {
    // ETH balance
    wrapperBalance = await ethersProvider.getBalance(wrapperAddress);
  } else {
    // ERC20 balance
    const underlyingToken = await ethers.getContractAt("TestERC20", underlyingTokenAddress);
    wrapperBalance = await underlyingToken.balanceOf(wrapperAddress);
  }

  // Get confidential token total supply (encrypted)
  const totalSupplyHandle = await cToken.confidentialTotalSupply();
  const totalSupply = await fhevm.publicDecryptEuint(FhevmType.euint64, totalSupplyHandle);

  // Ensure total supply is the same as the wrapper's counter
  expect(await wrapper.mintedSupply()).to.equal(totalSupply);

  // Get rate
  const rate = await cToken.rate();

  // Calculate expected backing (totalSupply is in euint64 units, multiply by rate to get underlying units)
  const expectedBacking = totalSupply * rate;

  // Verify invariant
  if (wrapperBalance !== expectedBacking) {
    throw new Error(
      `Wrapper backing invariant violated!\n` +
      `  Wrapper balance: ${wrapperBalance}\n` +
      `  Expected (totalSupply * rate): ${expectedBacking}\n` +
      `  Total supply: ${totalSupply}\n` +
      `  Rate: ${rate}`
    );
  }

  return { wrapperBalance, totalSupply, rate, expectedBacking };
};

export const checkTotalSupply = async (erc7984: RegulatedERC7984Upgradeable, expectedAmount: number) => {
    const totalSupplyHandle = await erc7984.confidentialTotalSupply();
    const totalSupply = await fhevm.publicDecryptEuint(FhevmType.euint64, totalSupplyHandle);
    expect(totalSupply).to.equal(expectedAmount);
}

/**
 * Finalizes an unwrap operation using the public decrypt flow
 * Replaces the old fhevm.awaitDecryptionOracle() pattern
 *
 * @param receipt - Transaction receipt from confidentialTransferAndCall
 * @param wrapper - Wrapper contract instance
 * @param signer - Signer to execute finalizeUnwrap (typically the unwrapper)
 * @returns Receipt from the finalizeUnwrap transaction
 */
export const finalizeUnwrapFromReceipt = async (
  receipt: any,
  wrapper: any,
  signer: HardhatEthersSigner
) => {
  // Extract UnwrapStarted event
  const unwrapStartedEvents = getUnwrapStartedEvent(receipt);
  expect(unwrapStartedEvents.length).to.be.equal(1);
  const unwrapStartedEvent = unwrapStartedEvents[0];

  // Publicly decrypt the handles
  const publicDecryptResults = await fhevm.publicDecrypt([
    unwrapStartedEvent.args[5],
    unwrapStartedEvent.args[6],
  ]);

  const abiEncodedClearBurnResults = publicDecryptResults.abiEncodedClearValues;
  const decryptionProof = publicDecryptResults.decryptionProof;

  // Finalize the unwrap with decrypted values
  const finalizeUnwrapTx = await wrapper.connect(signer).finalizeUnwrap(
    unwrapStartedEvent.args.requestId,
    abiEncodedClearBurnResults,
    decryptionProof,
  );

  return await finalizeUnwrapTx.wait();
};
