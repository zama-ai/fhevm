import { address } from '@solana/kit';

import type { BatchPosition, BatchTarget, OperatorAction, VaultDirection, VaultMetrics } from './batchTypes';

export type OperatorRequest = {
  readonly action: OperatorAction;
  readonly direction: VaultDirection;
  readonly position: BatchPosition;
};

const record = (value: unknown, name: string): Record<string, unknown> => {
  if (typeof value !== 'object' || value === null) throw new Error(`${name} must be an object`);
  return value as Record<string, unknown>;
};

const string = (value: unknown, name: string): string => {
  if (typeof value !== 'string' || value.length === 0) throw new Error(`${name} must be a non-empty string`);
  return value;
};

export const encodeOperatorRequest = (
  action: OperatorAction,
  direction: VaultDirection,
  position: BatchPosition,
): Record<string, string> => ({
  action,
  direction,
  batchIndex: position.batchIndex.toString(),
  batch: position.batch,
  amountBaseUnits: position.amountBaseUnits.toString(),
});

export const parseOperatorRequest = (value: unknown): OperatorRequest => {
  const body = record(value, 'operator request');
  if (body.action !== 'dispatch' && body.action !== 'settle') {
    throw new Error('action must be dispatch or settle');
  }
  if (body.direction !== 'deposit' && body.direction !== 'redeem') {
    throw new Error('direction must be deposit or redeem');
  }
  const position = {
    batchIndex: BigInt(string(body.batchIndex, 'batchIndex')),
    batch: address(string(body.batch, 'batch')),
    amountBaseUnits: BigInt(string(body.amountBaseUnits, 'amountBaseUnits')),
  };
  if (position.batchIndex < 0n || position.amountBaseUnits < 0n) throw new Error('invalid batch position');
  return { action: body.action, direction: body.direction, position };
};

export const encodeBatchTarget = (target: BatchTarget): Record<string, string> => ({
  batchIndex: target.batchIndex.toString(),
  batch: target.batch,
});

export const parseBatchTarget = (value: unknown): BatchTarget => {
  const target = record(value, 'prepared batch');
  const batchIndex = BigInt(string(target.batchIndex, 'prepared batch batchIndex'));
  if (batchIndex < 0n) throw new Error('prepared batch index cannot be negative');
  return {
    batchIndex,
    batch: address(string(target.batch, 'prepared batch batch')),
  };
};

export const encodeVaultMetrics = (metrics: VaultMetrics): Record<string, string> => ({
  totalAssets: metrics.totalAssets.toString(),
  totalShares: metrics.totalShares.toString(),
});

export const parseVaultMetrics = (value: unknown): VaultMetrics => {
  const metrics = record(value, 'vault metrics');
  return {
    totalAssets: BigInt(string(metrics.totalAssets, 'vault metrics totalAssets')),
    totalShares: BigInt(string(metrics.totalShares, 'vault metrics totalShares')),
  };
};
