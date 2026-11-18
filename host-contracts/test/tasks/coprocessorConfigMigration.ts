import { expect } from 'chai';
import { Interface } from 'ethers';
import fs from 'fs';
import { ethers, run, upgrades } from 'hardhat';
import http from 'http';
import type { AddressInfo } from 'net';

import { GATEWAY_CONFIG_ABI, compareWithLiveCoprocessorSet } from '../../tasks/coprocessorConfigMigration';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import { UPGRADE_TO_AND_CALL_INTERFACE } from '../../tasks/utils/upgradeProposal';
import { HOST_ADDRESSES_SOL_FILE, HOST_ENV_FILE, deployFreshEmptyUUPSProxy, readHostAddress } from './taskHelpers';

type GatewayCoprocessor = { txSenderAddress: string; signerAddress: string; s3BucketUrl: string };

const GATEWAY_CONFIG_INTERFACE = new Interface(GATEWAY_CONFIG_ABI);
const GATEWAY_CONFIG_ADDRESS = '0x00000000000000000000000000000000000000ec';

// A JSON-RPC endpoint that answers the GatewayConfig getters for a given coprocessor set, standing in
// for the Gateway chain.
class FakeGatewayRpc {
  coprocessors: GatewayCoprocessor[] = [];
  majorityThreshold = 1n;
  private server = http.createServer((req, res) => {
    let body = '';
    req.on('data', (chunk) => (body += chunk));
    req.on('end', () => {
      const request = JSON.parse(body);
      const response = Array.isArray(request) ? request.map((r) => this.answer(r)) : this.answer(request);
      res.setHeader('content-type', 'application/json');
      res.end(JSON.stringify(response));
    });
  });

  get url(): string {
    return `http://127.0.0.1:${(this.server.address() as AddressInfo).port}`;
  }

  async start(): Promise<void> {
    await new Promise<void>((resolve) => this.server.listen(0, '127.0.0.1', resolve));
  }

  async stop(): Promise<void> {
    await new Promise<void>((resolve) => this.server.close(() => resolve()));
  }

  private answer(request: { id: number; method: string; params: { data: string }[] }) {
    const result = (() => {
      switch (request.method) {
        case 'eth_chainId':
          return '0xd431';
        case 'eth_blockNumber':
          return '0x10';
        case 'eth_call':
          return this.call(request.params[0].data);
        default:
          throw new Error(`Unexpected method ${request.method}`);
      }
    })();
    return { jsonrpc: '2.0', id: request.id, result };
  }

  private call(data: string): string {
    const call = GATEWAY_CONFIG_INTERFACE.parseTransaction({ data })!;
    switch (call.name) {
      case 'getCoprocessorTxSenders':
        return GATEWAY_CONFIG_INTERFACE.encodeFunctionResult(call.name, [
          this.coprocessors.map((c) => c.txSenderAddress),
        ]);
      case 'getCoprocessor': {
        const coprocessor = this.coprocessors.find(
          (c) => c.txSenderAddress.toLowerCase() === call.args[0].toLowerCase(),
        )!;
        return GATEWAY_CONFIG_INTERFACE.encodeFunctionResult(call.name, [
          [coprocessor.txSenderAddress, coprocessor.signerAddress, coprocessor.s3BucketUrl],
        ]);
      }
      default:
        return GATEWAY_CONFIG_INTERFACE.encodeFunctionResult(call.name, [this.majorityThreshold]);
    }
  }
}

// The live set matching the COPROCESSOR_* env set of .env.example, with its own Gateway tx sender.
function liveSetMatchingEnv(): GatewayCoprocessor[] {
  return [
    {
      txSenderAddress: '0x000000000000000000000000000000000000ab01',
      signerAddress: getRequiredEnvVar('COPROCESSOR_SIGNER_ADDRESS_0'),
      s3BucketUrl: getRequiredEnvVar('COPROCESSOR_STORAGE_URL_0'),
    },
  ];
}

describe('compareWithLiveCoprocessorSet', function () {
  const signerA = '0x000000000000000000000000000000000000000a';
  const signerB = '0x000000000000000000000000000000000000000b';
  const coprocessor = (signerAddress: string, storageUrl: string) => ({
    txSenderAddress: '0x0000000000000000000000000000000000000001',
    signerAddress,
    storageUrl,
    prfEndpointUrl: '',
  });
  const context = {
    coprocessors: [coprocessor(signerA, 's3://a'), coprocessor(signerB, 's3://b')],
    majorityThreshold: 2n,
  };
  const gateway = {
    coprocessors: [
      { signerAddress: signerB, storageUrl: 's3://b' },
      { signerAddress: signerA, storageUrl: 's3://a' },
    ],
    majorityThreshold: 2n,
  };
  const inputVerifier = { signers: [signerB, signerA], threshold: 2n };

  it('matches by signer, whatever the order and the address case', function () {
    const checksummed = { signers: inputVerifier.signers.map((s) => ethers.getAddress(s)), threshold: 2n };
    expect(compareWithLiveCoprocessorSet(context, gateway, checksummed)).to.deep.equal([]);
  });

  it('reports a storage URL that differs from GatewayConfig', function () {
    const mismatches = compareWithLiveCoprocessorSet(
      { ...context, coprocessors: [coprocessor(signerA, 's3://wrong'), coprocessor(signerB, 's3://b')] },
      gateway,
      inputVerifier,
    );
    expect(mismatches).to.have.length(1);
    expect(mismatches[0]).to.match(/has storage URL "s3:\/\/wrong" in the context, "s3:\/\/a" in GatewayConfig/);
  });

  it('reports a signer missing on either side', function () {
    const signerC = '0x000000000000000000000000000000000000000C';
    const mismatches = compareWithLiveCoprocessorSet(
      { ...context, coprocessors: [coprocessor(signerA, 's3://a'), coprocessor(signerC, 's3://b')] },
      gateway,
      inputVerifier,
    );
    expect(mismatches.join('\n')).to.match(/GatewayConfig signer .* is missing from the context/);
    expect(mismatches.join('\n')).to.match(/Context signer .* is not registered in GatewayConfig/);
    expect(mismatches.join('\n')).to.match(/differ from the InputVerifier signers/);
  });

  it('reports a coprocessor count that differs from GatewayConfig', function () {
    const mismatches = compareWithLiveCoprocessorSet(
      { coprocessors: [coprocessor(signerA, 's3://a')], majorityThreshold: 2n },
      gateway,
      inputVerifier,
    );
    expect(mismatches[0]).to.equal('The context has 1 coprocessors, GatewayConfig has 2.');
  });

  it('reports a threshold that differs from GatewayConfig or InputVerifier', function () {
    expect(compareWithLiveCoprocessorSet({ ...context, majorityThreshold: 1n }, gateway, inputVerifier)).to.deep.equal([
      'The context threshold is 1, the GatewayConfig majority threshold is 2.',
      'The context threshold is 1, the InputVerifier threshold is 2.',
    ]);
  });
});

describe('CoprocessorConfig migration tasks', function () {
  const gatewayRpc = new FakeGatewayRpc();
  let originalEnvHost: string;
  let originalAddressesSol: string;
  const originalProcessAddress = process.env.COPROCESSOR_CONFIG_CONTRACT_ADDRESS;

  before(async function () {
    await gatewayRpc.start();
  });

  after(async function () {
    await gatewayRpc.stop();
  });

  beforeEach(function () {
    originalEnvHost = fs.readFileSync(HOST_ENV_FILE, 'utf-8');
    originalAddressesSol = fs.readFileSync(HOST_ADDRESSES_SOL_FILE, 'utf-8');
    gatewayRpc.coprocessors = liveSetMatchingEnv();
    gatewayRpc.majorityThreshold = BigInt(getRequiredEnvVar('COPROCESSOR_THRESHOLD'));
  });

  afterEach(function () {
    fs.writeFileSync(HOST_ENV_FILE, originalEnvHost);
    fs.writeFileSync(HOST_ADDRESSES_SOL_FILE, originalAddressesSol);
    if (originalProcessAddress === undefined) {
      delete process.env.COPROCESSOR_CONFIG_CONTRACT_ADDRESS;
    } else {
      process.env.COPROCESSOR_CONFIG_CONTRACT_ADDRESS = originalProcessAddress;
    }
  });

  const gatewayParams = () => ({ gatewayRpcUrl: gatewayRpc.url, gatewayConfigAddress: GATEWAY_CONFIG_ADDRESS });

  describe('task:deployEmptyProxyForCoprocessorConfig', function () {
    it('adds and records the empty proxy on a stack that has none', async function () {
      fs.writeFileSync(HOST_ENV_FILE, originalEnvHost.replace(/^COPROCESSOR_CONFIG_CONTRACT_ADDRESS=.*\n?/m, ''));

      await run('task:deployEmptyProxyForCoprocessorConfig');

      const proxyAddress = readHostAddress('COPROCESSOR_CONFIG_CONTRACT_ADDRESS');
      expect(await ethers.provider.getCode(proxyAddress)).to.not.equal('0x');
      expect(fs.readFileSync(HOST_ADDRESSES_SOL_FILE, 'utf-8')).to.contain(
        `address constant coprocessorConfigAdd = ${proxyAddress};`,
      );
    });

    it('is a no-op when the stack already has one', async function () {
      await run('task:deployEmptyProxyForCoprocessorConfig');
      expect(fs.readFileSync(HOST_ENV_FILE, 'utf-8')).to.equal(originalEnvHost);
    });
  });

  describe('task:checkCoprocessorSetAgainstGateway', function () {
    it('passes when the COPROCESSOR_* set matches GatewayConfig and InputVerifier', async function () {
      await run('task:checkCoprocessorSetAgainstGateway', gatewayParams());
    });

    it('lists every mismatch with GatewayConfig', async function () {
      gatewayRpc.coprocessors[0].s3BucketUrl = 's3://another-bucket';
      gatewayRpc.majorityThreshold = 2n;
      await expect(run('task:checkCoprocessorSetAgainstGateway', gatewayParams())).to.be.rejectedWith(
        /does not match the live set:\n- Signer .* "s3:\/\/another-bucket" in GatewayConfig\.\n- The context threshold is 1, the GatewayConfig majority threshold is 2\./,
      );
    });
  });

  describe('task:prepareDeployCoprocessorConfig', function () {
    it('prepares DAO calldata for the checked COPROCESSOR_* set without mutating the proxy', async function () {
      const deployer = new ethers.Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(ethers.provider);
      const proxyAddress = await deployFreshEmptyUUPSProxy(deployer);
      fs.writeFileSync(
        HOST_ENV_FILE,
        originalEnvHost.replace(
          /^COPROCESSOR_CONFIG_CONTRACT_ADDRESS=.*/m,
          `COPROCESSOR_CONFIG_CONTRACT_ADDRESS=${proxyAddress}`,
        ),
      );

      const implementationBefore = await upgrades.erc1967.getImplementationAddress(proxyAddress);
      const prepared = await run('task:prepareDeployCoprocessorConfig', { ...gatewayParams(), verifyContract: false });
      expect(await upgrades.erc1967.getImplementationAddress(proxyAddress)).to.equal(implementationBefore);

      expect(prepared.proxyAddress).to.equal(proxyAddress);
      const iface = new ethers.Interface([`function ${prepared.innerFunctionSignature}`]);
      const [coprocessors, majorityThreshold] = iface.decodeFunctionData(
        'initializeFromEmptyProxy',
        prepared.innerCalldata,
      );
      expect(coprocessors.length).to.equal(Number(getRequiredEnvVar('NUM_COPROCESSORS')));
      // The interface comes from the bare signature, so struct fields are positional: [0] is the tx sender.
      expect(coprocessors[0][0]).to.equal(getRequiredEnvVar('COPROCESSOR_TX_SENDER_ADDRESS_0'));
      expect(majorityThreshold).to.equal(BigInt(getRequiredEnvVar('COPROCESSOR_THRESHOLD')));
      expect(prepared.outerCalldata).to.equal(
        UPGRADE_TO_AND_CALL_INTERFACE.encodeFunctionData('upgradeToAndCall', [
          prepared.newImplementationAddress,
          prepared.innerCalldata,
        ]),
      );
    });

    it('refuses to prepare calldata for a set that does not match the live set', async function () {
      gatewayRpc.coprocessors[0].signerAddress = '0x000000000000000000000000000000000000dEaD';
      await expect(
        run('task:prepareDeployCoprocessorConfig', { ...gatewayParams(), verifyContract: false }),
      ).to.be.rejectedWith(
        /GatewayConfig signer 0x000000000000000000000000000000000000dEaD is missing from the context/,
      );
    });
  });
});
