import { probeBootstrap } from "../src/flow/readiness";
import { consumerOnlyHostListeners } from "../src/host-listener-mode";
import { loadState } from "../src/state/state";

const state = await loadState();
if (!state || !consumerOnlyHostListeners(state.scenario)) {
  throw new Error("Expected a consumer-only stack");
}
// kms-generation is a threshold-KMS quorum test, unsuitable for the centralized
// KMS used here. Audit the actual on-chain activation and published key material.
const material = await probeBootstrap(state);
if (!material) throw new Error("Consumer-only key bootstrap did not complete");
if (material.actualFheKeyId !== state.discovery?.actualFheKeyId ||
    material.actualCrsKeyId !== state.discovery?.actualCrsKeyId) {
  throw new Error("Active key material differs from the bootstrapped stack");
}
console.log(`[consumer-bootstrap] Active FHE key ${material.actualFheKeyId}, CRS ${material.actualCrsKeyId}; published material verified`);
