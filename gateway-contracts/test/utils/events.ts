import { ContractTransactionResponse, EventLog } from "ethers";

export async function getEventArgFromTxRequest(txRequest: ContractTransactionResponse, argIndex: number): Promise<any> {
  const receipt = await txRequest.wait();
  const event = receipt?.logs[0] as EventLog;
  return event?.args[argIndex];
}
