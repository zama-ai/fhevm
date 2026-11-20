-- `KmsNodeParams` gained `connectorUrl`, so `new_kms_context.kms_node_params` now holds the
-- ABI encoding of the current shape. Rows stored before this migration hold the v0.14 shape and
-- no longer decode. Their context cannot still be pending on-chain, because `reinitializeV3`
-- reverts while a lifecycle operation is in flight.
UPDATE new_kms_context SET status = 'failed' WHERE status IN ('pending', 'under_process');
