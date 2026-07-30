import {
  createSolanaRpc,
  getBase64EncodedWireTransaction,
  type Transaction,
} from "@solana/kit";

type SimulationValue = {
  readonly err: unknown;
  readonly logs?: readonly string[] | null;
};

const stringifyError = (error: unknown): string =>
  JSON.stringify(error, (_key, value: unknown) =>
    typeof value === "bigint" ? value.toString() : value,
  );

export const assertSimulationSucceeded = (
  label: string,
  simulation: SimulationValue,
): void => {
  if (simulation.err === null) return;
  const error = stringifyError(simulation.err);
  const logs = simulation.logs?.join("\n") ?? "";
  throw new Error(
    logs.length > 0
      ? `${label} failed local simulation: ${error}\n${logs}`
      : `${label} failed local simulation: ${error}`,
  );
};

const simulateTransactionLocally = async (
  rpc: ReturnType<typeof createSolanaRpc>,
  transaction: Transaction,
  label: string,
  sigVerify: boolean,
): Promise<void> => {
  const wireTransaction = getBase64EncodedWireTransaction(transaction);
  const simulation = await rpc
    .simulateTransaction(wireTransaction, {
      commitment: "confirmed",
      encoding: "base64",
      sigVerify,
    })
    .send();
  assertSimulationSucceeded(label, simulation.value);
};

export const simulateUnsignedTransactionLocally = async (
  rpc: ReturnType<typeof createSolanaRpc>,
  transaction: Transaction,
  label: string,
): Promise<void> => simulateTransactionLocally(rpc, transaction, label, false);

export const simulateSignedTransactionLocally = async (
  rpc: ReturnType<typeof createSolanaRpc>,
  transaction: Transaction,
  label: string,
): Promise<void> => simulateTransactionLocally(rpc, transaction, label, true);
