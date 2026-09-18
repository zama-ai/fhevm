import { expect, test } from 'bun:test';

const { selectImages } = require('../../../ci/preview-env/scripts/create-e2e-inputs.cjs') as {
  selectImages(input: {
    buildResults: Record<string, { result?: string; outputs: Record<string, string> }>;
    headTag: string;
    gpuWorkerTag?: string;
  }): {
    outputs: Record<string, string>;
    built: { repo: string; tag: string }[];
    skipped: { repo: string; envKey: string }[];
  };
};

// Enumerate the build-job contract independently of the production mapping.
function successfulBuilds() {
  const jobs: Record<string, string[]> = {
    'coprocessor-docker-build': [
      'db_migration',
      'gw_listener',
      'host_listener',
      'sns_worker',
      'tfhe_worker',
      'tx_sender',
      'zkproof_worker',
      'consensus_detector',
      'upgrade_controller',
    ],
    'kms-connector-docker-build': ['db_migration', 'gw_listener', 'kms_worker', 'tx_sender', 'endpoint', 'proxy'],
    'relayer-docker-build': ['relayer_migrate', 'relayer'],
    'listener-docker-build': [''],
    'gateway-contracts-docker-build': [''],
    'host-contracts-docker-build': [''],
    'test-suite-docker-build': [''],
    'gpu-worker-build': [],
  };
  return Object.fromEntries(
    Object.entries(jobs).map(([job, services]) => [
      job,
      {
        result: 'success',
        outputs: Object.fromEntries(
          services.map((service) => [`${service ? `${service}_` : ''}build_result`, 'success']),
        ),
      },
    ]),
  );
}

for (const gpuWorkerTag of [undefined, 'abcdef0-cuda12.8-sm90']) {
  test(`connector tags and registry checks are selected (GPU=${!!gpuWorkerTag})`, () => {
    const selected = selectImages({ buildResults: successfulBuilds(), headTag: 'abcdef0', gpuWorkerTag });
    expect(JSON.parse(selected.outputs['connector-versions'])).toEqual({
      'db-migration': 'abcdef0',
      'gw-listener': 'abcdef0',
      'kms-worker': 'abcdef0',
      'tx-sender': 'abcdef0',
      endpoint: 'abcdef0',
      proxy: 'abcdef0',
    });
    expect(selected.built).toContainEqual({ repo: 'fhevm/kms-connector/endpoint', tag: 'abcdef0' });
    expect(selected.built).toContainEqual({ repo: 'fhevm/kms-connector/proxy', tag: 'abcdef0' });
  });

  for (const service of ['endpoint', 'proxy'] as const) {
    const envKey = `CONNECTOR_${service.toUpperCase()}_VERSION`;

    for (const result of ['failure', 'cancelled', 'missing']) {
      test(`${service} ${result} rejects selection (GPU=${!!gpuWorkerTag})`, () => {
        const builds = successfulBuilds();
        if (result === 'missing') delete builds['kms-connector-docker-build'].outputs[`${service}_build_result`];
        else builds['kms-connector-docker-build'].outputs[`${service}_build_result`] = result;
        expect(() => selectImages({ buildResults: builds, headTag: 'abcdef0', gpuWorkerTag })).toThrow(
          `${service}_build_result`,
        );
      });
    }

    test(`skipped ${service} is left to the verified baseline (GPU=${!!gpuWorkerTag})`, () => {
      const builds = successfulBuilds();
      builds['kms-connector-docker-build'].outputs[`${service}_build_result`] = 'skipped';
      const selected = selectImages({ buildResults: builds, headTag: 'abcdef0', gpuWorkerTag });
      expect(JSON.parse(selected.outputs['connector-versions'])[service]).toBe('');
      expect(selected.skipped).toContainEqual({ repo: `fhevm/kms-connector/${service}`, envKey });
      expect(selected.built.some(({ repo }) => repo === `fhevm/kms-connector/${service}`)).toBe(false);
    });
  }
}

test('GPU override selects exact worker tags without requiring CPU worker builds', () => {
  const builds = successfulBuilds();
  for (const worker of ['tfhe', 'sns', 'zkproof'])
    delete builds['coprocessor-docker-build'].outputs[`${worker}_worker_build_result`];
  const selected = selectImages({ buildResults: builds, headTag: 'abcdef0', gpuWorkerTag: 'abcdef0-cuda12.8-sm90' });
  for (const worker of ['tfhe', 'sns', 'zkproof']) {
    expect(selected.outputs[`coprocessor-${worker}-worker-version`]).toBe('abcdef0-cuda12.8-sm90');
    expect(selected.built).toContainEqual({ repo: `fhevm/coprocessor/${worker}-worker`, tag: 'abcdef0-cuda12.8-sm90' });
  }
  expect(selected.outputs['coprocessor-host-listener-version']).toBe('abcdef0');
  expect(() => selectImages({ buildResults: builds, headTag: 'abcdef0' })).toThrow('worker_build_result');
});

test('empty GPU tag cannot fall back to CPU or baseline workers', () => {
  expect(() => selectImages({ buildResults: successfulBuilds(), headTag: 'abcdef0', gpuWorkerTag: '' })).toThrow(
    'no tag',
  );
});

for (const job of ['gpu-worker-build', 'coprocessor-docker-build', 'kms-connector-docker-build']) {
  for (const result of ['failure', 'cancelled', 'skipped', 'missing']) {
    test(`${job} ${result} rejects a populated GPU tag and successful child outputs`, () => {
      const builds = successfulBuilds();
      if (result === 'missing') delete builds[job];
      else builds[job].result = result;
      expect(() =>
        selectImages({ buildResults: builds, headTag: 'abcdef0', gpuWorkerTag: 'abcdef0-cuda12.8-sm90' }),
      ).toThrow(`${job}=${result}`);
    });
  }
}

test('failed parent cannot disguise a build-check failure as skipped unchanged images', () => {
  const builds = successfulBuilds();
  builds['kms-connector-docker-build'].result = 'failure';
  for (const key of Object.keys(builds['kms-connector-docker-build'].outputs)) {
    builds['kms-connector-docker-build'].outputs[key] = 'skipped';
  }
  expect(() => selectImages({ buildResults: builds, headTag: 'abcdef0' })).toThrow(
    'kms-connector-docker-build=failure',
  );
});

test('CPU selection does not require a GPU producer', () => {
  const builds = successfulBuilds();
  delete builds['gpu-worker-build'];
  expect(selectImages({ buildResults: builds, headTag: 'abcdef0' }).outputs['coprocessor-tfhe-worker-version']).toBe(
    'abcdef0',
  );
});
