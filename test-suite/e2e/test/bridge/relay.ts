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
  'function lzReceive((uint32 srcEid, bytes32 sender, uint64 nonce) origin, address receiver, bytes32 guid, bytes message, bytes extraData) payable',
  'function lzCompose(address from, address to, bytes32 guid, uint16 index, bytes message, bytes extraData) payable',
];
const MSGLIB_ABI = ['function validatePacket(bytes packet)'];
const BRIDGE_EVENTS_ABI = [
  'event HandleBridged(address indexed receiverDapp, bytes32 srcHandle, bytes32 dstHandle, bytes32 guid)',
];

const endpointIface = new ethers.Interface(ENDPOINT_ABI);
const bridgeIface = new ethers.Interface(BRIDGE_EVENTS_ABI);

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

/** Relays one bridged message; returns the dst handles (HandleBridged) in source handleList order. */
export async function relayBridgeMessage(
  sendReceipt: ethers.TransactionReceipt,
  ctx: RelayContext,
): Promise<{ dstHandles: string[]; guid: string }> {
  const [packetSent] = parseEvent(sendReceipt.logs, endpointIface, 'PacketSent', ctx.srcEndpoint);
  if (!packetSent) throw new Error('relay: no PacketSent emitted by ConfidentialBridge.send');
  const encodedPacket = packetSent.args[0] as string;
  const pkt = decodePacket(encodedPacket);

  const endpoint = new ethers.Contract(ctx.dstEndpoint, ENDPOINT_ABI, ctx.dstSigner);
  const libAddress: string = await endpoint.defaultReceiveLibrary(pkt.srcEid);
  const lib = new ethers.Contract(libAddress, MSGLIB_ABI, ctx.dstSigner);

  // 1. verify (sets the payload hash the endpoint requires before delivery)
  await (await lib.validatePacket(encodedPacket)).wait();

  // 2. lzReceive -> _lzReceive: derives dst handles, emits HandleBridged, queues compose-to-self
  const origin = { srcEid: pkt.srcEid, sender: pkt.sender, nonce: pkt.nonce };
  const recvReceipt = await (
    await endpoint.lzReceive(origin, ctx.dstBridge, pkt.guid, pkt.message, '0x')
  ).wait();

  const dstHandles = parseEvent(recvReceipt.logs, bridgeIface, 'HandleBridged', ctx.dstBridge).map(
    (e) => e.args.dstHandle as string,
  );
  if (dstHandles.length === 0) throw new Error('relay: no HandleBridged emitted on lzReceive');

  // 3. lzCompose -> ConfidentialBridge.lzCompose -> dstApp.onReceive
  const [composeSent] = parseEvent(recvReceipt.logs, endpointIface, 'ComposeSent', ctx.dstEndpoint);
  if (!composeSent) throw new Error('relay: no ComposeSent queued on lzReceive');
  await (
    await endpoint.lzCompose(
      composeSent.args.from,
      composeSent.args.to,
      composeSent.args.guid,
      composeSent.args.index,
      composeSent.args.message,
      '0x',
    )
  ).wait();

  return { dstHandles, guid: pkt.guid };
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
