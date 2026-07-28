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
  readonly lifecycle: Extract<BatchLifecycle, { kind: 'awaiting-dispatch' | 'proving' }>;
  readonly action: OperatorAction | null;
}) {
  const phase = lifecycle.kind === 'awaiting-dispatch' ? 1 : lifecycle.proofReady ? 3 : 2;
  const title =
    lifecycle.kind === 'awaiting-dispatch'
      ? action === 'dispatch'
        ? 'Starting encrypted settlement'
        : lifecycle.remainingSlots > 0n
          ? 'Waiting for batch close'
          : 'Batch ready'
      : lifecycle.proofReady || action === 'settle'
        ? 'Verifying settlement on Solana'
        : 'Processing encrypted settlement';
  const detail =
    lifecycle.kind === 'awaiting-dispatch'
      ? lifecycle.remainingSlots > 0n
        ? `Batch closes in ~${lifecycle.remainingSlots.toString()} slots`
        : 'The local keeper is advancing the batch automatically'
      : lifecycle.proofReady
        ? 'The proof is ready and is being finalized on-chain'
        : 'The privacy service is computing the encrypted batch result';

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

export function ClaimPanel({
  title,
  detail,
  label,
  busyLabel,
  busy,
  onClaim,
}: {
  readonly title: string;
  readonly detail: string;
  readonly label: string;
  readonly busyLabel: string;
  readonly busy: boolean;
  readonly onClaim: () => void;
}) {
  return (
    <div className="claim-panel">
      <div>
        <strong>{title}</strong>
        <small>{detail}</small>
      </div>
      <button className="panel-action" type="button" disabled={busy} onClick={onClaim}>
        {busy ? busyLabel : label}
      </button>
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
