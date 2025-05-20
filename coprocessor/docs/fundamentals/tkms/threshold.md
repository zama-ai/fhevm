# Threshold

The threshold realization is part of the same binary as the KMS Core, but `n` KMS Cores are running independently of each other, hosted by different companies. This means that in order to compromise the secret FHE key that is able to decrypt all ciphertexts, one would only need to compromise the key storage of _at least_ `t` KMS Cores administered by distinct companies on distinct servers.
More specifically this may simply involve compromising more than `t` local file-systems if Nitro is not used, more than `t` Nitro enclaves.

Public and private key storage may be done on the local filesystem, or it may be outsourced to an S3 instance.
Observe that there is a different strategy for the public, respectively the private key material. This is because the public key material is _never_ loaded again after construction by the KMS Core, but is required to be easily accessible to other systems. On the other hand, the private key material is only used by the KMS Core and is never exposed to other systems. Furthermore, it is loaded into RAM during each booting of the KMS Core.

The cryptographic operations carried out by the threshold back=end are fulfilled by an MPC implementation of the necessary operations of the [tfhe-rs](https://github.com/zama-ai/tfhe-rs) library.
The underlying MPC protocol is what is known as a _statistically maliciously robust_ and _proactively_ secure MPC protocol. Specifically this implies the following:
- Statistically: the underlying protocols cannot be “broken” by an adversary regardless of the amount of computation power. This also means that they do not rely on any exotic cryptographic assumptions. (For practical reasons standard security of hash functions is still required.)
- Maliciously Robust: the protocol can finish execution _correctly_ with up to `t` parties misbehaving by running rogue software or not participating.
- Proactive: it is possible to "undo" a leakage of key material of at most `t` parties byt refreshing their key shares. That is, if a few servers are compromised it is possible to make the stolen material 100% useless without the need to regenerate a new public key.

The MPC protocol is based on peer-reviewed cryptographic core protocols and peer reviewed modifications. For more modifications see [this paper](https://eprint.iacr.org/2023/815).