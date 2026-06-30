import { task, types } from 'hardhat/config';
import type { TaskArguments } from 'hardhat/types';

import { ensureAddressesDirectoryExists, readHostEnv, writeHostEnvLine } from '../taskDeploy';
import { getRequiredEnvVar, loadHostAddresses } from '../utils/loadVariables';

task('task:deployHandlesListConfidentialOApp').setAction(async function (_, hre) {
  const { ethers } = hre;
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);

  const parsedEnv = readHostEnv();
  const bridgeAddress = parsedEnv.CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS;
  if (!bridgeAddress) {
    throw new Error(
      'CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS not found in addresses/.env.host. Run task:deployBridge first.',
    );
  }

  console.log(`Deploying HandlesListConfidentialOApp (bridge=${bridgeAddress}, owner=${deployer.address})...`);
  const appFactory = await ethers.getContractFactory('HandlesListConfidentialOApp', deployer);
  const app = await appFactory.deploy(bridgeAddress, deployer.address);
  await app.waitForDeployment();
  const appAddress = await app.getAddress();
  console.log(`HandlesListConfidentialOApp deployed at ${appAddress} (tx ${app.deploymentTransaction()?.hash})`);

  await hre.run('task:setHandlesListConfidentialOAppAddress', { address: appAddress });
});

task('task:setHandlesListConfidentialOAppAddress')
  .addParam('address', 'The address of the deployed HandlesListConfidentialOApp instance')
  .setAction(async function (taskArguments: TaskArguments) {
    ensureAddressesDirectoryExists();
    const content = `HANDLES_LIST_OAPP_CONTRACT_ADDRESS=${taskArguments.address}\n`;
    try {
      writeHostEnvLine(content, 'a');
      console.log(`HandlesListConfidentialOApp address ${taskArguments.address} written successfully!`);
    } catch (err) {
      throw new Error(`Failed to write HandlesListConfidentialOApp address: ${String(err)}`);
    }
  });

task('task:wireHandlesListConfidentialOApp', 'Sets the canonical remote HandlesListConfidentialOApp peer (setPeer)')
  .addParam('remoteEid', 'LayerZero V2 endpoint id of the remote chain', undefined, types.int)
  .addParam('remoteApp', 'Address of the HandlesListConfidentialOApp on the remote chain')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);

    const parsedEnv = readHostEnv();
    const localAppAddress = parsedEnv.HANDLES_LIST_OAPP_CONTRACT_ADDRESS;
    if (!localAppAddress) {
      throw new Error(
        'HANDLES_LIST_OAPP_CONTRACT_ADDRESS not found in addresses/.env.host. Run task:deployHandlesListConfidentialOApp first.',
      );
    }
    if (!ethers.isAddress(taskArguments.remoteApp)) {
      throw new Error(`Invalid --remote-app address: ${taskArguments.remoteApp}`);
    }

    const app = await ethers.getContractAt('HandlesListConfidentialOApp', localAppAddress, deployer);
    const remoteEid: number = taskArguments.remoteEid;
    // setPeer takes bytes32 (forward-compat with non-EVM peers). The CLI accepts a
    // regular EVM address for convenience; we pad here to the on-chain bytes32 type.
    const remoteAppBytes32 = ethers.zeroPadValue(taskArguments.remoteApp, 32);

    console.log(
      `Wiring local app ${localAppAddress} → peer { eid=${remoteEid}, app=${taskArguments.remoteApp} (bytes32=${remoteAppBytes32}) }`,
    );
    const tx = await app.setPeer(remoteEid, remoteAppBytes32);
    console.log(`  tx ${tx.hash}`);
    await tx.wait();
    console.log('HandlesListConfidentialOApp wiring done.');
  });

task('task:sendHandlesList', 'Generates a list of fresh encrypted handles on-chain and bridges them to a remote peer')
  .addParam('dstEid', 'LayerZero V2 endpoint id of the destination chain', undefined, types.int)
  .addParam('dstApp', 'Address of the HandlesListConfidentialOApp on the destination chain')
  .addOptionalParam(
    'app',
    'Source HandlesListConfidentialOApp address (default: HANDLES_LIST_OAPP_CONTRACT_ADDRESS from addresses/.env.host)',
  )
  .addOptionalParam('count', 'Number of random handles to generate and bridge', 1, types.int)
  .addOptionalParam(
    'payloadLength',
    'Number of bytes in the opaque app-level payload; the payload is filled with 0xff bytes (0 → empty payload)',
    0,
    types.int,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);

    let appAddress: string = taskArguments.app;
    if (!appAddress) {
      const parsedEnv = readHostEnv();
      appAddress = parsedEnv.HANDLES_LIST_OAPP_CONTRACT_ADDRESS ?? '';
      if (!appAddress) {
        throw new Error(
          'No app address: pass --app or set HANDLES_LIST_OAPP_CONTRACT_ADDRESS in addresses/.env.host (task:setHandlesListConfidentialOAppAddress).',
        );
      }
    }
    if (!ethers.isAddress(appAddress)) {
      throw new Error(`Invalid --app address: ${appAddress}`);
    }
    if (!ethers.isAddress(taskArguments.dstApp)) {
      throw new Error(`Invalid --dst-app address: ${taskArguments.dstApp}`);
    }

    const dstEid: number = taskArguments.dstEid;
    const dstApp: string = taskArguments.dstApp;
    const dstAppBytes32 = ethers.zeroPadValue(dstApp, 32);
    const count: number = taskArguments.count;
    if (count <= 0) {
      throw new Error(`--count must be a positive integer, got: ${count}`);
    }
    const payloadLength: number = taskArguments.payloadLength;
    if (payloadLength < 0) {
      throw new Error(`--payload-length must be a non-negative integer, got: ${payloadLength}`);
    }
    // Build a payload of `payloadLength` 0xff bytes (e.g. 3 → 0xffffff). Used to exercise
    // the fee/gas impact of the opaque payload size; the destination peer does not
    // interpret its contents.
    const payload: string = payloadLength === 0 ? '0x' : '0x' + 'ff'.repeat(payloadLength);

    // Abbreviate the payload for logging so a large `--payload-length` doesn't dump a
    // wall of `ff` bytes (e.g. 0xffff..ff). Only shorten past a threshold.
    const payloadForLog: string = payloadLength > 100 ? '0xffff..ff' : payload;

    // Destination-side lzCompose runs `onConfidentialBridgeReceived`, whose cost grows
    // with the number of handles (two persistent ACL grants + an event entry per handle)
    // and the payload size (hashed into `resultBridgedHash` and copied around). Size the
    // budget with a simple linear over-estimate: base + per-handle + per-payload-byte.
    const COMPOSE_GAS_BASE = 200_000n;
    const COMPOSE_GAS_PER_HANDLE = 100_000n;
    const COMPOSE_GAS_PER_PAYLOAD_BYTE = 50n;
    const composeGas: bigint =
      COMPOSE_GAS_BASE + COMPOSE_GAS_PER_HANDLE * BigInt(count) + COMPOSE_GAS_PER_PAYLOAD_BYTE * BigInt(payloadLength);

    const app = await ethers.getContractAt('HandlesListConfidentialOApp', appAddress, deployer);

    const bridgeAddress: string = await app.confidentialBridge();

    const configuredPeer: string = await app.peers(dstEid);
    if (configuredPeer.toLowerCase() !== dstAppBytes32.toLowerCase()) {
      throw new Error(
        `--dst-app ${dstApp} (bytes32=${dstAppBytes32}) does not match the app's configured peer for eid ${dstEid} ` +
          `(${configuredPeer}). Run task:wireHandlesListConfidentialOApp to set the peer, or pass the correct --dst-app.`,
      );
    }

    const fee = await app.quoteGenerateAndSendHandlesList(dstEid, count, payload, composeGas);
    console.log(
      `Bridging ${count} handle(s) via ${appAddress} → eid=${dstEid} app=${dstApp} (payloadLength=${payloadLength}, payload=${payloadForLog})`,
    );
    console.log(`  bridge=${bridgeAddress}, composeGas=${composeGas}, nativeFee=${fee.nativeFee.toString()} wei`);

    const tx = await app.generateAndSendHandlesList(dstEid, count, payload, composeGas, {
      value: fee.nativeFee,
    });
    console.log(`  tx ${tx.hash}`);
    const receipt = await tx.wait();
    console.log(`  mined in block ${receipt?.blockNumber}`);

    // Echo the freshly generated source-side handles (owner is granted ACL allowance on
    // each, so they can be decrypted via `mock:query`).
    const sentSig = 'HandlesListConfidentialOAppSent(uint32,bytes32,bytes32[],bytes32)';
    const sentTopic = ethers.id(sentSig);
    const sentLog = receipt?.logs?.find(
      (l) => l.address.toLowerCase() === appAddress.toLowerCase() && l.topics[0] === sentTopic,
    );
    if (sentLog) {
      const [handlesListSent] = ethers.AbiCoder.defaultAbiCoder().decode(['bytes32[]', 'bytes32'], sentLog.data);
      console.log(`Source handles sent (${handlesListSent.length}):`);
      for (const h of handlesListSent) {
        console.log(`  ${h}`);
      }
    }
    console.log(
      'Track LZ delivery at https://testnet.layerzeroscan.com/ (testnet) or https://layerzeroscan.com/ (mainnet).',
    );
  });

task(
  'task:readReceivedHandlesList',
  'Reads the destination-chain handles recorded by the most recent inbound bridge delivery',
)
  .addOptionalParam(
    'app',
    'HandlesListConfidentialOApp address (default: HANDLES_LIST_OAPP_CONTRACT_ADDRESS from addresses/.env.host)',
  )
  .addOptionalParam(
    'fromBlock',
    'Block number to start scanning HandlesListConfidentialOAppReceived events from (default: latest 50k blocks)',
    undefined,
    types.int,
  )
  .addOptionalParam(
    'blockRange',
    'Max blocks per eth_getLogs request (keep under your RPC provider limit, e.g. 10000)',
    9_000,
    types.int,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    let appAddress: string = taskArguments.app;
    if (!appAddress) {
      const parsedEnv = readHostEnv();
      appAddress = parsedEnv.HANDLES_LIST_OAPP_CONTRACT_ADDRESS ?? '';
      if (!appAddress) {
        throw new Error(
          'No app address: pass --app or set HANDLES_LIST_OAPP_CONTRACT_ADDRESS in addresses/.env.host (task:setHandlesListConfidentialOAppAddress).',
        );
      }
    }
    if (!ethers.isAddress(appAddress)) {
      throw new Error(`Invalid --app address: ${appAddress}`);
    }

    const app = await ethers.getContractAt('HandlesListConfidentialOApp', appAddress);

    // The destination handles are recovered off-chain from the emitted event.
    const latest = await ethers.provider.getBlockNumber();
    const fromBlock: number =
      taskArguments.fromBlock !== undefined ? taskArguments.fromBlock : Math.max(0, latest - 50_000);

    const blockRange: number = taskArguments.blockRange;
    if (blockRange <= 0) {
      throw new Error(`--block-range must be a positive integer, got: ${blockRange}`);
    }

    // Many public RPCs cap `eth_getLogs` to a fixed block span (commonly 10k). Scan in
    // windows below that cap, walking backwards from `latest` so the most recent delivery
    // is found first, and stop at the first window that yields events.
    const filter = app.filters.HandlesListConfidentialOAppReceived();
    let events: Awaited<ReturnType<typeof app.queryFilter>> = [];
    let windowTo = latest;
    while (windowTo >= fromBlock) {
      const windowFrom = Math.max(fromBlock, windowTo - (blockRange - 1));
      const chunk = await app.queryFilter(filter, windowFrom, windowTo);
      if (chunk.length > 0) {
        events = chunk;
        break;
      }
      if (windowFrom === fromBlock) break;
      windowTo = windowFrom - 1;
    }

    if (events.length === 0) {
      console.log(
        `No HandlesListConfidentialOAppReceived events found on ${appAddress} in blocks ${fromBlock}-${latest}. ` +
          'The bridge may not have delivered yet, or widen the range with --from-block.',
      );
      return;
    }

    const last = events[events.length - 1];
    const { srcEid, srcApp, handlesListReceived, guid } = last.args;
    const dstHandles: string[] = Array.from(handlesListReceived as Iterable<string>);

    console.log(
      `Last received delivery on ${appAddress} (block ${last.blockNumber}, guid=${guid}, srcEid=${srcEid}, srcApp=${srcApp}):`,
    );
    console.log(`Destination handles (${dstHandles.length}):`);
    for (const h of dstHandles) {
      console.log(`  ${h}`);
    }

    // Cross-check the on-chain commitment against the event payload.
    const onChainHash: string = await app.resultBridgedHash(guid);
    console.log(`On-chain resultBridgedHash[${guid}] = ${onChainHash}`);
    console.log('Decrypt each destination handle with `pnpm mock:query <handle>`.');
  });

task('task:verifyHandlesListConfidentialOApp')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If addresses from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { ethers, run }) {
    if (useInternalProxyAddress) {
      loadHostAddresses();
    }
    const appAddress = getRequiredEnvVar('HANDLES_LIST_OAPP_CONTRACT_ADDRESS');
    const bridgeAddress = getRequiredEnvVar('CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS');
    const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployerAddress = new ethers.Wallet(deployerPrivateKey).address;
    await run('verify:verify', {
      address: appAddress,
      constructorArguments: [bridgeAddress, deployerAddress],
    });
  });
