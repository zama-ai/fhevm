# Inclusion Proof

The execution layer in fhEVM can perform computations on ciphertexts. At some point, it becomes necessary to reveal the actual values of these ciphertexts. However, the private key is managed by the KMS (Key Management System). The question arises: how can we perform asynchronous decryption requests (which make the values public) and re-encryptions (for personal information) when the execution layer and the KMS are decoupled?
This is where inclusion proofs come into play.


## How to Compute an Inclusion Proof

## Verification of the Proof in KMS BC ISC

## Notes on Root Hash Verification

This section will be elaborated upon in the future to explain the validation of root hash integrity.