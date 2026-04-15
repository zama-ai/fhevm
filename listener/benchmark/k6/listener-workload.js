import http from 'k6/http';
import { check, sleep } from 'k6';
import { textSummary } from 'https://jslib.k6.io/k6-summary/0.1.0/index.js';
import {
  RPC_METHODS,
  getSelectedChains,
  getRandomChain,
  makeRpcRequest,
  getFullUrl,
  recordMetrics,
} from './config.js';

// =============================================================================
// LISTENER WORKLOAD - Chain Agnostic
// =============================================================================
// Simulates realistic blockchain listener/indexer load pattern
// 
// Usage:
//   k6 run listener-workload.js --env CHAIN_ID=11155111
//   k6 run listener-workload.js --env CHAINS=11155111,43113
//   k6 run listener-workload.js  # defaults to all chains
// =============================================================================

// =============================================================================
// CONFIGURATION
// =============================================================================
const RATE = parseInt(__ENV.RATE || '50');        // Requests per second
const DURATION = __ENV.DURATION || '10m';         // Test duration
const CONFIG_TAG = __ENV.CONFIG || 'default';     // public/mixed/default

// =============================================================================
// METHOD DISTRIBUTION (Realistic Listener Pattern)
// =============================================================================
// Note: eth_getLogs removed - not used by this listener
const METHOD_WEIGHTS = {
  [RPC_METHODS.ETH_BLOCK_NUMBER]: 55,        // Polling heartbeat (was 50%)
  [RPC_METHODS.ETH_GET_BLOCK_BY_NUMBER]: 30, // Block fetching (was 25%)
  [RPC_METHODS.ETH_GET_BLOCK_RECEIPTS]: 13,  // Receipt fetching (was 8%)
  [RPC_METHODS.ETH_GET_BLOCK_BY_HASH]: 2,    // Reorg handling (was 2%)
};

// Select chains based on environment
const selectedChains = getSelectedChains();

// =============================================================================
// K6 OPTIONS
// =============================================================================
export const options = {
  scenarios: {
    listener_load: {
      executor: 'constant-arrival-rate',
      rate: RATE,
      timeUnit: '1s',
      duration: DURATION,
      preAllocatedVUs: RATE * 2,
      maxVUs: RATE * 4,
    },
  },
  thresholds: {
    'http_req_failed': ['rate<0.05'],  // Less than 5% errors
  },
};

// =============================================================================
// STATE CACHE (per-VU, shared across iterations)
// =============================================================================
const chainStateCache = new Map();

function initializeCache() {
  for (const chain of selectedChains) {
    if (!chainStateCache.has(chain.id)) {
      chainStateCache.set(chain.id, {
        latestBlockNumber: null,
        latestBlockHash: null,
        timestamp: 0,
      });
    }
  }
}

// Initialize cache on VU startup
initializeCache();

// =============================================================================
// MAIN TEST FUNCTION
// =============================================================================
export default function () {
  const params = {
    headers: {
      'Content-Type': 'application/json',
      'Accept-Encoding': 'gzip, deflate',
    },
    timeout: '60s',
  };

  // Select random chain from selected chains
  const chain = getRandomChain(selectedChains);
  
  // Select method based on weighted distribution
  const method = selectWeightedMethod();
  
  const startTime = Date.now();
  let payload;

  switch (method) {
    case RPC_METHODS.ETH_BLOCK_NUMBER:
      payload = makeRpcRequest(method);
      break;

    case RPC_METHODS.ETH_GET_BLOCK_BY_NUMBER:
      const blockNum = getLatestBlockNumber(chain);
      payload = makeRpcRequest(method, [blockNum, false]);
      break;

    case RPC_METHODS.ETH_GET_BLOCK_BY_HASH:
      const blockHash = getCachedBlockHash(chain);
      if (!blockHash) {
        // Skip if we don't have a cached hash yet
        return;
      }
      payload = makeRpcRequest(method, [blockHash, false]);
      break;

    case RPC_METHODS.ETH_GET_BLOCK_RECEIPTS:
      const receiptBlockNum = getLatestBlockNumber(chain);
      payload = makeRpcRequest(method, [receiptBlockNum]);
      break;

    default:
      console.error(`Unsupported method: ${method}`);
      return;
  }

  const requestTags = {
    method: method,
    chain: chain.name,
    chainId: chain.id.toString(),
    config: CONFIG_TAG,
  };

  // Add tags to the HTTP request params so http_req_duration gets tagged
  const taggedParams = {
    ...params,
    tags: requestTags,
  };

  const res = http.post(getFullUrl(chain), JSON.stringify(payload), taggedParams);
  const success = recordMetrics(res, method, chain, startTime);

  check(res, {
    'status is 200': (r) => r.status === 200,
    'has valid result': (r) => {
      try {
        const body = JSON.parse(r.body);
        return body.result !== undefined && !body.error;
      } catch (e) {
        return false;
      }
    },
  }, requestTags);

  if (__ENV.DEBUG) {
    console.log(`Request: method=${method}, chain=${chain.name}, duration=${Date.now() - startTime}ms, success=${success}`);
  }

  // Update cache if we got a successful block response
  if (success && method === RPC_METHODS.ETH_GET_BLOCK_BY_NUMBER) {
    updateBlockCache(chain, res);
  }

  // Small sleep to smooth request distribution
  sleep(0.01);
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

function selectWeightedMethod() {
  const rand = Math.random() * 100;
  let cumulative = 0;

  for (const [method, weight] of Object.entries(METHOD_WEIGHTS)) {
    cumulative += weight;
    if (rand <= cumulative) {
      return method;
    }
  }

  return RPC_METHODS.ETH_BLOCK_NUMBER; // Fallback
}

function getLatestBlockNumber(chain) {
  const cached = chainStateCache.get(chain.id);
  const now = Date.now();

  // Use cache if less than 5 seconds old
  if (cached && cached.latestBlockNumber && (now - cached.timestamp) < 5000) {
    return cached.latestBlockNumber;
  }

  // Fetch latest block number
  const payload = makeRpcRequest(RPC_METHODS.ETH_BLOCK_NUMBER);
  const res = http.post(getFullUrl(chain), JSON.stringify(payload), {
    headers: { 'Content-Type': 'application/json' },
    timeout: '10s',
  });

  if (res.status === 200) {
    try {
      const body = JSON.parse(res.body);
      if (body.result) {
        cached.latestBlockNumber = body.result;
        cached.timestamp = now;
        return body.result;
      }
    } catch (e) {
      // Ignore parsing errors
    }
  }

  return 'latest'; // Fallback to 'latest' tag
}

function getCachedBlockHash(chain) {
  const cached = chainStateCache.get(chain.id);
  return cached ? cached.latestBlockHash : null;
}

function updateBlockCache(chain, res) {
  try {
    const body = JSON.parse(res.body);
    if (body.result && body.result.hash && body.result.number) {
      const cached = chainStateCache.get(chain.id);
      cached.latestBlockHash = body.result.hash;
      cached.latestBlockNumber = body.result.number;
      cached.timestamp = Date.now();
    }
  } catch (e) {
    // Ignore cache update errors
  }
}

// =============================================================================
// SUMMARY OUTPUT
// =============================================================================
export function handleSummary(data) {
  const totalReqs = data.metrics.http_reqs?.values?.count || 0;
  const errorRate = data.metrics.http_req_failed?.values?.rate || 0;
  const avgDuration = data.metrics.http_req_duration?.values?.avg || 0;
  const p95Duration = data.metrics.http_req_duration?.values?.['p(95)'] || 0;
  const p99Duration = data.metrics.http_req_duration?.values?.['p(99)'] || 0;

  console.log('\n=== Listener Workload Summary ===');
  console.log(`  Chains: ${selectedChains.map(c => c.name).join(', ')}`);
  console.log(`  Total requests: ${totalReqs}`);
  console.log(`  Error rate: ${(errorRate * 100).toFixed(2)}%`);
  console.log(`  Avg latency: ${avgDuration.toFixed(0)}ms`);
  console.log(`  P95 latency: ${p95Duration.toFixed(0)}ms`);
  console.log(`  P99 latency: ${p99Duration.toFixed(0)}ms`);
  
  console.log('\n=== Method Distribution (Expected) ===');
  for (const [method, weight] of Object.entries(METHOD_WEIGHTS)) {
    console.log(`  ${method}: ${weight}%`);
  }

  // Show per-method statistics from custom Trend metrics
  console.log('\n=== Per-Method Performance ===');
  
  const methodMetrics = {
    'eth_blockNumber': 'latency_eth_blockNumber',
    'eth_getBlockByNumber': 'latency_eth_getBlockByNumber',
    'eth_getBlockByHash': 'latency_eth_getBlockByHash',
    'eth_getBlockReceipts': 'latency_eth_getBlockReceipts',
  };
  
  for (const [method, metricName] of Object.entries(methodMetrics)) {
    const metric = data.metrics[metricName];
    
    if (metric && metric.values && metric.values.avg > 0) {
      const avg = metric.values.avg || 0;
      const min = metric.values.min || 0;
      const max = metric.values.max || 0;
      const p50 = metric.values.med || 0;
      const p95 = metric.values['p(95)'] || 0;
      const p99 = metric.values['p(99)'] || 0;
      
      console.log(`  ${method}:`);
      console.log(`    - Min: ${min.toFixed(1)}ms`);
      console.log(`    - Avg: ${avg.toFixed(1)}ms`);
      console.log(`    - P50: ${p50.toFixed(1)}ms`);
      console.log(`    - P95: ${p95.toFixed(1)}ms`);
      console.log(`    - P99: ${p99.toFixed(1)}ms`);
      console.log(`    - Max: ${max.toFixed(1)}ms`);
    } else {
      console.log(`  ${method}: Not tested (0% weight or no data)`);
    }
  }

  // Show per-method error breakdown
  console.log('\n=== Error Breakdown ===');
  const totalErrors = data.metrics.method_errors?.values?.count || 0;
  console.log(`  Total errors: ${totalErrors}`);
  
  if (totalErrors > 0) {
    // Extract per-method errors from tagged metrics
    const methodErrorCounts = {};
    
    // Parse method_errors counter with tags
    if (data.metrics.method_errors?.values?.tags) {
      for (const [tagKey, metricData] of Object.entries(data.metrics.method_errors.values.tags)) {
        const methodMatch = tagKey.match(/method:([^,]+)/);
        const chainMatch = tagKey.match(/chain:([^,]+)/);
        
        if (methodMatch) {
          const method = methodMatch[1];
          const chain = chainMatch ? chainMatch[1] : 'unknown';
          const count = metricData.count || metricData;
          
          if (!methodErrorCounts[method]) {
            methodErrorCounts[method] = {};
          }
          methodErrorCounts[method][chain] = count;
        }
      }
    }
    
    for (const [method, chainCounts] of Object.entries(methodErrorCounts)) {
      console.log(`  ${method}:`);
      for (const [chain, count] of Object.entries(chainCounts)) {
        console.log(`    - ${chain}: ${count} errors`);
      }
    }
  }

  // Show JSON-RPC error details if any
  const jsonRpcErrorCount = data.metrics.jsonrpc_errors?.values?.count || 0;
  if (jsonRpcErrorCount > 0) {
    console.log('\n=== JSON-RPC Errors ===');
    console.log(`  Total JSON-RPC errors: ${jsonRpcErrorCount}`);
  }

  return {
    stdout: textSummary(data, { indent: '  ', enableColors: true }),
  };
}
