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

type OperatorCopy = {
  readonly awaitingTitle: string;
  readonly provingPendingTitle: string;
  readonly provingReadyTitle: string;
  readonly provingReadyDetail: string;
  readonly settleLabel: string;
};

export function OperatorPanel({
  lifecycle,
  action,
  copy,
  onAction,
}: {
  readonly lifecycle: Extract<BatchLifecycle, { kind: 'awaiting-dispatch' | 'proving' }>;
  readonly action: OperatorAction | null;
  readonly copy: OperatorCopy;
  readonly onAction: (action: OperatorAction) => void;
}) {
  if (lifecycle.kind === 'awaiting-dispatch') {
    return (
      <div className="operator-panel">
        <div>
          <span className="operator-label">Demo operator</span>
          <strong>{copy.awaitingTitle}</strong>
          <small>This is not a wallet action.</small>
        </div>
        <button
          className="panel-action"
          type="button"
          disabled={lifecycle.remainingSlots > 0n || action !== null}
          onClick={() => onAction('dispatch')}
        >
          {action === 'dispatch'
            ? 'Dispatching…'
            : lifecycle.remainingSlots > 0n
              ? `Available in ~${lifecycle.remainingSlots.toString()} slots`
              : 'Dispatch batch'}
        </button>
      </div>
    );
  }

  return (
    <div className="operator-panel">
      <div>
        <span className="operator-label">Demo operator</span>
        <strong>{lifecycle.proofReady ? copy.provingReadyTitle : copy.provingPendingTitle}</strong>
        <small>{lifecycle.proofReady ? copy.provingReadyDetail : 'Proof readiness is checked automatically.'}</small>
      </div>
      {lifecycle.proofReady && (
        <button className="panel-action" type="button" disabled={action !== null} onClick={() => onAction('settle')}>
          {action === 'settle' ? 'Settling…' : copy.settleLabel}
        </button>
      )}
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
