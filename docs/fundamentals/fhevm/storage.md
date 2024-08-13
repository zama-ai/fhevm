# Storage

Ciphertexts in fhEVM-native are stored onchain in the storage of a predefined contract that has no code and is used just for ciphertexts. At the time of writing, its address is **0x5e**.

Contract storage in the EVM is a key-value store. For ciphertexts, we use the handle as a key and the value is the actual ciphertext.

Furthermore, stored ciphertexts are immutable, making ciphertext storage append-only.

Ciphertexts can be read by anyone. We expose the `GetCiphertext` function on the `FheLib` precompiled contract. Nodes/validators must support it.

## GetCiphertext Function (selector: ff627e77)

The `GetCiphertext` function returns a serialized TFHE ciphertext given:
 * the ebool/e(u)int value (also called a handle) for which the ciphertext is requested

GetCiphertext only works via the `eth_call` RPC.

To call GetCiphertext via `eth_call`, the following Python can serve as an example:

```python
import http.client
import json

# This is the address of the FheLib precompile. This value is hardcoded per blockchain.
fhe_lib_precompile_address = "0x000000000000000000000000000000000000005d"

# The ebool/e(u)int value for which the ciphertext is requested.
handle = "f038cdc8bf630e239f143abeb039b91ec82ec17a8460582e7a409fa551030c06"

# The function selector of GetCiphertext.
get_ciphertext_selector = "ff627e77"

# Call the FheLib precompile with `data` being the handle to the ciphertext.
payload = {
    "jsonrpc": "2.0",
    "method": "eth_call",
    "params": [
        {
            "to": fhe_lib_precompile_address,
            "data": "0x" + handle
        },
        "latest"
    ],
    "id": 1,
}

con = http.client.HTTPConnection("localhost", 8545)
con.request("POST", "/", body=json.dumps(payload),
            headers={"Content-Type": "application/json"})
resp = json.loads(con.getresponse().read())

# Remove leading "0x" and decode hex to get a byte buffer with the ciphertext.
ciphertext = bytes.fromhex(resp["result"][2:])
```