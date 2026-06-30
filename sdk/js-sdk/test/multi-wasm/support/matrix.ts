import type { TkmsVersion } from '../../../src/wasm/tkms/loadKmsLib.js';
import type { TfheVersion } from '../../../src/wasm/tfhe/loadTfheLib.js';
import type { WasmAssetLoadMode } from '../../../src/core/types/wasmAssets.js';
import { existsSync, readFileSync } from 'node:fs';
import { resolve } from 'node:path';

export const WASM_ASSET_LOAD_MODES: readonly WasmAssetLoadMode[] = [
  'auto',
  'embedded-base64',
  'verified-blob',
  'precheck-direct-url',
  'trusted-direct-url',
] as const;

export const LOCAL_CDN = 'local' as const;

export type MultiWasmCdn = typeof LOCAL_CDN | string;

export type MultiWasmRoundTrip = {
  readonly clearType: 'bool' | 'uint8' | 'uint16' | 'uint32' | 'uint64' | 'uint128' | 'uint256' | 'address';
  readonly contractMethod:
    | 'setEbool'
    | 'setEuint8'
    | 'setEuint16'
    | 'setEuint32'
    | 'setEuint64'
    | 'setEuint128'
    | 'setEuint256'
    | 'setEaddress';
  readonly makePublic: boolean;
  readonly value: boolean | number | string;
};

export type MultiWasmAssetUrlSet = {
  readonly tfheWasm: string;
  readonly tfheWorker: string;
  readonly kmsWasm: string;
};

export type MultiWasmVersionPair = {
  readonly tfhe: TfheVersion;
  readonly kms: TkmsVersion;
  readonly cdns?: readonly string[];
};

export type MultiWasmMatrix = {
  readonly schemaVersion: 1;
  readonly defaults: {
    readonly roundTrip: MultiWasmRoundTrip;
  };
  readonly supportedVersionPairs: readonly MultiWasmVersionPair[];
  readonly assetUrlSets: Record<string, MultiWasmAssetUrlSet>;
};

export type MultiWasmMatrixEntry = {
  readonly id: string;
  readonly label: string;
  readonly versionPair: MultiWasmVersionPair;
  readonly wasmAssetLoadMode: WasmAssetLoadMode;
  readonly cdn: MultiWasmCdn;
};

export type MultiWasmMatrixSelection = {
  readonly tfhe?: string | undefined;
  readonly kms?: string | undefined;
  readonly mode?: string | undefined;
  readonly cdn?: string | undefined;
};

export const matrixPath = resolve(import.meta.dirname, '../matrix.json');

export function loadMatrix(): MultiWasmMatrix {
  if (!existsSync(matrixPath)) {
    throw new Error(`Missing multi-WASM matrix: ${matrixPath}`);
  }

  const matrix = JSON.parse(readFileSync(matrixPath, 'utf-8')) as MultiWasmMatrix;
  validateMatrix(matrix);
  return matrix;
}

export function generateMatrixEntries(matrix: MultiWasmMatrix): readonly MultiWasmMatrixEntry[] {
  const allCdns = Object.keys(matrix.assetUrlSets);
  if (!allCdns.includes(LOCAL_CDN)) {
    throw new Error(`Multi-WASM matrix assetUrlSets must include "${LOCAL_CDN}".`);
  }

  const entries: MultiWasmMatrixEntry[] = [];

  for (const versionPair of matrix.supportedVersionPairs) {
    const pairCdns = versionPair.cdns ?? allCdns;

    for (const mode of WASM_ASSET_LOAD_MODES) {
      if (mode === 'embedded-base64') {
        if (pairCdns.includes(LOCAL_CDN)) {
          entries.push(makeEntry(versionPair, mode, LOCAL_CDN));
        }
        continue;
      }

      for (const cdn of pairCdns) {
        entries.push(makeEntry(versionPair, mode, cdn));
      }
    }
  }

  return entries;
}

export function selectMatrixEntries(
  matrix: MultiWasmMatrix,
  selection: MultiWasmMatrixSelection,
): readonly MultiWasmMatrixEntry[] {
  const tfhe = selection.tfhe === undefined ? undefined : normalizeModuleVersion(selection.tfhe);
  const kms = selection.kms === undefined ? undefined : normalizeModuleVersion(selection.kms);
  const mode = selection.mode;
  const cdn = selection.cdn;

  if (mode !== undefined && !WASM_ASSET_LOAD_MODES.includes(mode as WasmAssetLoadMode)) {
    throw new Error(`Unknown --mode "${mode}". Supported modes: ${WASM_ASSET_LOAD_MODES.join(', ')}.`);
  }

  if (cdn !== undefined && matrix.assetUrlSets[cdn] === undefined) {
    throw new Error(`Unknown --cdn "${cdn}". Supported CDNs: ${Object.keys(matrix.assetUrlSets).join(', ')}.`);
  }

  const entries = generateMatrixEntries(matrix).filter((entry) => {
    if (tfhe !== undefined && entry.versionPair.tfhe !== tfhe) {
      return false;
    }
    if (kms !== undefined && entry.versionPair.kms !== kms) {
      return false;
    }
    if (mode !== undefined && entry.wasmAssetLoadMode !== mode) {
      return false;
    }
    if (cdn !== undefined && entry.cdn !== cdn) {
      return false;
    }
    return true;
  });

  if (entries.length === 0) {
    throw new Error(
      [
        `No multi-WASM matrix entry matches selection: ${formatSelection(selection)}.`,
        'Available entries:',
        ...generateMatrixEntries(matrix).map((entry) => `  ${entry.id}`),
      ].join('\n'),
    );
  }

  return entries;
}

export function resolveEntryAssetUrls(matrix: MultiWasmMatrix, entry: MultiWasmMatrixEntry): MultiWasmAssetUrlSet {
  const template = matrix.assetUrlSets[entry.cdn];
  if (template === undefined) {
    throw new Error(`Unknown assetUrlSet "${entry.cdn}" for entry ${entry.id}`);
  }

  return {
    tfheWasm: renderAssetUrlTemplate(template.tfheWasm, entry.versionPair),
    tfheWorker: renderAssetUrlTemplate(template.tfheWorker, entry.versionPair),
    kmsWasm: renderAssetUrlTemplate(template.kmsWasm, entry.versionPair),
  };
}

export function normalizeModuleVersion(version: string): string {
  return version.trim().replace(/^v/i, '');
}

function makeEntry(
  versionPair: MultiWasmVersionPair,
  mode: WasmAssetLoadMode,
  cdn: MultiWasmCdn,
): MultiWasmMatrixEntry {
  const id = `tfhe-${versionPair.tfhe}__kms-${versionPair.kms}__${mode}__${cdn}`;
  const label = `TFHE ${versionPair.tfhe} + TKMS ${versionPair.kms} / ${mode} / ${cdn}`;
  return { id, label, versionPair, wasmAssetLoadMode: mode, cdn };
}

function renderAssetUrlTemplate(template: string, versions: MultiWasmVersionPair): string {
  return template.replace(/\{tfhe\}/g, versions.tfhe).replace(/\{kms\}/g, versions.kms);
}

function formatSelection(selection: MultiWasmMatrixSelection): string {
  const parts: string[] = [];
  if (selection.tfhe !== undefined) {
    parts.push(`--tfhe ${selection.tfhe}`);
  }
  if (selection.kms !== undefined) {
    parts.push(`--kms ${selection.kms}`);
  }
  if (selection.mode !== undefined) {
    parts.push(`--mode ${selection.mode}`);
  }
  if (selection.cdn !== undefined) {
    parts.push(`--cdn ${selection.cdn}`);
  }
  return parts.length === 0 ? '(none)' : parts.join(' ');
}

function validateMatrix(matrix: MultiWasmMatrix): void {
  if (matrix.schemaVersion !== 1) {
    throw new Error(`Unsupported multi-WASM matrix schemaVersion: ${matrix.schemaVersion}`);
  }

  if (!Array.isArray(matrix.supportedVersionPairs) || matrix.supportedVersionPairs.length === 0) {
    throw new Error('Multi-WASM matrix must declare at least one supportedVersionPairs entry.');
  }

  validateAssetUrlSets(matrix.assetUrlSets);

  const cdnNames = Object.keys(matrix.assetUrlSets);
  const seenPairs = new Set<string>();
  for (const pair of matrix.supportedVersionPairs) {
    if (typeof pair.tfhe !== 'string' || pair.tfhe === '') {
      throw new Error('Multi-WASM matrix versionPair is missing tfhe.');
    }
    if (typeof pair.kms !== 'string' || pair.kms === '') {
      throw new Error('Multi-WASM matrix versionPair is missing kms.');
    }
    if (pair.cdns !== undefined) {
      if (!Array.isArray(pair.cdns) || pair.cdns.length === 0) {
        throw new Error(`Multi-WASM matrix versionPair tfhe=${pair.tfhe} kms=${pair.kms} has empty "cdns" allow-list.`);
      }
      for (const cdn of pair.cdns) {
        if (!cdnNames.includes(cdn)) {
          throw new Error(
            `Multi-WASM matrix versionPair tfhe=${pair.tfhe} kms=${pair.kms} lists unknown cdn "${cdn}". Known: ${cdnNames.join(', ')}.`,
          );
        }
      }
    }
    const key = `${pair.tfhe}\0${pair.kms}`;
    if (seenPairs.has(key)) {
      throw new Error(`Duplicate multi-WASM matrix versionPair: tfhe=${pair.tfhe} kms=${pair.kms}`);
    }
    seenPairs.add(key);
  }
}

function validateAssetUrlSets(assetUrlSets: Record<string, MultiWasmAssetUrlSet>): void {
  if (assetUrlSets === null || assetUrlSets === undefined || typeof assetUrlSets !== 'object') {
    throw new Error('Multi-WASM matrix must define assetUrlSets.');
  }

  const names = Object.keys(assetUrlSets);
  if (names.length === 0) {
    throw new Error('Multi-WASM matrix assetUrlSets must contain at least one set.');
  }

  if (!names.includes(LOCAL_CDN)) {
    throw new Error(`Multi-WASM matrix assetUrlSets must include "${LOCAL_CDN}".`);
  }

  for (const name of names) {
    const assetUrlSet = assetUrlSets[name];
    if (assetUrlSet === undefined) {
      throw new Error(`Missing multi-WASM matrix assetUrlSet "${name}".`);
    }
    if (typeof assetUrlSet.tfheWasm !== 'string' || assetUrlSet.tfheWasm === '') {
      throw new Error(`Invalid tfheWasm in multi-WASM matrix assetUrlSet "${name}".`);
    }
    if (typeof assetUrlSet.tfheWorker !== 'string' || assetUrlSet.tfheWorker === '') {
      throw new Error(`Invalid tfheWorker in multi-WASM matrix assetUrlSet "${name}".`);
    }
    if (typeof assetUrlSet.kmsWasm !== 'string' || assetUrlSet.kmsWasm === '') {
      throw new Error(`Invalid kmsWasm in multi-WASM matrix assetUrlSet "${name}".`);
    }
  }
}
