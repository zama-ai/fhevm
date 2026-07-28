import { createSolanaRpc, type Address, type Signature } from '@solana/kit';
import { useEffect, useMemo, useState } from 'react';

import {
  readDecryptionEvidence,
  readTransactionEvidence,
  type DecryptionEvidenceRecord,
} from './evidenceStore';
import { readConfidentialBalanceEvidence, type ConfidentialBalanceEvidence } from './revealShares';
import type { DemoController } from './useDemoController';

type TransactionEvidence = {
  readonly label: string;
  readonly signature: Signature;
  readonly slot: bigint;
};

const short = (value: string): string => `${value.slice(0, 7)}…${value.slice(-7)}`;

const explorerUrl = (signature: Signature, rpcUrl: string): string =>
  `https://explorer.solana.com/tx/${signature}?cluster=custom&customUrl=${encodeURIComponent(rpcUrl)}`;

const readOptionalBalanceEvidence = async (
  read: () => Promise<ConfidentialBalanceEvidence>,
): Promise<ConfidentialBalanceEvidence | null> => {
  try {
    return await read();
  } catch (error) {
    if (error instanceof Error && error.message.includes('does not exist')) return null;
    throw error;
  }
};

function CopyValue({ label, value }: { readonly label: string; readonly value: string }) {
  const [copyState, setCopyState] = useState<'idle' | 'copied' | 'failed'>('idle');
  return (
    <span className="evidence-value">
      <code translate="no">{short(value)}</code>
      <button
        type="button"
        aria-label={`Copy ${label}`}
        aria-live="polite"
        onClick={() => {
          void navigator.clipboard.writeText(value).then(() => {
            setCopyState('copied');
            globalThis.setTimeout(() => setCopyState('idle'), 1_500);
          }).catch(() => setCopyState('failed'));
        }}
      >
        {copyState === 'copied' ? 'Copied' : copyState === 'failed' ? 'Copy failed' : 'Copy'}
      </button>
    </span>
  );
}

export function DeveloperEvidence({ controller }: { readonly controller: DemoController }) {
  const { state } = controller;
  const [open, setOpen] = useState(false);
  const [transactions, setTransactions] = useState<readonly TransactionEvidence[]>([]);
  const [decryptions, setDecryptions] = useState<readonly DecryptionEvidenceRecord[]>([]);
  const [shares, setShares] = useState<ConfidentialBalanceEvidence | null>(null);
  const [claimedUsdc, setClaimedUsdc] = useState<ConfidentialBalanceEvidence | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [refreshNonce, setRefreshNonce] = useState(0);
  const session = state.connection.kind === 'ready' ? state.connection.session : null;
  const evidenceRevision = [
    state.depositLifecycle?.kind ?? 'no-deposit',
    state.depositLifecycle?.kind === 'settled' && state.depositLifecycle.claimed ? 'deposit-claimed' : '',
    state.depositOperatorAction ?? '',
    state.redeemLifecycle?.kind ?? 'no-redeem',
    state.redeemLifecycle?.kind === 'settled' && state.redeemLifecycle.claimed ? 'redeem-claimed' : '',
    state.redeemOperatorAction ?? '',
    state.completedRedeemPosition?.batchIndex.toString() ?? '',
    state.revealedShares?.handle ?? '',
    state.revealedUsdc?.handle ?? '',
  ].join(':');
  const addresses = useMemo(() => {
    if (session === null) return [];
    const values: Array<{ readonly address: Address; readonly label: string }> = [
      { address: session.signer.address, label: 'Wallet account activity' },
    ];
    if (state.deposit.kind === 'joined') {
      values.push({ address: state.deposit.result.batch, label: 'Deposit batch transaction' });
    }
    const redeemPosition =
      state.redeem.kind === 'joined' ? state.redeem.result : state.completedRedeemPosition;
    if (redeemPosition !== null) {
      values.push({ address: redeemPosition.batch, label: 'Redeem batch transaction' });
    }
    return values;
  }, [session, state.completedRedeemPosition, state.deposit, state.redeem]);

  useEffect(() => {
    if (!open || session === null) return;
    let canceled = false;
    const load = async () => {
      setLoading(true);
      try {
        const rpc = createSolanaRpc(session.config.rpcUrl);
        const signatureLists = await Promise.all(
          addresses.map(async ({ address, label }) => ({
            label,
            signatures: await rpc
              .getSignaturesForAddress(address, { commitment: 'confirmed', limit: 4 })
              .send(),
          })),
        );
        const discoveredTransactions = signatureLists
          .flatMap(({ label, signatures }) =>
            signatures.map(({ signature, slot }) => ({ label, signature, slot })),
          )
          .sort((left, right) => (left.slot === right.slot ? 0 : left.slot > right.slot ? -1 : 1))
          .slice(0, addresses.length * 4);
        const storedTransactions = readTransactionEvidence(session);
        const storedStatuses =
          storedTransactions.length === 0
            ? []
            : (
                await rpc
                  .getSignatureStatuses(
                    storedTransactions.map(({ signature }) => signature),
                    { searchTransactionHistory: true },
                  )
                  .send()
              ).value;
        const verifiedStoredTransactions = storedTransactions.flatMap((record, index) =>
          storedStatuses[index] === null ? [] : [{ ...record, slot: -1n }],
        );
        const seen = new Set<string>();
        const nextTransactions = [
          ...verifiedStoredTransactions,
          ...discoveredTransactions,
        ].filter(({ signature }) => {
          if (seen.has(signature)) return false;
          seen.add(signature);
          return true;
        });
        const [nextShares, nextUsdc] = await Promise.all([
          readOptionalBalanceEvidence(() =>
            readConfidentialBalanceEvidence(session, session.config.mints.payoutConfidential),
          ),
          readOptionalBalanceEvidence(() =>
            readConfidentialBalanceEvidence(session, session.config.mints.joinConfidential),
          ),
        ]);
        session.assertActive();
        if (!canceled) {
          setTransactions(nextTransactions);
          setDecryptions(readDecryptionEvidence(session));
          setShares(nextShares);
          setClaimedUsdc(nextUsdc);
          setError(null);
        }
      } catch (loadError) {
        if (!canceled) setError(loadError instanceof Error ? loadError.message : String(loadError));
      } finally {
        if (!canceled) setLoading(false);
      }
    };
    void load();
    return () => {
      canceled = true;
    };
  }, [addresses, evidenceRevision, open, refreshNonce, session]);

  if (session === null) return null;

  return (
    <details
      className="developer-evidence"
      open={open}
      onToggle={(event) => setOpen(event.currentTarget.open)}
    >
      <summary>
        <span>
          <strong>Developer evidence</strong>
          <small>Localnet transactions & encrypted state</small>
        </span>
      </summary>
      <div className="evidence-content">
        <div className="evidence-toolbar">
          <span role="status" aria-live="polite">
            {loading ? 'Refreshing localnet evidence…' : 'Live localnet evidence'}
          </span>
          <button type="button" disabled={loading} onClick={() => setRefreshNonce((value) => value + 1)}>
            Refresh
          </button>
        </div>
        <dl>
          <div>
            <dt>RPC</dt>
            <dd>
              <CopyValue label="RPC URL" value={session.config.rpcUrl} />
            </dd>
          </div>
          <div>
            <dt>Wallet</dt>
            <dd>
              <CopyValue label="wallet address" value={session.signer.address} />
            </dd>
          </div>
          {shares !== null && (
            <>
              <div>
                <dt>cShares handle</dt>
                <dd>
                  <CopyValue label="cShares handle" value={shares.handle} />
                </dd>
              </div>
              <div>
                <dt>Encrypted-value account</dt>
                <dd>
                  <CopyValue label="encrypted-value account" value={shares.encryptedValue} />
                </dd>
              </div>
            </>
          )}
          {claimedUsdc !== null && (
            <div>
              <dt>cUSDC handle</dt>
              <dd>
                <CopyValue label="cUSDC handle" value={claimedUsdc.handle} />
              </dd>
            </div>
          )}
        </dl>

        {transactions.length > 0 && (
          <div className="evidence-transactions">
            <strong>Recent on-chain activity</strong>
            <ul>
              {transactions.map((transaction) => (
                <li key={transaction.signature}>
                  <span>{transaction.label}</span>
                  <CopyValue label="transaction signature" value={transaction.signature} />
                  <a
                    href={explorerUrl(transaction.signature, session.config.rpcUrl)}
                    target="_blank"
                    rel="noreferrer"
                  >
                    Explorer
                  </a>
                </li>
              ))}
            </ul>
          </div>
        )}
        {decryptions.length > 0 && (
          <div className="evidence-decryptions">
            <strong>Recent user decryptions</strong>
            <ul>
              {decryptions.slice(0, 4).map((decryption) => (
                <li key={decryption.jobId}>
                  <span>{decryption.label}</span>
                  <span className="decryption-identifiers">
                    <CopyValue label="decryption handle" value={decryption.handle} />
                    <CopyValue label="decryption job id" value={decryption.jobId} />
                  </span>
                  <small>
                    {(decryption.totalElapsedMs / 1_000).toFixed(2)}s wallet→cleartext ·{' '}
                    {(decryption.queueToResponseMs / 1_000).toFixed(2)}s queued→response
                  </small>
                </li>
              ))}
            </ul>
          </div>
        )}
        {error !== null && (
          <p className="evidence-error" role="status" aria-live="polite">
            Evidence is temporarily unavailable: {error}
          </p>
        )}
        <p className="evidence-note">
          Demo-only: faucet funding, automatic keeper actions, 7% illustrative APY, and fast-forwarding. Encryption,
          settlement, claims, transactions, KMS authorization, and browser decryption use the real local stack.
          Timings are individual observations, not a p95 benchmark.
        </p>
      </div>
    </details>
  );
}
