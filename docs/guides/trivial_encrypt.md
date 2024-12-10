# **Trivial encryption of `ebytesXX` types**

This guide explains how to perform trivial encryption of `ebytesXX` types in fhEVM smart contracts using the new `padToBytesXX` functions. These functions ensure that byte arrays are correctly padded to match the required sizes for `ebytes64`, `ebytes128`, and `ebytes256` types.

---

## **Overview**

The `TFHE.padToBytesXX` functions are designed to make encryption of byte arrays seamless and ensure compatibility with `ebytesXX` types. These functions:

- Pad the provided byte array to the appropriate length (`64`, `128`, or `256` bytes).
- Prevent runtime errors caused by improperly sized input data.
- Work seamlessly with `TFHE.asEbytesXX` for encryption.

### **Workflow**

1. **Pad Input Data**:
   Use the `padToBytesXX` functions to ensure your byte array matches the size requirements.
2. **Encrypt the Padded Data**:
   Use `TFHE.asEbytesXX` to encrypt the padded byte array into the corresponding encrypted type.
3. **Grant Access**:
   Use `TFHE.allowThis` and `TFHE.allow` to define access control for the encrypted data.

---

## **Example: Trivial Encryption**

Below is an example demonstrating how to encrypt and manage `ebytes64`, `ebytes128`, and `ebytes256` types:

```solidity
function trivialEncrypt() public {
  // Encrypt a 64-byte array
  ebytes64 yBytes64 = TFHE.asEbytes64(
    TFHE.padToBytes64(
      hex"19d179e0cc7e816dc944582ed4f5652f5951900098fc2e0a15a7ea4dc8cfa4e3b6c54beea5ee95e56b728762f659347ce1d4aa1b05fcc5"
    )
  );
  TFHE.allowThis(yBytes64);
  TFHE.allow(yBytes64, msg.sender);

  // Encrypt a 128-byte array
  ebytes128 yBytes128 = TFHE.asEbytes128(
    TFHE.padToBytes128(
      hex"13e7819123de6e2870c7e83bb764508e22d7c3ab8a5aee6bdfb26355ef0d3f1977d651b83bf5f78634fa360aa14debdc3daa6a587b5c2fb1710ab4d6677e62a8577f2d9fecc190ad8b11c9f0a5ec3138b27da1f055437af8c90a9495dad230"
    )
  );
  TFHE.allowThis(yBytes128);
  TFHE.allow(yBytes128, msg.sender);

  // Encrypt a 256-byte array
  ebytes256 yBytes256 = TFHE.asEbytes256(
    TFHE.padToBytes256(
      hex"d179e0cc7e816dc944582ed4f5652f5951900098fc2e0a15a7ea4dc8cfa4e3b6c54beea5ee95e56b728762f659347ce1d4aa1b05fcc513e7819123de6e2870c7e83bb764508e22d7c3ab8a5aee6bdfb26355ef0d3f1977d651b83bf5f78634fa360aa14debdc3daa6a587b5c2fb1710ab4d6677e62a8577f2d9fecc190ad8b11c9f0a5ec3138b27da1f055437af8c90a9495dad230"
    )
  );
  TFHE.allowThis(yBytes256);
  TFHE.allow(yBytes256, msg.sender);
}
```

---

## Function details

### `TFHE.padToBytesXX`

Pads the given byte array to the required size for encryption as `ebytesXX`.

- **Function signatures**:

  - `TFHE.padToBytes64(bytes calldata input) public pure returns (bytes memory)`
  - `TFHE.padToBytes128(bytes calldata input) public pure returns (bytes memory)`
  - `TFHE.padToBytes256(bytes calldata input) public pure returns (bytes memory)`

- **Inputs**:

  - `input`: A `bytes` array to be padded.

- **Outputs**:
  - A `bytes` array padded to the size specified by the function (`64`, `128`, or `256` bytes).

### `TFHE.asEbytesXX`

Converts the padded byte array into an encrypted `ebytesXX` type.

- **Function Signatures**:

  - `TFHE.asEbytes64(bytes memory paddedInput) public returns (ebytes64)`
  - `TFHE.asEbytes128(bytes memory paddedInput) public returns (ebytes128)`
  - `TFHE.asEbytes256(bytes memory paddedInput) public returns (ebytes256)`

- **Inputs**:

  - `paddedInput`: A byte array already padded to the required size.

- **Outputs**:
  - Encrypted `ebytesXX` type.

---

## Best Practices

1. **Always Pad Input Data**:
   Ensure input data is padded to the correct length using `padToBytesXX` before encrypting.
2. **Access Control**:
   Use `TFHE.allowThis` and `TFHE.allow` to specify which addresses or contracts can access the encrypted data.

3. **Test Thoroughly**:
   Verify the padding and encryption in both mocked and real fhEVM environments to ensure correct behavior.
