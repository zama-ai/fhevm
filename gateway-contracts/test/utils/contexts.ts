import { time } from "@nomicfoundation/hardhat-network-helpers";
import { BigNumberish, HDNodeWallet, Wallet } from "ethers";

import { createAndFundRandomWallets, createRandomWallets } from ".";
import { CoprocessorContexts } from "../../typechain-types";
import {
  CoprocessorContextTimePeriodsStruct,
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

  // Define time periods
  const coprocessorsPreActivationTimePeriod = 100;
  const coprocessorsSuspendedTimePeriod = 100;
  const timePeriods: CoprocessorContextTimePeriodsStruct = {
    preActivationTimePeriod: coprocessorsPreActivationTimePeriod,
    suspendedTimePeriod: coprocessorsSuspendedTimePeriod,
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
  await coprocessorContexts.connect(owner).addCoprocessorContext(featureSet, result.coprocessors, timePeriods);

  return {
    ...result,
    featureSet,
    timePeriods,
  };
}

export async function refreshCoprocessorContextAfterTimePeriod(
  timePeriod: BigNumberish,
  coprocessorContexts: CoprocessorContexts,
) {
  // Increase the block timestamp to reach the end of the time period
  await time.increase(timePeriod);

  // Refresh the statuses of coprocessor contexts
  await coprocessorContexts.refreshCoprocessorContextStatuses();
}
