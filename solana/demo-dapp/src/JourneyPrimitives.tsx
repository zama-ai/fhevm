import type { CSSProperties, ReactNode } from 'react';

import type { BatchLifecycle, OperatorAction } from './batchTypes';

export type TimelineStep = {
  readonly state: 'idle' | 'active' | 'complete';
  readonly title: string;
  readonly detail: string;
};

export function JourneyTimeline({
  steps,
  framed = false,
}: {
  readonly steps: readonly TimelineStep[];
  readonly framed?: boolean;
}) {
  return (
    <ol
      className={`timeline${framed ? ' framed' : ''}`}
      style={{ '--timeline-columns': steps.length } as CSSProperties}
    >
      {steps.map((step, index) => (
        <li className={step.state === 'idle' ? undefined : step.state} key={step.title}>
          <span>{step.state === 'complete' ? '✓' : index + 1}</span>
          <div>
            <strong>{step.title}</strong>
            <small>{step.detail}</small>
          </div>
        </li>
      ))}
    </ol>
  );
}

export function SettlementProgress({
  lifecycle,
  action,
}: {
  readonly lifecycle: Extract<BatchLifecycle, { kind: 'awaiting-dispatch' | 'dispatched' | 'refunding' }>;
  readonly action: OperatorAction | null;
}) {
  const phase = lifecycle.kind === 'awaiting-dispatch' ? 1 : lifecycle.kind === 'dispatched' ? 2 : 3;
  const title =
    lifecycle.kind === 'awaiting-dispatch'
      ? action === 'dispatch'
        ? 'Starting encrypted settlement'
        : lifecycle.remainingSecs > 0n
          ? 'Waiting for batch close'
          : 'Batch ready'
      : lifecycle.kind === 'dispatched'
        ? 'Verifying settlement on Solana'
        : lifecycle.refunded
          ? 'Contribution refunded'
          : 'Refunding your contribution';
  const detail =
    lifecycle.kind === 'awaiting-dispatch'
      ? lifecycle.remainingSecs > 0n
        ? `Batch closes in ~${lifecycle.remainingSecs.toString()}s`
        : 'The local keeper is advancing the batch automatically'
      : lifecycle.kind === 'dispatched'
        ? 'The encrypted batch result is being certified and finalized on-chain'
        : lifecycle.refunded
          ? 'The batch could not settle, so your exact amount is back in your private balance'
          : 'The batch could not settle, so the local keeper is returning your exact amount';

  return (
    <div className="settlement-progress">
      <div>
        <span className="operator-label">Automatic settlement</span>
        <strong role="status" aria-live="polite">
          {title}
        </strong>
        <small>{detail}</small>
      </div>
      <progress aria-label={title} max={3} value={phase} />
    </div>
  );
}

export function ActionError({
  children,
  retryLabel,
  onRetry,
}: {
  readonly children: ReactNode;
  readonly retryLabel?: string;
  readonly onRetry?: () => void;
}) {
  return (
    <div className="action-error" role="alert">
      <span>{children}</span>
      {retryLabel !== undefined && onRetry !== undefined && (
        <button type="button" onClick={onRetry}>
          {retryLabel}
        </button>
      )}
    </div>
  );
}
