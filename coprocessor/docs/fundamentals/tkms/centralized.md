# Centralized

The centralized realization is part of the same binary as the KMS Core. That is, it has very low overhead. However, it also means that to compromise the secret FHE key that is able to decrypt all ciphertexts, one would only need to compromise the key storage of the KMS Core. This may simply involve compromising the local file-system if Nitro is not used.

Public and private key storage may be done on the local filesystem, or it may be outsourced to an S3 instance.
Observe that there is a different strategy for the public, respectively the private key material. This is because the public key material is _never_ loaded again after construction by the KMS Core, but is required to be easily accessible to other systems. On the other hand, the private key material is only used by the KMS Core and is never exposed to other systems. Furthermore, it is loaded into RAM during each booting of the KMS Core.

The cryptographic operations carried out by the centralized back=end are carried out directly through the usage of [tfhe-rs](https://github.com/zama-ai/tfhe-rs).