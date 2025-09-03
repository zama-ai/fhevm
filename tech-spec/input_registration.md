# Input registration (a.k.a. ZkPoK, Zero-knowledge Proof of Knowledge)
Usually the first step when interacting with a confidential smart contract is to add an encrypted input.
To do so a user needs to push "fresh" ciphertexts with a proof of knowledge to the Gateway.
If everything is ok, i.e. the coprocessors verified and validated the proof for the given ciphertexts, a handle is associated to each ciphertext.

Since the goal of the Console and the Relayer is to abstract the protocol away from the user a HTTP endpoint exists for it.
<!-- TODO: add diagram -->