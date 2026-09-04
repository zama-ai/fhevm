import { expect } from 'chai';
import { ethers } from 'ethers';

import {
  type CanonicalOutputRow,
  type CiphertextAttestationMetadata,
  type ConsensusDatabaseReport,
  type TransactionCompletionRow,
  type TransactionScope,
  assertAttestationMatchesOutputEvidence,
  assertCanonicalOutputDigestBindings,
  assertConsensusEventBindings,
  assertEquivalentCanonicalOutputs,
  assertEquivalentCompletedTransactions,
  attestationEvidenceFromCanonicalOutput,
  attestationMetadataFromWgetHeaders,
  containerName,
  kmsNamespaceAttestationHeadArgs,
  rfc023AttestationPrehash,
  rfc023CiphertextUrl,
  transactionScopesFromOutputs,
} from './helpers';

const handle = Buffer.alloc(32, 0x11);
const transactionId = Buffer.alloc(32, 0x22);
const keyId = Buffer.alloc(32, 0x66);
const attestationSigner = new ethers.Wallet(`0x${'44'.repeat(32)}`);

function digest(value: Buffer): Buffer {
  return Buffer.from(ethers.getBytes(ethers.keccak256(value)));
}

const output = (overrides: Partial<CanonicalOutputRow> = {}): CanonicalOutputRow => ({
  handle,
  ciphertext: Buffer.from('materialized-ciphertext'),
  snsCiphertext: null,
  ciphertextType: 2,
  ciphertextVersion: 0,
  fheOperation: 0,
  transactionId,
  hostChainId: 12345,
  blockNumber: 7,
  keyId,
  ciphertextDigest: digest(Buffer.from('materialized-ciphertext')),
  snsCiphertextDigest: Buffer.alloc(32, 0x55),
  ciphertext128Format: 11,
  ...overrides,
});

const completion = (overrides: Partial<TransactionCompletionRow> = {}): TransactionCompletionRow => ({
  transactionId,
  hostChainId: 12345,
  blockNumber: 7,
  totalCount: 3,
  completedCount: 3,
  errorCount: 0,
  ...overrides,
});

const scope: TransactionScope = {
  transactionId,
  hostChainId: 12345,
  blockNumber: 7,
};

function signedAttestation(
  evidence = attestationEvidenceFromCanonicalOutput(output()),
  contextId = 1n,
): CiphertextAttestationMetadata {
  const unsigned = {
    version: 1 as const,
    keyId: evidence.keyId,
    ciphertextDigest: evidence.ciphertextDigest,
    snsCiphertextDigest: evidence.snsCiphertextDigest,
    format: 'compressed_on_cpu' as const,
  };
  return {
    ...unsigned,
    signer: attestationSigner.address,
    signature: attestationSigner.signingKey.sign(rfc023AttestationPrehash(unsigned, evidence.handle, contextId))
      .serialized,
  };
}

function attestationHeaders(metadata: CiphertextAttestationMetadata): string {
  return [
    'HTTP/1.1 200 OK',
    `x-amz-meta-ct-attestation: ${JSON.stringify({
      version: metadata.version,
      key_id: metadata.keyId,
      ciphertext_digest: metadata.ciphertextDigest,
      sns_ciphertext_digest: metadata.snsCiphertextDigest,
      format: metadata.format,
      signer: metadata.signer,
      signature: metadata.signature,
    })}`,
  ].join('\n');
}

describe('Materialization consensus harness helpers', () => {
  const expectedHandle = `0x${handle.toString('hex')}`;

  it('requires exact ciphertext and provenance equality across homogeneous databases', () => {
    expect(() =>
      assertEquivalentCanonicalOutputs([[output()], [output()], [output()]], [expectedHandle]),
    ).to.not.throw();
    expect(() =>
      assertEquivalentCanonicalOutputs(
        [
          [output()],
          [
            output({
              ciphertext: Buffer.from('different'),
              ciphertextDigest: digest(Buffer.from('different')),
            }),
          ],
        ],
        [expectedHandle],
      ),
    ).to.throw('same-SW/same-backend consensus requires exact ciphertext and provenance equality');
    expect(() =>
      assertEquivalentCanonicalOutputs(
        [[output()], [output({ transactionId: Buffer.alloc(32, 0x99) })]],
        [expectedHandle],
      ),
    ).to.throw('same-SW/same-backend consensus requires exact ciphertext and provenance equality');
    expect(() =>
      assertEquivalentCanonicalOutputs([[output()], [output({ ciphertext128Format: 10 })]], [expectedHandle]),
    ).to.throw('same-SW/same-backend consensus requires exact ciphertext and provenance equality');
  });

  it('does not accept a raw output that has not reached the digest/publication boundary', () => {
    expect(() =>
      assertEquivalentCanonicalOutputs(
        [[output({ ciphertextDigest: null })], [output({ ciphertextDigest: null })]],
        [expectedHandle],
      ),
    ).to.throw('is not publishable: digest missing');
    expect(() => assertCanonicalOutputDigestBindings([[output({ ciphertext128Format: 0 })]])).to.throw(
      'unsupported ciphertext_digest.ciphertext128_format 0',
    );
  });

  it('binds each durable ciphertext digest to Keccak256 of its raw TFHE bytes', () => {
    expect(() =>
      assertCanonicalOutputDigestBindings([[output({ ciphertextDigest: Buffer.alloc(32, 0x99) })]]),
    ).to.throw('Keccak256(raw ciphertext) does not match ciphertext_digest.ciphertext');
  });

  it('checks a retained SNS ciphertext against its digest when that raw value is available', () => {
    const snsCiphertext = Buffer.from('sns-ciphertext');
    expect(() =>
      assertCanonicalOutputDigestBindings([[output({ snsCiphertext, snsCiphertextDigest: Buffer.alloc(32, 0x99) })]]),
    ).to.throw('Keccak256(raw SNS ciphertext) does not match ciphertext_digest.ciphertext128');
  });

  it('binds exactly one Gateway consensus event per canonical handle', () => {
    const report: ConsensusDatabaseReport = {
      databaseUrl: 'postgresql://example.invalid/coprocessor',
      outputs: [output()],
      transactions: [],
    };
    const event = {
      ctHandle: expectedHandle,
      keyId: BigInt(`0x${keyId.toString('hex')}`),
      ciphertextDigest: `0x${output().ciphertextDigest!.toString('hex')}`,
      snsCiphertextDigest: `0x${output().snsCiphertextDigest!.toString('hex')}`,
      senders: [],
      blockNumber: 7,
    };
    expect(() => assertConsensusEventBindings([report], [event])).to.not.throw();
    expect(() =>
      assertConsensusEventBindings([report], [{ ...event, ciphertextDigest: `0x${'ff'.repeat(32)}` }]),
    ).to.throw('does not bind the canonical key/digests');
    expect(() => assertConsensusEventBindings([report], [event, event])).to.throw(
      'Gateway emitted duplicate AddCiphertextMaterialConsensus events',
    );
  });

  it('derives one provenance scope per producing transaction', () => {
    expect(transactionScopesFromOutputs([output(), output({ handle: Buffer.alloc(32, 0x12) })], 0)).to.have.length(1);
    expect(() =>
      transactionScopesFromOutputs([output(), output({ handle: Buffer.alloc(32, 0x12), blockNumber: 8 })], 0),
    ).to.throw('multiple canonical provenance scopes');
  });

  it('retries incomplete transactions and fails hard on errored or mismatched ones', () => {
    expect(() => assertEquivalentCompletedTransactions([[completion()], [completion()]], [scope])).to.not.throw();
    expect(() =>
      assertEquivalentCompletedTransactions([[completion()], [completion({ completedCount: 2 })]], [scope]),
    ).to.throw('has completed 2 of 3 computations');
    expect(() =>
      assertEquivalentCompletedTransactions([[completion()], [completion({ errorCount: 1 })]], [scope]),
    ).to.throw('errored computations');
    expect(() =>
      assertEquivalentCompletedTransactions(
        [[completion()], [completion({ totalCount: 4, completedCount: 4 })]],
        [scope],
      ),
    ).to.throw('transaction completion mismatch');
    expect(() => assertEquivalentCompletedTransactions([[completion()], []], [scope])).to.throw(
      'is missing transaction completion',
    );
  });

  it('uses the generated compose naming convention for service controls', () => {
    expect(containerName(0, 'tfhe-worker')).to.equal('coprocessor-tfhe-worker');
    expect(containerName(2, 'tfhe-worker')).to.equal('coprocessor2-tfhe-worker');
  });

  it('probes the exact RFC-023 object in the KMS worker network namespace without a shell', () => {
    const url = rfc023CiphertextUrl('http://minio:9000/coproc-0-ct128', expectedHandle);
    expect(url).to.equal(`http://minio:9000/coproc-0-ct128/${handle.toString('hex')}/1`);
    expect(kmsNamespaceAttestationHeadArgs('kms-connector-kms-worker', 'probe-image', url)).to.deep.equal([
      'run',
      '--rm',
      '--network',
      'container:kms-connector-kms-worker',
      '--entrypoint',
      '/usr/bin/wget',
      'probe-image',
      '--server-response',
      '--spider',
      '--timeout=5',
      url,
    ]);
  });

  it('rejects a non-RFC-023-safe Coprocessor bucket URL before opening a Docker probe', () => {
    expect(() => rfc023CiphertextUrl('ftp://minio:9000/coproc-0-ct128', expectedHandle)).to.throw(
      'unsupported Coprocessor bucket URL',
    );
    expect(() => rfc023CiphertextUrl('http://minio:9000/coproc-0-ct128?wrong=1', expectedHandle)).to.throw(
      'unsupported Coprocessor bucket URL',
    );
  });

  it('does not normalize the registered bucket string away from the connector request target', () => {
    expect(rfc023CiphertextUrl('http://minio:9000/coproc-0-ct128/', expectedHandle)).to.equal(
      `http://minio:9000/coproc-0-ct128//${handle.toString('hex')}/1`,
    );
  });

  it('requires an RFC-023 attestation to bind the exact terminal output, context, and registered signer', () => {
    const evidence = attestationEvidenceFromCanonicalOutput(output());
    const metadata = signedAttestation(evidence);
    const bucket = {
      txSender: '0x00000000000000000000000000000000000000a1',
      signer: attestationSigner.address,
      bucketUrl: 'http://minio:9000/coproc-0-ct128',
    };
    expect(() => assertAttestationMatchesOutputEvidence(metadata, evidence, bucket)).to.not.throw();
    expect(attestationMetadataFromWgetHeaders(attestationHeaders(metadata))).to.deep.equal(metadata);
    const minimalU256Headers = attestationHeaders({
      ...metadata,
      // Rust's U256 JSON serializer legitimately removes this leading zero.
      // The RFC payload remains 32-byte padded, so parser normalization must
      // preserve signature verification and the canonical DB comparison.
      keyId: metadata.keyId.replace(/^0x0/, '0x'),
    });
    expect(attestationMetadataFromWgetHeaders(minimalU256Headers).keyId).to.equal(metadata.keyId);

    expect(() =>
      assertAttestationMatchesOutputEvidence(
        { ...metadata, ciphertextDigest: `0x${'55'.repeat(32)}` },
        evidence,
        bucket,
      ),
    ).to.throw('ciphertext_digest does not match canonical output');
    expect(() =>
      assertAttestationMatchesOutputEvidence({ ...metadata, keyId: `0x${'77'.repeat(32)}` }, evidence, bucket),
    ).to.throw('key_id');
    expect(() =>
      assertAttestationMatchesOutputEvidence(
        { ...metadata, snsCiphertextDigest: `0x${'88'.repeat(32)}` },
        evidence,
        bucket,
      ),
    ).to.throw('sns_ciphertext_digest does not match canonical output');
    expect(() =>
      assertAttestationMatchesOutputEvidence({ ...metadata, format: 'uncompressed_on_cpu' }, evidence, bucket),
    ).to.throw('does not match canonical ciphertext_digest.ciphertext128_format');
    expect(() =>
      assertAttestationMatchesOutputEvidence(metadata, evidence, { ...bucket, signer: ethers.ZeroAddress }),
    ).to.throw('does not match registered Coprocessor');
    expect(() => assertAttestationMatchesOutputEvidence(signedAttestation(evidence, 0n), evidence, bucket)).to.throw(
      'not advertised signer',
    );
    expect(() =>
      assertAttestationMatchesOutputEvidence(
        signedAttestation({ ...evidence, handle: ethers.ZeroHash }),
        evidence,
        bucket,
      ),
    ).to.throw('not advertised signer');
  });

  it('uses the RFC-023 V1 packed payload pinned by the Rust implementation', () => {
    expect(
      rfc023AttestationPrehash(
        {
          version: 1,
          keyId: ethers.toBeHex(7n, 32),
          ciphertextDigest: `0x${'11'.repeat(32)}`,
          snsCiphertextDigest: `0x${'22'.repeat(32)}`,
          format: 'uncompressed_on_cpu',
        },
        `0x${'aa'.repeat(32)}`,
        0n,
      ),
    ).to.equal('0x97f99a874a16f680d5c6b60b4cca7356877a78ce59f49872ad21030ebe6e0dd8');
  });

  it('rejects malformed or duplicate attestation response headers', () => {
    expect(() => attestationMetadataFromWgetHeaders('HTTP/1.1 200 OK\nx-amz-meta-ct-attestation: not-json')).to.throw(
      'not valid JSON',
    );
    const headers = attestationHeaders(signedAttestation());
    expect(() => attestationMetadataFromWgetHeaders(`${headers}\n${headers.split('\n')[1]}`)).to.throw(
      'expected exactly one',
    );
    expect(() =>
      attestationMetadataFromWgetHeaders(
        headers.replace('"format":"compressed_on_cpu"', '"format":"compressed_on_cpu","format":"uncompressed_on_cpu"'),
      ),
    ).to.throw('duplicate object member "format"');
    expect(() =>
      attestationMetadataFromWgetHeaders(
        headers.replace(
          '"format":"compressed_on_cpu"',
          '"format":"compressed_on_cpu","for\\u006dat":"uncompressed_on_cpu"',
        ),
      ),
    ).to.throw('duplicate object member "format"');
    expect(() => attestationMetadataFromWgetHeaders(headers.replace('"compressed_on_cpu"', '"toString"'))).to.throw(
      'unsupported attestation format',
    );
  });
});
