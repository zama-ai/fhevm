import { mine } from "@nomicfoundation/hardhat-network-helpers";
import { BigNumberish, HDNodeWallet, Wallet } from "ethers";

import { createAndFundRandomWallets, createRandomWallets } from ".";
import { CoprocessorContexts } from "../../typechain-types";
import {
  CoprocessorContextBlockPeriodsStruct,
  CoprocessorStruct,
} from "../../typechain-types/contracts/interfaces/ICoprocessorContexts";

export enum ContextStatus {
  NotInitialized = 0,
  Generating = 1,
  PreActivation = 2,
  Active = 3,
  Suspended = 4,
  Deactivated = 5,
  Compromised = 6,
  Destroyed = 7,
}

function _createCoprocessors(
  nCoprocessors: number,
  coprocessorTxSenders: HDNodeWallet[],
  coprocessorSigners: HDNodeWallet[],
) {
  const coprocessors: CoprocessorStruct[] = [];
  for (let i = 0; i < nCoprocessors; i++) {
    coprocessors.push({
      name: `Coprocessor ${i}`,
      txSenderAddress: coprocessorTxSenders[i].address,
      signerAddress: coprocessorSigners[i].address,
      s3BucketUrl: `s3://bucket-${i}`,
    });
  }
  return { coprocessors, coprocessorTxSenders, coprocessorSigners };
}

export function createCoprocessors(nCoprocessors: number) {
  const coprocessorTxSenders = createRandomWallets(nCoprocessors);
  const coprocessorSigners = createRandomWallets(nCoprocessors);

  return _createCoprocessors(nCoprocessors, coprocessorTxSenders, coprocessorSigners);
}

export async function createAndFundCoprocessors(nCoprocessors: number) {
  const coprocessorTxSenders = await createAndFundRandomWallets(nCoprocessors);
  const coprocessorSigners = await createAndFundRandomWallets(nCoprocessors);

  return _createCoprocessors(nCoprocessors, coprocessorTxSenders, coprocessorSigners);
}

export async function addNewCoprocessorContext(
  nCoprocessors: number,
  coprocessorContexts: CoprocessorContexts,
  owner: Wallet,
  fund: boolean = false,
) {
  // Define coprocessor context fields
  const featureSet = 2030;

  // Define block periods
  const coprocessorsPreActivationBlockPeriod = 100;
  const coprocessorsSuspendedBlockPeriod = 100;
  const blockPeriods: CoprocessorContextBlockPeriodsStruct = {
    preActivationBlockPeriod: coprocessorsPreActivationBlockPeriod,
    suspendedBlockPeriod: coprocessorsSuspendedBlockPeriod,
  };

  let result: {
    coprocessors: CoprocessorStruct[];
    coprocessorTxSenders: HDNodeWallet[];
    coprocessorSigners: HDNodeWallet[];
  };

  // Create a set of coprocessors with new transaction sender and signer addresses
  // Fund them if required
  if (fund) {
    result = await createAndFundCoprocessors(nCoprocessors);
  } else {
    result = createCoprocessors(nCoprocessors);
  }

  // Add a new coprocessor context
  await coprocessorContexts.connect(owner).addCoprocessorContext(featureSet, result.coprocessors, blockPeriods);

  return {
    ...result,
    featureSet,
    blockPeriods,
  };
}

export async function refreshCoprocessorContextAfterBlockPeriod(
  blockPeriod: BigNumberish,
  coprocessorContexts: CoprocessorContexts,
) {
  // Mine the number of blocks
  await mine(blockPeriod);

  // Refresh the statuses of coprocessor contexts
  await coprocessorContexts.refreshCoprocessorContextStatuses();
}
