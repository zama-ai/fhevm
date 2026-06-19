import { dataSlice, ethers, toBigInt } from 'ethers';

// Stand-in for LayerZero's executor (EndpointV2Mock doesn't deliver): reads PacketSent on the
// source and replays it on the destination (validatePacket -> lzReceive -> lzCompose).

// PacketV1Codec layout: version(1) nonce(8) srcEid(4) sender(32) dstEid(4) receiver(32) guid(32) message(...)
function decodePacket(encoded: string) {
  return {
    nonce: toBigInt(dataSlice(encoded, 1, 9)),
    srcEid: Number(toBigInt(dataSlice(encoded, 9, 13))),
    sender: dataSlice(encoded, 13, 45),
    guid: dataSlice(encoded, 81, 113),
    message: dataSlice(encoded, 113),
  };
}

const ENDPOINT_ABI = [
  'event PacketSent(bytes encodedPayload, bytes options, address sendLibrary)',
  'event ComposeSent(address from, address to, bytes32 guid, uint16 index, bytes message)',
  'function defaultReceiveLibrary(uint32 srcEid) view returns (address)',
  'function inboundNonce(address receiver, uint32 srcEid, bytes32 sender) view returns (uint64)',
  'function lzReceive((uint32 srcEid, bytes32 sender, uint64 nonce) origin, address receiver, bytes32 guid, bytes message, bytes extraData) payable',
  'function lzCompose(address from, address to, bytes32 guid, uint16 index, bytes message, bytes extraData) payable',
];
const MSGLIB_ABI = ['function validatePacket(bytes packet)'];
const BRIDGE_EVENTS_ABI = [
  'event HandleBridged(address indexed receiverDapp, bytes32 srcHandle, bytes32 dstHandle, bytes32 guid)',
];

const endpointIface = new ethers.Interface(ENDPOINT_ABI);
const bridgeIface = new ethers.Interface(BRIDGE_EVENTS_ABI);

// The dst signer is a shared NonceManager; re-sync its cached nonce from chain before a relay so
// gaps between deliveries (e.g. a deferred lzCompose) can't leave it behind the real account nonce.
function resyncNonce(signer: ethers.Signer) {
  const nonceManager = signer as Partial<ethers.NonceManager>;
  if (typeof nonceManager.reset === 'function') nonceManager.reset();
}

function parseEvent(logs: readonly ethers.Log[], iface: ethers.Interface, name: string, address?: string) {
  const wanted = address?.toLowerCase();
  const out: ethers.LogDescription[] = [];
  for (const log of logs) {
    if (wanted && log.address.toLowerCase() !== wanted) continue;
    let parsed: ethers.LogDescription | null = null;
    try {
      parsed = iface.parseLog({ topics: [...log.topics], data: log.data });
    } catch {
      continue;
    }
    if (parsed?.name === name) out.push(parsed);
  }
  return out;
}

export interface RelayContext {
  srcEndpoint: string; // LZ endpoint on the source chain (emits PacketSent)
  dstEndpoint: string; // LZ endpoint on the destination chain
  dstBridge: string; // destination ConfidentialBridge (lzReceive receiver + compose target)
  dstSigner: ethers.Signer; // funded signer driving the destination delivery txs
}

/** Captured `ComposeSent` args, replayable later via {@link relayCompose}. */
export interface PendingCompose {
  from: string;
  to: string;
  guid: string;
  index: number;
  message: string;
}

/**
 * Verifies + delivers a packet on the destination: validatePacket -> lzReceive (derives the dst
 * handles, emits HandleBridged, queues the compose-to-self) -> lzCompose unless `skipCompose`.
 */
async function deliver(
  ctx: RelayContext,
  packet: { encodedPacket: string; origin: { srcEid: number; sender: string; nonce: bigint }; guid: string; message: string },
  skipCompose: boolean,
): Promise<{ dstHandles: string[]; compose: PendingCompose }> {
  resyncNonce(ctx.dstSigner);
  const endpoint = new ethers.Contract(ctx.dstEndpoint, ENDPOINT_ABI, ctx.dstSigner);
  const lib = new ethers.Contract(await endpoint.defaultReceiveLibrary(packet.origin.srcEid), MSGLIB_ABI, ctx.dstSigner);
  await (await lib.validatePacket(packet.encodedPacket)).wait();

  const recvReceipt = await (
    await endpoint.lzReceive(packet.origin, ctx.dstBridge, packet.guid, packet.message, '0x')
  ).wait();
  const dstHandles = parseEvent(recvReceipt.logs, bridgeIface, 'HandleBridged', ctx.dstBridge).map(
    (e) => e.args.dstHandle as string,
  );
  if (dstHandles.length === 0) throw new Error('relay: no HandleBridged emitted on lzReceive');

  const [composeSent] = parseEvent(recvReceipt.logs, endpointIface, 'ComposeSent', ctx.dstEndpoint);
  if (!composeSent) throw new Error('relay: no ComposeSent queued on lzReceive');
  const compose: PendingCompose = {
    from: composeSent.args.from,
    to: composeSent.args.to,
    guid: composeSent.args.guid,
    index: Number(composeSent.args.index),
    message: composeSent.args.message,
  };

  if (!skipCompose) await relayCompose(ctx, compose);
  return { dstHandles, compose };
}

/**
 * Relays one bridged message; returns the dst handles (HandleBridged) in source handleList order.
 * With `skipCompose`, only `lzReceive` is delivered (the dst app's `onReceive` is left pending) —
 * run it later with {@link relayCompose} using the returned `compose`.
 */
export async function relayBridgeMessage(
  sendReceipt: ethers.TransactionReceipt,
  ctx: RelayContext,
  opts: { skipCompose?: boolean } = {},
): Promise<{ dstHandles: string[]; guid: string; compose: PendingCompose }> {
  const [packetSent] = parseEvent(sendReceipt.logs, endpointIface, 'PacketSent', ctx.srcEndpoint);
  if (!packetSent) throw new Error('relay: no PacketSent emitted by ConfidentialBridge.send');
  const encodedPacket = packetSent.args[0] as string;
  const pkt = decodePacket(encodedPacket);
  const origin = { srcEid: pkt.srcEid, sender: pkt.sender, nonce: pkt.nonce };
  const { dstHandles, compose } = await deliver(ctx, { encodedPacket, origin, guid: pkt.guid, message: pkt.message }, opts.skipCompose ?? false);
  return { dstHandles, guid: pkt.guid, compose };
}

/** Delivers the compose leg captured by {@link relayBridgeMessage} (runs the dst app's onReceive). */
export async function relayCompose(ctx: RelayContext, compose: PendingCompose): Promise<void> {
  resyncNonce(ctx.dstSigner);
  const endpoint = new ethers.Contract(ctx.dstEndpoint, ENDPOINT_ABI, ctx.dstSigner);
  await (
    await endpoint.lzCompose(compose.from, compose.to, compose.guid, compose.index, compose.message, '0x')
  ).wait();
}

/** Encodes a LayerZero V2 packet (inverse of {@link decodePacket}). */
function encodePacket(p: {
  nonce: bigint;
  srcEid: number;
  sender: string;
  dstEid: number;
  receiver: string;
  guid: string;
  message: string;
}): string {
  return ethers.concat([
    '0x01', // PacketV1 version
    ethers.toBeHex(p.nonce, 8),
    ethers.toBeHex(p.srcEid, 4),
    p.sender,
    ethers.toBeHex(p.dstEid, 4),
    p.receiver,
    p.guid,
    p.message,
  ]);
}

/**
 * Forges a destination delivery (HandleBridged + onReceive) with **no matching source
 * BridgeHandle**, so the coprocessor never associates the derived handle — used to exercise
 * `grantFallbackPlaintext`. Bypasses `bridge.send` by crafting + verifying the packet directly.
 *
 * `params.srcEid`/`srcBridge` should be a throwaway inbound channel (its own lib + peer), so the
 * forged delivery's nonce advances that channel rather than the real host<->chain-b one.
 * Returns the derived destination handles.
 */
export async function forgeDelivery(
  ctx: RelayContext,
  params: { srcEid: number; dstEid: number; srcBridge: string; srcHandle: string; dstApp: string; payload: string },
): Promise<string[]> {
  const endpoint = new ethers.Contract(ctx.dstEndpoint, ENDPOINT_ABI, ctx.dstSigner);
  const sender = ethers.zeroPadValue(params.srcBridge, 32);
  const nonce: bigint = (await endpoint.inboundNonce(ctx.dstBridge, params.srcEid, sender)) + 1n;
  const guid = ethers.zeroPadValue(ethers.toBeHex(nonce), 32);
  const message = ethers.AbiCoder.defaultAbiCoder().encode(
    ['bytes32', 'bytes32', 'bytes', 'bytes32[]'],
    [sender, ethers.zeroPadValue(params.dstApp, 32), params.payload, [params.srcHandle]],
  );
  const encodedPacket = encodePacket({
    nonce,
    srcEid: params.srcEid,
    sender,
    dstEid: params.dstEid,
    receiver: ethers.zeroPadValue(ctx.dstBridge, 32),
    guid,
    message,
  });
  const { dstHandles } = await deliver(ctx, { encodedPacket, origin: { srcEid: params.srcEid, sender, nonce }, guid, message }, false);
  return dstHandles;
}

/** Reads the LayerZero GUID assigned to a `ConfidentialBridge.send` from its `PacketSent`. */
export function extractBridgeGuid(sendReceipt: ethers.TransactionReceipt, srcEndpoint: string): string {
  const [packetSent] = parseEvent(sendReceipt.logs, endpointIface, 'PacketSent', srcEndpoint);
  if (!packetSent) throw new Error('extractBridgeGuid: no PacketSent emitted by ConfidentialBridge.send');
  return decodePacket(packetSent.args[0] as string).guid;
}

/** Real-LZ path: wait for the executor to deliver, polling the dst chain for HandleBridged by guid. */
export async function waitForBridgedHandles(
  dstProvider: ethers.Provider,
  dstBridge: string,
  guid: string,
  fromBlock: number,
  timeoutMs: number,
): Promise<string[]> {
  const topic = bridgeIface.getEvent('HandleBridged')!.topicHash;
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    const logs = await dstProvider.getLogs({ address: dstBridge, topics: [topic], fromBlock });
    const matched = logs
      .map((log) => bridgeIface.parseLog({ topics: [...log.topics], data: log.data }))
      .filter((parsed): parsed is ethers.LogDescription => parsed?.args.guid === guid)
      .map((parsed) => parsed.args.dstHandle as string);
    if (matched.length > 0) return matched;
    await new Promise((resolve) => setTimeout(resolve, 3_000));
  }
  throw new Error(`real-LZ delivery: no HandleBridged for guid ${guid} within ${timeoutMs}ms`);
}
