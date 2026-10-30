export type SenderFaultMode = 'nonce-low' | 'accepted-timeout';
export interface SenderRpcEvent {
  mode: string;
  handle?: string;
  attempt?: number;
  hash?: string;
  receiptStatus?: string;
  elapsedMs?: number;
  acceptedBeforeCancel?: boolean;
}

/** A healthy run, an unrelated request, or a partial injection cannot pass. */
export function assertSenderFault(events: SenderRpcEvent[], handles: string[], mode: SenderFaultMode): void {
  if (handles.length !== 12 || new Set(handles).size !== 12 || handles.some((h) => !/^0x[a-f0-9]{64}$/.test(h))) {
    throw new Error('twelve unique named handles required');
  }
  for (const handle of handles) {
    const own = events.filter((event) => event.handle === handle);
    if (mode === 'nonce-low') {
      const attempts = own.filter((event) => event.mode === 'nonce-low').map((event) => event.attempt);
      if (JSON.stringify(attempts) !== '[1,2,3]') throw new Error(`nonce fault not observed three times for ${handle}`);
    } else {
      const accepted = own.find(
        (event) =>
          event.mode === 'accepted-response-dropped' &&
          /^0x[a-f0-9]{64}$/.test(event.hash ?? '') &&
          event.receiptStatus === '0x1' &&
          event.acceptedBeforeCancel === true,
      );
      // Sender timeout is four seconds. Allow setup overhead, but an immediate
      // connection close does not establish a client deadline expiration.
      if (
        !accepted ||
        !own.some(
          (event) => event.mode === 'client-timeout' && event.hash === accepted.hash && (event.elapsedMs ?? 0) >= 3000,
        )
      ) {
        throw new Error(`accepted transaction and client timeout not observed for ${handle}`);
      }
    }
  }
}
