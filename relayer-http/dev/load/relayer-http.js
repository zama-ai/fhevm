// k6 traffic for relayer-http, in the profiles of monitoring/design.md (Traffic profile): `make load`.
//
//   PROFILE=normal  one request every 5 s (a few thousand a day, compressed), DURATION default 10m
//   PROFILE=load    RATE requests per second (default 100), DURATION default 2m
//
// The mix, drawn per request:
//   70 %  public decrypt of a random non-empty subset of HANDLES: few distinct contents, so many repeats
//   10 %  public decrypt of a handle from an unsupported chain: 400 malformed, nothing forwarded
//   10 %  public decrypt with no handle: 400 malformed
//    5 %  user decrypt with an empty body: 400 malformed
//    5 %  an unknown route: 404 not_found, flow none
// User decrypts that reach the KMS nodes need a user's EIP-712 signature: `make user-decrypt`, not this script.
import http from 'k6/http';
import { check } from 'k6';

const URL = __ENV.RELAYER_HTTP_URL;
const HANDLES = (__ENV.HANDLES || '').split(',').filter(Boolean);
// The current KMS context's extraData, for the requests the nodes must accept; the rejected ones never reach them.
const EXTRA_DATA = __ENV.EXTRA_DATA || '0x00';
const PROFILE = __ENV.PROFILE || 'normal';
const PROFILES = {
  normal: { rate: 1, timeUnit: '5s', duration: __ENV.DURATION || '10m' },
  load: { rate: Number(__ENV.RATE || 100), timeUnit: '1s', duration: __ENV.DURATION || '2m' },
};
const profile = PROFILES[PROFILE];
if (!profile) throw new Error(`PROFILE must be one of ${Object.keys(PROFILES)}`);

export const options = {
  scenarios: {
    [PROFILE]: {
      executor: 'constant-arrival-rate',
      rate: profile.rate,
      timeUnit: profile.timeUnit,
      duration: profile.duration,
      preAllocatedVUs: 20,
      maxVUs: 1000,
    },
  },
  summaryTrendStats: ['p(50)', 'p(95)', 'p(99)', 'max'],
};

const JSON_HEADERS = { headers: { 'content-type': 'application/json' } };

// A random non-empty subset of the handles, in their order.
function someHandles() {
  const picked = HANDLES.filter(() => Math.random() < 0.5);
  return picked.length > 0 ? picked : [HANDLES[Math.floor(Math.random() * HANDLES.length)]];
}

// `handle` with its chain id (bytes 22..30) replaced by 999, which no configuration supports.
function otherChain(handle) {
  return handle.slice(0, 2 + 44) + '00000000000003e7' + handle.slice(2 + 60);
}

function post(path, body, kind) {
  return http.post(`${URL}${path}`, JSON.stringify(body), { ...JSON_HEADERS, tags: { kind } });
}

export default function () {
  const draw = Math.random();
  let response;
  let kind;
  if (HANDLES.length > 0 && draw < 0.7) {
    kind = 'public_decrypt';
    response = post('/v4/public-decrypt', { ciphertextHandles: someHandles(), extraData: EXTRA_DATA }, kind);
  } else if (HANDLES.length > 0 && draw < 0.8) {
    kind = 'unsupported_chain';
    response = post('/v4/public-decrypt', { ciphertextHandles: [otherChain(HANDLES[0])], extraData: '0x00' }, kind);
  } else if (draw < 0.9) {
    kind = 'no_handle';
    response = post('/v4/public-decrypt', { ciphertextHandles: [], extraData: '0x00' }, kind);
  } else if (draw < 0.95) {
    kind = 'empty_user_decrypt';
    response = post('/v4/user-decrypt', {}, kind);
  } else {
    kind = 'unknown_route';
    response = http.get(`${URL}/v4/nope`, { tags: { kind } });
  }
  const expected = kind === 'public_decrypt' ? [200, 502, 503, 504, 403, 404] : kind === 'unknown_route' ? [404] : [400];
  check(response, { [`${kind}: expected status`]: (r) => expected.includes(r.status) });
}
