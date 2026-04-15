import { Counter, Trend } from 'k6/metrics';

// =============================================================================
// CHAIN REGISTRY
// =============================================================================
export const CHAIN_REGISTRY = {
  11155111: { 
    id: 11155111,
    name: 'Sepolia', 
    endpoint: '/listener-indexer/evm/11155111',
    blockTime: 12,
  },
  43113: { 
    id: 43113,
    name: 'Fuji', 
    endpoint: '/listener-indexer/evm/43113',
    blockTime: 2,
  },
  97: { 
    id: 97,
    name: 'BSC-Testnet', 
    endpoint: '/listener-indexer/evm/97',
    blockTime: 3,
  },
  80002: { 
    id: 80002,
    name: 'Amoy', 
    endpoint: '/listener-indexer/evm/80002',
    blockTime: 2,
  },
  84532: { 
    id: 84532,
    name: 'Base-Sepolia', 
    endpoint: '/listener-indexer/evm/84532',
    blockTime: 2,
  },
};

export const BASE_URL = __ENV.BASE_URL || 'http://localhost:4000';

// =============================================================================
// RPC METHODS
// =============================================================================
export const RPC_METHODS = {
  ETH_BLOCK_NUMBER: 'eth_blockNumber',
  ETH_GET_BLOCK_BY_NUMBER: 'eth_getBlockByNumber',
  ETH_GET_BLOCK_BY_HASH: 'eth_getBlockByHash',
  ETH_GET_BLOCK_RECEIPTS: 'eth_getBlockReceipts',
  ETH_GET_LOGS: 'eth_getLogs',
};

// =============================================================================
// CUSTOM METRICS - Separate Trend per method for proper breakdown
// =============================================================================
export const methodLatencyBlockNumber = new Trend('latency_eth_blockNumber', true);
export const methodLatencyGetBlock = new Trend('latency_eth_getBlockByNumber', true);
export const methodLatencyGetBlockHash = new Trend('latency_eth_getBlockByHash', true);
export const methodLatencyGetReceipts = new Trend('latency_eth_getBlockReceipts', true);

export const methodErrors = new Counter('method_errors');
export const chainErrors = new Counter('chain_errors');
export const responseSizes = new Trend('response_sizes');
export const jsonRpcErrors = new Counter('jsonrpc_errors');
export const parsingErrors = new Counter('parsing_errors');

// =============================================================================
// CHAIN SELECTION HELPERS
// =============================================================================

/**
 * Get chains to test based on environment variables
 * Priority: CHAIN_ID > CHAINS > all chains
 */
export function getSelectedChains() {
  // Single chain via CHAIN_ID
  if (__ENV.CHAIN_ID) {
    const chainId = parseInt(__ENV.CHAIN_ID);
    const chain = CHAIN_REGISTRY[chainId];
    if (!chain) {
      throw new Error(`Invalid CHAIN_ID: ${chainId}. Valid IDs: ${Object.keys(CHAIN_REGISTRY).join(', ')}`);
    }
    return [chain];
  }

  // Multiple chains via CHAINS (comma-separated)
  if (__ENV.CHAINS) {
    const chainIds = __ENV.CHAINS.split(',').map(id => id.trim());
    const chains = chainIds.map(id => {
      const chainId = parseInt(id);
      const chain = CHAIN_REGISTRY[chainId];
      if (!chain) {
        throw new Error(`Invalid chain ID in CHAINS: ${id}. Valid IDs: ${Object.keys(CHAIN_REGISTRY).join(', ')}`);
      }
      return chain;
    });
    return chains;
  }

  // Default: all chains
  return Object.values(CHAIN_REGISTRY);
}

/**
 * Get random chain from selected chains
 */
export function getRandomChain(chains) {
  return chains[randomIntBetween(0, chains.length - 1)];
}

// =============================================================================
// UTILITY FUNCTIONS
// =============================================================================

export function randomIntBetween(min, max) {
  return Math.floor(Math.random() * (max - min + 1)) + min;
}

export function makeRpcRequest(method, params = []) {
  return {
    jsonrpc: '2.0',
    method: method,
    params: params,
    id: Math.floor(Math.random() * 100000000),
  };
}

export function getFullUrl(chain) {
  return `${BASE_URL}${chain.endpoint}`;
}

export function truncateString(str, maxLength = 200) {
  if (!str || str.length <= maxLength) return str;
  const half = Math.floor(maxLength / 2);
  return `${str.substring(0, half)}...${str.substring(str.length - half)}`;
}

/**
 * Record metrics for a request
 * @returns {boolean} success - true if request was successful
 */
export function recordMetrics(res, method, chain, startTime) {
  const duration = Date.now() - startTime;
  
  const tags = {
    method: method,
    chain: chain.name,
    chainId: chain.id.toString(),
  };

  if (!res || !res.status) {
    console.error(`[ERROR] No response - Method: ${method}, Chain: ${chain.name}`);
    methodErrors.add(1, tags);
    chainErrors.add(1, tags);
    return false;
  }

  // Record response size
  if (res.body) {
    responseSizes.add(res.body.length, tags);
  }

  // Record method-specific latency
  switch(method) {
    case 'eth_blockNumber':
      methodLatencyBlockNumber.add(duration, tags);
      break;
    case 'eth_getBlockByNumber':
      methodLatencyGetBlock.add(duration, tags);
      break;
    case 'eth_getBlockByHash':
      methodLatencyGetBlockHash.add(duration, tags);
      break;
    case 'eth_getBlockReceipts':
      methodLatencyGetReceipts.add(duration, tags);
      break;
  }

  // Parse response and check for errors
  let parsedBody = null;
  try {
    parsedBody = JSON.parse(res.body);
  } catch (e) {
    console.error(`[ERROR] Parsing failed - Method: ${method}, Chain: ${chain.name}, Body: ${truncateString(res.body, 100)}`);
    parsingErrors.add(1, tags);
    methodErrors.add(1, tags);
    chainErrors.add(1, tags);
    return false;
  }

  // Check for JSON-RPC errors
  if (parsedBody?.error) {
    const errorTags = {
      ...tags,
      error_code: parsedBody.error.code?.toString() || 'unknown',
      error_message: truncateString(parsedBody.error.message || '', 50),
    };
    
    console.error(`[ERROR] JSON-RPC error - Method: ${method}, Chain: ${chain.name}, Code: ${parsedBody.error.code}, Message: ${parsedBody.error.message}`);
    
    jsonRpcErrors.add(1, errorTags);
    methodErrors.add(1, tags);
    chainErrors.add(1, tags);
    
    return false;
  }

  // Success if we have a result
  return parsedBody?.result !== undefined;
}
