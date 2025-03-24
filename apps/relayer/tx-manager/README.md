# Transaction Manager

A service to submit transaction to any supported chain.

Inputs:
- Payload
    - TODO: figure out if the contract address is in the payload

The transaction should pass if correct.

## HTTPZ Gateway

Acceptable failures:
- The contract does not exist.
- The execution is reverted because of an ACL issue.

Unacceptable failures:
- Gas issue 

## HTTPZ Host

Acceptable failures:
- The contract does not exist.
- The method does not exist with the expected ABI.

Unacceptable failures:
- Gas issue.
