export interface GatewayEventIdentity {
  handle: string; eventBlock: number; eventBlockHash: string; eventTxHash: string; eventLogIndex: number;
}
const optional = (value: unknown) => String(value ?? '').replace(/^Some\(/, '').replace(/\)$/, '').replaceAll('"', '').toLowerCase();

/** Only a successful decoded consensus comparison emits this precise warning.
 * A watermark, unrelated drift warning, or process startup is not an event ack. */
export const gatewayEventWarnings = (logs: string, event: GatewayEventIdentity): string[] => logs.split('\n').filter((line) => {
  try {
    const record = JSON.parse(line.slice(line.indexOf('{'))) as {level?:string; fields?:Record<string,unknown>};
    const fields = record.fields ?? {};
    return record.level?.toLowerCase() === 'warn' && fields.message === 'Drift detected: local digest does not match consensus' &&
      optional(fields.handle) === event.handle.toLowerCase() && Number(fields.block_number) === event.eventBlock &&
      optional(fields.block_hash) === event.eventBlockHash.toLowerCase() && optional(fields.tx_hash) === event.eventTxHash.toLowerCase() &&
      optional(fields.log_index) !== '' && Number(optional(fields.log_index)) === event.eventLogIndex && fields.source === 'consensus';
  } catch { return false; }
});
