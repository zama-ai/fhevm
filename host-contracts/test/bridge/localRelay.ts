import { expect } from 'chai';
import { dataSlice, getBytes, hexlify, toBigInt, zeroPadValue } from 'ethers';
import { ethers } from 'hardhat';

/**
 * Validates the bridge e2e relay mechanism (EndpointV2Mock + LocalSimpleMessageLib): read
 * `PacketSent` on the source, then `validatePacket` + `lzReceive` + `lzCompose` on the
 * destination. The e2e runs the same steps across two RPC providers.
 */
const EID_A = 1;
const EID_B = 2;

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

async function deployEndpoint(eid: number, remoteEid: number, owner: any) {
  const endpoint = await (await ethers.getContractFactory('EndpointV2Mock', owner)).deploy(eid, owner.address);
  await endpoint.waitForDeployment();
  const lib = await (await ethers.getContractFactory('LocalSimpleMessageLib', owner)).deploy(await endpoint.getAddress());
  await lib.waitForDeployment();
  const libAddr = await lib.getAddress();
  await (await endpoint.registerLibrary(libAddr)).wait();
  await (await endpoint.setDefaultSendLibrary(remoteEid, libAddr)).wait();
  await (await endpoint.setDefaultReceiveLibrary(remoteEid, libAddr, 0)).wait();
  return { endpoint, lib };
}

describe('Local LayerZero relay mechanism', function () {
  it('delivers a message end-to-end via the relay sequence', async function () {
    const [owner] = await ethers.getSigners();

    const a = await deployEndpoint(EID_A, EID_B, owner);
    const b = await deployEndpoint(EID_B, EID_A, owner);

    const oappA = await (
      await ethers.getContractFactory('MockOApp', owner)
    ).deploy(await a.endpoint.getAddress(), owner.address);
    await oappA.waitForDeployment();
    const oappB = await (
      await ethers.getContractFactory('MockOApp', owner)
    ).deploy(await b.endpoint.getAddress(), owner.address);
    await oappB.waitForDeployment();

    await (await oappA.setPeer(EID_B, zeroPadValue(await oappB.getAddress(), 32))).wait();
    await (await oappB.setPeer(EID_A, zeroPadValue(await oappA.getAddress(), 32))).wait();

    // --- source: send ---
    const message = hexlify(getBytes('0x' + 'ab'.repeat(40)));
    const sendTx = await oappA.send(EID_B, message, '0x');
    const receipt = await sendTx.wait();

    // read PacketSent(encodedPayload, options, sendLibrary) emitted by endpoint A
    const endpointAddrA = (await a.endpoint.getAddress()).toLowerCase();
    let encodedPacket: string | undefined;
    for (const log of receipt!.logs) {
      if (log.address.toLowerCase() !== endpointAddrA) continue;
      const parsed = a.endpoint.interface.parseLog(log);
      if (parsed?.name === 'PacketSent') {
        encodedPacket = parsed.args[0] as string;
        break;
      }
    }
    expect(encodedPacket, 'PacketSent emitted').to.not.equal(undefined);

    const pkt = decodePacket(encodedPacket!);
    expect(pkt.srcEid).to.equal(EID_A);

    // --- destination relay: verify, then deliver ---
    await (await b.lib.validatePacket(encodedPacket!)).wait();

    const origin = { srcEid: pkt.srcEid, sender: pkt.sender, nonce: pkt.nonce };
    await (await b.endpoint.lzReceive(origin, await oappB.getAddress(), pkt.guid, pkt.message, '0x')).wait();

    expect(await oappB.lastReceived()).to.equal(message);

    // --- destination relay: compose (queued by _lzReceive -> sendCompose) ---
    await (
      await b.endpoint.lzCompose(await oappB.getAddress(), await oappB.getAddress(), pkt.guid, 0, pkt.message, '0x')
    ).wait();

    expect(await oappB.lastComposed()).to.equal(message);
  });
});
