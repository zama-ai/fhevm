
import { task, types } from 'hardhat/config';
import type { TaskArguments } from 'hardhat/types';
import { getRequiredEnvVar, loadHostAddresses } from '../utils/loadVariables';
import { readHostEnv, ensureAddressesDirectoryExists, writeHostEnvLine } from '../taskDeploy';

task('task:deployConfidentialOFT').setAction(async function (_, hre) {
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
  
    console.log(`Deploying ConfidentialOFT (bridge=${bridgeAddress}, owner=${deployer.address})...`);
    const oftFactory = await ethers.getContractFactory('ConfidentialOFT', deployer);
    const oft = await oftFactory.deploy(bridgeAddress, deployer.address);
    await oft.waitForDeployment();
    const oftAddress = await oft.getAddress();
    console.log(`ConfidentialOFT deployed at ${oftAddress} (tx ${oft.deploymentTransaction()?.hash})`);
  
    await hre.run('task:setConfidentialOFTAddress', { address: oftAddress });
  });
  
  task('task:setConfidentialOFTAddress')
    .addParam('address', 'The address of the deployed ConfidentialOFT instance')
    .setAction(async function (taskArguments: TaskArguments) {
      ensureAddressesDirectoryExists();
      const content = `CONFIDENTIAL_OFT_CONTRACT_ADDRESS=${taskArguments.address}\n`;
      try {
        writeHostEnvLine(content, 'a');
        console.log(`ConfidentialOFT address ${taskArguments.address} written successfully!`);
      } catch (err) {
        throw new Error(`Failed to write ConfidentialOFT address: ${String(err)}`);
      }
    });

task('task:wireConfidentialOFT', 'Sets the canonical remote ConfidentialOFT peer (setPeer)')
  .addParam('remoteEid', 'LayerZero V2 endpoint id of the remote chain', undefined, types.int)
  .addParam('remoteOft', 'Address of the ConfidentialOFT on the remote chain')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);

    const parsedEnv = readHostEnv();
    const localOftAddress = parsedEnv.CONFIDENTIAL_OFT_CONTRACT_ADDRESS;
    if (!localOftAddress) {
      throw new Error(
        'CONFIDENTIAL_OFT_CONTRACT_ADDRESS not found in addresses/.env.host. Run task:deployConfidentialOFT first.',
      );
    }
    if (!ethers.isAddress(taskArguments.remoteOft)) {
      throw new Error(`Invalid --remote-oft address: ${taskArguments.remoteOft}`);
    }

    const oft = await ethers.getContractAt('ConfidentialOFT', localOftAddress, deployer);
    const remoteEid: number = taskArguments.remoteEid;
    // ConfidentialOFT.setPeer takes bytes32 (forward-compat with non-EVM peers). The
    // CLI accepts a regular EVM address for convenience; we pad here to the on-chain
    // bytes32 type.
    const remoteOftBytes32 = ethers.zeroPadValue(taskArguments.remoteOft, 32);

    console.log(
      `Wiring local OFT ${localOftAddress} → peer { eid=${remoteEid}, oft=${taskArguments.remoteOft} (bytes32=${remoteOftBytes32}) }`,
    );
    const tx = await oft.setPeer(remoteEid, remoteOftBytes32);
    console.log(`  tx ${tx.hash}`);
    await tx.wait();
    console.log('ConfidentialOFT wiring done.');
  });

task('task:mintCOFT', 'Owner-mints an encrypted amount into a ConfidentialOFT instance')
  .addOptionalParam(
    'coft',
    'Address of the ConfidentialOFT to mint into (default: CONFIDENTIAL_OFT_CONTRACT_ADDRESS from addresses/.env.host)',
  )
  .addParam('handle', 'Encrypted input handle (32-byte hex) produced by `mock:encrypt`')
  .addParam('inputProof', 'Input proof (hex bytes) produced by `mock:encrypt`')
  .addOptionalParam('to', 'Recipient address for the mint (default: deployer)')
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);

    let oftAddress: string = taskArguments.coft;
    if (!oftAddress) {
      const parsedEnv = readHostEnv();
      oftAddress = parsedEnv.CONFIDENTIAL_OFT_CONTRACT_ADDRESS ?? '';
      if (!oftAddress) {
        throw new Error(
          'No OFT address: pass --coft or set CONFIDENTIAL_OFT_CONTRACT_ADDRESS in addresses/.env.host (task:setConfidentialOFTAddress).',
        );
      }
    }
    if (!ethers.isAddress(oftAddress)) {
      throw new Error(`Invalid --coft address: ${oftAddress}`);
    }

    const handle: string = taskArguments.handle;
    if (!ethers.isHexString(handle, 32)) {
      throw new Error(`--handle must be a 32-byte hex string (0x + 64 hex chars), got: ${handle}`);
    }
    const inputProof: string = taskArguments.inputProof;
    if (!ethers.isHexString(inputProof)) {
      throw new Error(`--input-proof must be a 0x-prefixed hex string, got: ${inputProof}`);
    }

    const recipient: string = taskArguments.to ?? deployer.address;
    if (!ethers.isAddress(recipient)) {
      throw new Error(`Invalid --to recipient: ${recipient}`);
    }

    const oft = await ethers.getContractAt('ConfidentialOFT', oftAddress, deployer);
    console.log(`Minting into ConfidentialOFT ${oftAddress} (recipient=${recipient}, handle=${handle})`);
    const tx = await oft.mint(recipient, handle, inputProof);
    console.log(`  tx ${tx.hash}`);
    const receipt = await tx.wait();
    console.log(`  mined in block ${receipt?.blockNumber}`);

    // Echo the resulting balance handle so the caller can decrypt it via `mock:query`.
    const balanceHandle = await oft.balanceOf(recipient);
    console.log(`Balance handle for ${recipient}: ${balanceHandle}`);
  });

task('task:bridgeCOFT', 'Bridges a ConfidentialOFT balance (or a specific amount handle) to a remote chain')
  .addParam('dstEid', 'LayerZero V2 endpoint id of the destination chain', undefined, types.int)
  .addParam('dstOft', 'Address of the ConfidentialOFT on the destination chain')
  .addOptionalParam(
    'coft',
    'Source ConfidentialOFT address (default: CONFIDENTIAL_OFT_CONTRACT_ADDRESS from addresses/.env.host)',
  )
  .addOptionalParam('recipient', 'Recipient on the destination chain (default: deployer)')
  .addOptionalParam(
    'amountHandle',
    'Specific amount handle (bytes32) to bridge for partial sends. Default: deployer\'s current balance handle (i.e. bridge the full balance).',
  )
  .addOptionalParam(
    'composeGas',
    'Destination-side lzCompose gas budget for the OFT mint',
    200_000,
    types.int,
  )
  .setAction(async function (taskArguments: TaskArguments, { ethers }) {
    const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployer = new ethers.Wallet(privateKey).connect(ethers.provider);

    let oftAddress: string = taskArguments.coft;
    if (!oftAddress) {
      const parsedEnv = readHostEnv();
      oftAddress = parsedEnv.CONFIDENTIAL_OFT_CONTRACT_ADDRESS ?? '';
      if (!oftAddress) {
        throw new Error(
          'No OFT address: pass --coft or set CONFIDENTIAL_OFT_CONTRACT_ADDRESS in addresses/.env.host (task:setConfidentialOFTAddress).',
        );
      }
    }
    if (!ethers.isAddress(oftAddress)) {
      throw new Error(`Invalid --coft address: ${oftAddress}`);
    }
    if (!ethers.isAddress(taskArguments.dstOft)) {
      throw new Error(`Invalid --dst-oft address: ${taskArguments.dstOft}`);
    }

    const dstEid: number = taskArguments.dstEid;
    const dstOft: string = taskArguments.dstOft;
    const dstOftBytes32 = ethers.zeroPadValue(dstOft, 32);
    const recipient: string = taskArguments.recipient ?? deployer.address;
    if (!ethers.isAddress(recipient)) {
      throw new Error(`Invalid --recipient address: ${recipient}`);
    }
    const composeGas: bigint = BigInt(taskArguments.composeGas);

    const oft = await ethers.getContractAt('ConfidentialOFT', oftAddress, deployer);

    // Pull the bridge straight from the OFT's immutable so the user doesn't have to
    // supply it (and so a mismatched CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS in .env
    // can't produce a wrong fee quote).
    const bridgeAddress: string = await oft.confidentialBridge();
    const bridge = await ethers.getContractAt('ConfidentialBridge', bridgeAddress, deployer);

    // Resolve the amount handle. Default = current balance (bridge the full balance);
    // explicit --amount-handle enables partial sends (e.g. a 250-out-of-1000 transfer
    // would pass a freshly-encrypted euint64=250 handle produced by `mock:encrypt`).
    let amountHandle: string;
    if (taskArguments.amountHandle) {
      amountHandle = taskArguments.amountHandle;
      if (!ethers.isHexString(amountHandle, 32)) {
        throw new Error(`--amount-handle must be a 32-byte hex string, got: ${amountHandle}`);
      }
    } else {
      const rawBalance = await oft.balanceOf(deployer.address);
      amountHandle = ethers.toBeHex(rawBalance, 32);
      if (amountHandle === ethers.ZeroHash) {
        throw new Error(
          `Source OFT ${oftAddress} has no balance handle for ${deployer.address}; run task:mintCOFT first or pass --amount-handle.`,
        );
      }
    }

    // Quote the LayerZero native fee. The payload here MUST match what
    // ConfidentialOFT.send dispatches inside the bridge — `abi.encode(recipient,
    // <amount handle>)`. Sizing must match exactly: LZ V2's `_payNative` requires
    // `msg.value == nativeFee` (not >=), so an under-sized quote causes a revert
    // with `NotEnoughNative(uint256)` (selector 0x9f704120).
    const payload = ethers.AbiCoder.defaultAbiCoder().encode(
      ['address', 'bytes32'],
      [recipient, amountHandle],
    );
    const fee = await bridge.quote(dstEid, oftAddress, dstOftBytes32, payload, [amountHandle], composeGas, '0x');
    console.log(
      `Bridging via ${oftAddress} → eid=${dstEid} oft=${dstOft} (recipient=${recipient}, handle=${amountHandle})`,
    );
    console.log(`  bridge=${bridgeAddress}, composeGas=${composeGas}, nativeFee=${fee.nativeFee.toString()} wei`);

    // The destination peer is resolved internally by the OFT from its peer registry
    // (set via task:wireConfidentialOFT); `--dst-oft` above is only used to build the
    // off-chain fee quote and must match the configured peer.
    const tx = await oft.send(dstEid, amountHandle, recipient, composeGas, {
      value: fee.nativeFee,
    });
    console.log(`  tx ${tx.hash}`);
    const receipt = await tx.wait();
    console.log(`  mined in block ${receipt?.blockNumber}`);
    console.log(
      'Track LZ delivery at https://testnet.layerzeroscan.com/ (testnet) or https://layerzeroscan.com/ (mainnet).',
    );
  });

  task('task:verifyConfidentialOFT')
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
    const oftAddress = getRequiredEnvVar('CONFIDENTIAL_OFT_CONTRACT_ADDRESS');
    const bridgeAddress = getRequiredEnvVar('CONFIDENTIAL_BRIDGE_CONTRACT_ADDRESS');
    const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
    const deployerAddress = new ethers.Wallet(deployerPrivateKey).address;
    await run('verify:verify', {
      address: oftAddress,
      constructorArguments: [bridgeAddress, deployerAddress],
    });
  });