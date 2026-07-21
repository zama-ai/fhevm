import type { Hex } from 'viem';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, it, expect, beforeAll } from 'vitest';
import { getAddress } from 'viem';
import { setFhevmRuntimeConfig } from '@fhevm/sdk/viem';
import { parseTransportKeyPair, serializeTransportKeyPair } from '@fhevm/sdk/actions/chain';
import { getViemTestConfig, type CreateViemDecryptClientFn, type FheTestViemConfig } from '../setup-viem.js';
import { createLogger } from '../setupCommon.js';

////////////////////////////////////////////////////////////////////////////////
//
// Golden serialized-permit fixtures — SDK-version serialization compatibility.
//
// A permit serialized by ANY released SDK version must keep parsing in every
// later version (dApps cache them for up to 365 days and upgrade the SDK
// independently). These fixtures freeze the serialized format: the JSON files
// under test/fheTest/fixtures/permits/ were produced by the SDK version noted
// inside each file and are committed forever — a future serialization or
// validation change that breaks them breaks real cached permits.
//
// Verify (default):
//   CHAIN=localstack_v12 npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.permitFixtures.test.ts
//
// Generate a missing fixture for a chain (requires the chain's stack running):
//   GENERATE_PERMIT_FIXTURES=1 CHAIN=localstack_v12 npx vitest run --config test/fheTest/vitest.config.ts viem/clientDecrypt.permitFixtures.test.ts
//
// Fixtures are chain-specific (the EIP-712 signature covers the host chain id
// and the gateway Decryption address), deterministic for localstack chains,
// and signed with the public test mnemonic — no real key material.
//
// Parse-only by design: parsing validates structure and signature but not
// expiry or the KMS context, so fixtures never rot. (Decrypt-ability of stale
// permits is covered by clientDecrypt.stalePermitMigration.tests.ts.)
//
////////////////////////////////////////////////////////////////////////////////

const FIXTURES_DIR = join(dirname(fileURLToPath(import.meta.url)), '..', 'fixtures', 'permits');

function readSdkVersion(): string {
  try {
    const packageJsonPath = join(dirname(fileURLToPath(import.meta.url)), '..', '..', '..', 'src', 'package.json');
    return (JSON.parse(readFileSync(packageJsonPath, 'utf8')) as { version: string }).version;
  } catch {
    return 'unknown';
  }
}

// Frozen validity window: 2026-07-01T00:00:00Z + 365 days. Only relevant at
// decrypt time (parse does not check expiry), kept fixed for determinism.
const FIXTURE_START_TIMESTAMP = 1782864000 - 365 * 86400;
const FIXTURE_DURATION_DAYS = 365;

const V1_SELF_TYPES = {
  EIP712Domain: [
    { name: 'name', type: 'string' },
    { name: 'version', type: 'string' },
    { name: 'chainId', type: 'uint256' },
    { name: 'verifyingContract', type: 'address' },
  ],
  UserDecryptRequestVerification: [
    { name: 'publicKey', type: 'bytes' },
    { name: 'contractAddresses', type: 'address[]' },
    { name: 'startTimestamp', type: 'uint256' },
    { name: 'durationDays', type: 'uint256' },
    { name: 'extraData', type: 'bytes' },
  ],
} as const;

const V1_DELEGATED_TYPES = {
  EIP712Domain: [
    { name: 'name', type: 'string' },
    { name: 'version', type: 'string' },
    { name: 'chainId', type: 'uint256' },
    { name: 'verifyingContract', type: 'address' },
  ],
  DelegatedUserDecryptRequestVerification: [
    { name: 'publicKey', type: 'bytes' },
    { name: 'contractAddresses', type: 'address[]' },
    { name: 'delegatorAddress', type: 'address' },
    { name: 'startTimestamp', type: 'uint256' },
    { name: 'durationDays', type: 'uint256' },
    { name: 'extraData', type: 'bytes' },
  ],
} as const;

type SerializedPermit = {
  readonly version: number;
  readonly eip712: {
    readonly domain: Record<string, unknown>;
    readonly primaryType?: string | undefined;
    readonly types: Record<string, ReadonlyArray<{ readonly name: string; readonly type: string }>>;
    readonly message: Record<string, unknown>;
  };
  readonly signature: string;
  readonly signerAddress: string;
};

type PermitFixtureFile = {
  readonly _comment: string;
  readonly generatedBySdkVersion: string;
  readonly chainName: string;
  readonly transportKeyPair: { readonly publicKey: string; readonly privateKey: string };
  readonly permits: Record<string, SerializedPermit>;
};

export function defineClientDecryptPermitFixturesTests(parameters: {
  readonly runIf: boolean;
  readonly createFhevmDecryptClient: CreateViemDecryptClientFn;
}): void {
  const generateMode = process.env.GENERATE_PERMIT_FIXTURES === '1';
  const chainName = getViemTestConfig().chainName;
  const fixturePath = join(FIXTURES_DIR, `${chainName}.json`);
  const fixtureExists = existsSync(fixturePath);

  if (parameters.runIf && !fixtureExists && !generateMode) {
    console.log(
      `  [permitFixtures] no fixture for '${chainName}' (${fixturePath}) — ` +
        `generate one with GENERATE_PERMIT_FIXTURES=1 while the chain's stack is running`,
    );
  }

  describe.runIf(parameters.runIf && (fixtureExists || generateMode))(
    'Decrypt client — golden serialized-permit fixtures',
    () => {
      let config: FheTestViemConfig;

      beforeAll(() => {
        config = getViemTestConfig();
        setFhevmRuntimeConfig({
          auth: {
            type: 'ApiKeyHeader',
            value: config.zamaApiKey,
          },
          logger: createLogger(console.log),
        });
      });

      async function createReadyClient() {
        const client = parameters.createFhevmDecryptClient({
          chain: config.fhevmChain,
          publicClient: config.publicClient,
        });
        await client.ready;
        return client;
      }

      /** Signs a deterministic old-era V1 permit and returns it in serialized form. */
      async function buildFixturePermit(
        client: Awaited<ReturnType<typeof createReadyClient>>,
        transportKeyPair: Awaited<ReturnType<(typeof client)['generateTransportKeyPair']>>,
        opts: { readonly extraData: Hex; readonly delegatorAddress?: Hex },
      ): Promise<SerializedPermit> {
        const account = opts.delegatorAddress !== undefined ? config.bob.account : config.account;
        const domain = {
          name: 'Decryption',
          version: '1',
          chainId: BigInt(config.fhevmChain.id),
          verifyingContract: getAddress(config.fhevmChain.fhevm.gateway.contracts.decryption.address as Hex),
        };
        const publicKey = (transportKeyPair as { readonly publicKey: Hex }).publicKey;
        const contractAddresses = [getAddress(config.fheTestAddress as Hex)];
        const isDelegated = opts.delegatorAddress !== undefined;

        const message = {
          publicKey,
          contractAddresses,
          ...(opts.delegatorAddress !== undefined ? { delegatorAddress: getAddress(opts.delegatorAddress) } : {}),
          startTimestamp: String(FIXTURE_START_TIMESTAMP),
          durationDays: String(FIXTURE_DURATION_DAYS),
          extraData: opts.extraData,
        };

        const signature = await account.signTypedData!({
          domain,
          types: isDelegated
            ? {
                DelegatedUserDecryptRequestVerification: [
                  ...V1_DELEGATED_TYPES.DelegatedUserDecryptRequestVerification,
                ],
              }
            : { UserDecryptRequestVerification: [...V1_SELF_TYPES.UserDecryptRequestVerification] },
          primaryType: isDelegated ? 'DelegatedUserDecryptRequestVerification' : 'UserDecryptRequestVerification',
          message: {
            ...message,
            contractAddresses,
            startTimestamp: BigInt(FIXTURE_START_TIMESTAMP),
            durationDays: BigInt(FIXTURE_DURATION_DAYS),
          },
        });

        // Route through the SDK's canonical parse + serialize so the fixture
        // freezes exactly what serializeSignedDecryptionPermit produces.
        const parsed = await client.parseSignedDecryptionPermit({
          serializedPermit: {
            version: 1,
            eip712: {
              domain,
              types: (isDelegated ? V1_DELEGATED_TYPES : V1_SELF_TYPES) as unknown as Record<
                string,
                ReadonlyArray<{ readonly name: string; readonly type: string }>
              >,
              primaryType: isDelegated ? 'DelegatedUserDecryptRequestVerification' : 'UserDecryptRequestVerification',
              message,
            },
            signature,
            signerAddress: account.address,
          },
          transportKeyPair: transportKeyPair as Parameters<
            typeof client.parseSignedDecryptionPermit
          >[0]['transportKeyPair'],
        });

        return (await client.serializeSignedDecryptionPermit({ signedPermit: parsed })) as SerializedPermit;
      }

      it(
        generateMode && !fixtureExists ? 'generates the fixture file for this chain' : 'fixture file exists',
        async () => {
          if (fixtureExists || !generateMode) {
            expect(fixtureExists).toBe(true);
            return;
          }

          const client = await createReadyClient();
          const transportKeyPair = await client.generateTransportKeyPair();

          const fixture: PermitFixtureFile = {
            _comment:
              'Golden serialized-permit fixtures. Frozen output of serializeSignedDecryptionPermit — ' +
              'every later SDK version must keep parsing these (dApps cache permits for up to 365 days). ' +
              'Signed with the public test mnemonic; contains no real key material. Do not regenerate ' +
              'unless the chain deployment itself changed; regenerating hides serialization regressions.',
            generatedBySdkVersion: readSdkVersion(),
            chainName,
            transportKeyPair: (await serializeTransportKeyPair(client, {
              transportKeyPair,
            })) as PermitFixtureFile['transportKeyPair'],
            permits: {
              selfV1ExtraDataV0: await buildFixturePermit(client, transportKeyPair, { extraData: '0x00' as Hex }),
              selfV1ExtraDataV1: await buildFixturePermit(client, transportKeyPair, {
                extraData: `0x01${1n.toString(16).padStart(64, '0')}` as Hex,
              }),
              delegatedV1ExtraDataV0: await buildFixturePermit(client, transportKeyPair, {
                extraData: '0x00' as Hex,
                delegatorAddress: config.alice.account.address,
              }),
            },
          };

          mkdirSync(dirname(fixturePath), { recursive: true });
          writeFileSync(fixturePath, `${JSON.stringify(fixture, null, 2)}\n`);
          console.log(`  [permitFixtures] wrote ${fixturePath}`);
          expect(existsSync(fixturePath)).toBe(true);
        },
      );

      it.runIf(fixtureExists)('parses every committed permit fixture (frozen serialization format)', async () => {
        const fixture = JSON.parse(readFileSync(fixturePath, 'utf8')) as PermitFixtureFile;
        expect(fixture.chainName).toBe(chainName);

        const client = await createReadyClient();
        const transportKeyPair = await parseTransportKeyPair(client, fixture.transportKeyPair);

        for (const [name, serialized] of Object.entries(fixture.permits)) {
          const restored = await client.parseSignedDecryptionPermit({
            serializedPermit: serialized,
            transportKeyPair,
          });
          expect(restored.version, name).toBe(serialized.version);
          expect(restored.signerAddress.toLowerCase(), name).toBe(serialized.signerAddress.toLowerCase());
          expect((restored.eip712.message as { extraData: string }).extraData, name).toBe(
            (serialized.eip712.message as { extraData: string }).extraData,
          );
          expect(restored.isDelegated, name).toBe(name.startsWith('delegated'));
          console.log(`  [permitFixtures] parsed ${name} ✔`);
        }
      });

      it.runIf(fixtureExists)('parses every fixture after a JSON string round-trip (localStorage flow)', async () => {
        const fixture = JSON.parse(readFileSync(fixturePath, 'utf8')) as PermitFixtureFile;

        const client = await createReadyClient();
        const transportKeyPair = await parseTransportKeyPair(
          client,
          JSON.parse(JSON.stringify(fixture.transportKeyPair)) as PermitFixtureFile['transportKeyPair'],
        );

        for (const [name, serialized] of Object.entries(fixture.permits)) {
          const revived = JSON.parse(JSON.stringify(serialized)) as SerializedPermit;
          const restored = await client.parseSignedDecryptionPermit({
            serializedPermit: revived,
            transportKeyPair,
          });
          expect(restored.version, name).toBe(serialized.version);
        }
      });
    },
  );
}
