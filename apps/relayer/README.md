# Relayer

Services to handle relaying logic between a native chain or a user and the HTTPZ Gateway chain.


## Requirements

Rust and Docker Compose are required to run the relayer.

## How to launch

Run the stack and the relayer:

`make httpz-clean && make console-side-clean && make httpz-run && make console-side-run && make relayer-run` 

Run the input test once the relayer is running:

`make httpz-test-input`


## To do

- [x] Catch input registration events
- [x] Catch public decryption events from Host chain listener
- [ ] Catch private decryption events
- [ ] Mock HTTP and catch private decryption and input registration inputs.
    - Useful to launch the relayer as a standalone application. <br>Indeed the requests should come from the sqs listener and the relayer won't have a direct HTTP endpoint.
- [ ] Mock Authorization 
    - Useful to launch the relayer as a standalone application. <br>Indeed for debugging it's quite nice to just approve all requests to make sure that things work properly without launching the full stack.
- [ ] Send authorization request on public-decryption event
- [ ] Authorize request from Console mock
- [ ] Send transaction request to tx-manager
- [ ] Send transaction on-chain in tx-manager mock
- [ ] Split tx-manager into another service
- [ ] Send receipt after tx-manager
- [ ] Catch callback from Gateway
- [ ] Send callback to SQS
    - Input registration
    - Private decryption
- [ ] Send callback on-chain
- [ ] Do a pass over error handling
    - SQS not being reachable
    - Pg not being reachable
- [ ] Add proper integration tests
- [ ] Fine-grain input validation
    - Filter out invalid ciphertexts/proofs
    - Check ACL on both chains

