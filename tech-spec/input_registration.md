# Input registration (also known as ZkPoK (Zero-knowledge Proof of Knowledge))


[Reference in Zama's blockchain tech-spec](https://github.com/zama-ai/tech-spec/blob/main/architecture/zkpok.md)


Usually the first step when interacting with a confidential smart contract is to add an encrypted input.
To do so a user needs to push the newly created ciphertext with a proof of knowledge to the gateway.
If everything is ok, i.e. the coprocessors verified and validated the proof for the given ciphertexts a handle is associated to each ciphertext.

Since the goal of the Console and the Relayer is to abstract the gateway away from the user an HTTP endpoint as to exist for it.

## HTTP endpoint inputs

<!-- TODO: add http endpoint inputs and how to call it using the npm httpz/sdk library -->

