import type {
  FetchFheEncryptionKeySourceParameters,
  FetchFheEncryptionKeySourceReturnType,
  RelayerClient,
} from '../types.js';
import type { FetchKeyUrlResult } from '../../../types/relayer-p.js';
import { setAuth } from '../../../base/auth.js';
import { RelayerFetchError } from '../../../errors/RelayerFetchError.js';
import { assertRecordArrayProperty, assertRecordNonNullableProperty } from '../../../base/record.js';
import { fetchWithRetry } from '../../../base/fetch.js';
import { sdkName, version } from '../../../_version.js';
import { assertRecordStringArrayProperty, assertRecordStringProperty, removeSuffix } from '../../../base/string.js';

////////////////////////////////////////////////////////////////////////////////
// fetchFheEncryptionKeySource
////////////////////////////////////////////////////////////////////////////////

export async function fetchFheEncryptionKeySource(
  relayerClient: RelayerClient,
  parameters: FetchFheEncryptionKeySourceParameters,
): Promise<FetchFheEncryptionKeySourceReturnType> {
  const { options } = parameters;
  const relayerUrl = relayerClient.relayerUrl;
  const init = setAuth(
    {
      method: 'GET',
      headers: {
        'ZAMA-SDK-VERSION': version,
        'ZAMA-SDK-NAME': sdkName,
      },
    } satisfies RequestInit,
    options?.auth,
  );

  const url = `${removeSuffix(relayerUrl, '/')}/v2/keyurl`;

  let response;
  try {
    response = await fetchWithRetry({
      url,
      init,
      retries: options?.fetchRetries,
      retryDelayMs: options?.fetchRetryDelayInMilliseconds,
    });
  } catch (cause) {
    if ((cause as { name: string }).name === 'AbortError') {
      throw cause;
    }
    _throwFetchError({
      url,
      message: `Fetch ${url} failed!`,
      cause,
    });
  }

  if (!response.ok) {
    _throwFetchError({
      url,
      message: `HTTP error! status: ${response.status} on ${response.url}`,
    });
  }

  let json;
  try {
    json = (await response.json()) as unknown;
  } catch (e) {
    _throwFetchError({
      url,
      message: 'JSON parsing failed.',
      cause: e,
    });
  }

  _assertIsRelayerFetchResponseJson(url, json);

  const result = json.response;
  _assertIsFetchKeyUrlResult(result, 'response');

  return {
    publicKeySource: {
      id: result.fheKeyInfo[0].fhePublicKey.dataId,
      url: result.fheKeyInfo[0].fhePublicKey.urls[0],
    },
    crsSource: {
      id: result.crs[2048].dataId,
      url: result.crs[2048].urls[0],
      capacity: 2048,
    },
    metadata: {
      relayerUrl,
      chainId: relayerClient.chainId,
    },
  };
}

////////////////////////////////////////////////////////////////////////////////

function _throwFetchError(params: { url: string; message: string; cause?: unknown }): never {
  throw new RelayerFetchError({
    url: params.url,
    fetchMethod: 'GET',
    operation: 'KEY_URL',
    retryCount: 0,
    elapsed: 0,
    message: params.message,
    cause: params.cause,
  });
}

////////////////////////////////////////////////////////////////////////////////
// Asserts
////////////////////////////////////////////////////////////////////////////////

function _assertIsRelayerFetchResponseJson(url: string, json: unknown): asserts json is { response: unknown } {
  if (json === undefined || json === null || typeof json !== 'object') {
    _throwFetchError({
      url,
      message: 'Unexpected response JSON.',
    });
  }
  if (!('response' in json && json.response !== null && json.response !== undefined)) {
    _throwFetchError({
      url,
      message: "Unexpected response JSON format: missing 'response' property.",
    });
  }
}

function _assertIsFetchKeyUrlResult(value: unknown, valueName: string): asserts value is FetchKeyUrlResult {
  // value.fheKeyInfo (array)
  assertRecordArrayProperty(value, 'fheKeyInfo', valueName, {});
  if (value.fheKeyInfo.length !== 1) {
    throw new Error(`Unexpected 'fheKeyInfo' array length.`);
  }

  const fheKeyInfo = value.fheKeyInfo[0];
  const fheKeyInfoName = `${valueName}.fheKeyInfo[0]`;

  // value.fheKeyInfo[0].fhePublicKey (record)
  assertRecordNonNullableProperty(fheKeyInfo, 'fhePublicKey', fheKeyInfoName, {});

  const fhePublicKey = fheKeyInfo.fhePublicKey;
  const fhePublicKeyName = `${fheKeyInfoName}.fhePublicKey`;

  // value.fheKeyInfo[0].fhePublicKey.dataId (string)
  assertRecordStringProperty(fhePublicKey, 'dataId', fhePublicKeyName, {});
  // value.fheKeyInfo[0].fhePublicKey.urls (string[])
  assertRecordStringArrayProperty(fhePublicKey, 'urls', fhePublicKeyName, {});
  if (fhePublicKey.urls.length !== 1) {
    throw new Error(`Unexpected '${fhePublicKeyName}.urls' array length.`);
  }

  // value.crs (record)
  assertRecordNonNullableProperty(value, 'crs', valueName, {});
  // value.crs[2048] (record)
  assertRecordNonNullableProperty(value.crs, '2048', `${valueName}.crs`, {});
  // value.crs[2048].dataId (record)
  assertRecordStringProperty(value.crs[2048], 'dataId', `${valueName}.crs[2048]`, {});
  // value.crs[2048].urls (string[])
  assertRecordStringArrayProperty(value.crs[2048], 'urls', `${valueName}.crs[2048]`, {});
  if (value.crs[2048].urls.length !== 1) {
    throw new Error(`Unexpected '${valueName}.crs[2048].urls' array length.`);
  }
}

////////////////////////////////////////////////////////////////////////////////
