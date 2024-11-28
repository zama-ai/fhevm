# Gateway API Specifications

## Endpoints

<details>
  <summary>GET /keyurl ---- Retrieve links for retrieving the public keys and CRS' in the system.</summary>

#### Description

This endpoint returns a JSON object containing URLs from an S3 bucket, allowing the client to download key files such as the blockchain public key, CRS files for input proof generation, the bootstrap key, and the address and public verification keys for each of the MPC servers running the TKMS.

For each file (with the exception of the verification key and address), a list of cryptographic signatures is provided to ensure the integrity and authenticity of the downloaded content. These signatures should be considered as a multi-sig. This means that instead of needing all the signatures to validate the content, only a subset, specifically >1/3 of the total signatures (if n nodes are signing), is required to verify that the content is legitimate. 

No query parameters are required, as the gateway is already preconfigured for a specific blockchain.


#### Query Parameters

No parameters.

#### Headers

None.

#### Response

**Success (200 OK)**

The request is successful, and the response will include a JSON object with a `status` and a `response`. The response again consists of the following elements: 
- `crs`: A map of containing information on the different CRS'. The key of the map is the max amount of bits the CRS can support proofs for. The value is an object of the following elements for the given CRS:
    * `data_id`: The 20 byte (lower-case) hex encoded handle/ID identifying the CRS.
    * `param_choice`: An integer representing the choice of parameters for the public key to be used with the CRS. 
    * `signatures`: A list of signatures (one from each MPC party constituting the TKMS backend). Each signature is a hex (lower-case) encoded EIP712 signature on the `safe_serialization` of `PublicParam<Bls12_446>`.
    * `urls`: A list of URLs where the data can be fetched. The data at the end-point is a `safe_serialization` of `PublicParam<Bls12_446>`.
- `fhe_key_info`: A list of objects, each representing information on a key-set in the system. More specifically each element consists of the following:
    * `fhe_public_key`: An element which contains information about the public encryption key of a FHE key set. More specifically it consists of the following elements:
        * `data_id`: The 20 byte (lower-case) hex encoded handle/ID identifying the key.
        * `param_choice`: An integer representing the choice of parameters used to generate the key. 
        * `signatures`: A list of signatures (one from each MPC party constituting the TKMS backend). Each signature is a hex (lower-case) encoded EIP712 signature on the `safe_serialization` of `CompactPublicKey`.
        * `urls`: A list of URLs where the data can be fetched. The data at the end-point is a `safe_serialization` of `CompactPublicKey`.
    * `fhe_server_key`: An element which contains information about the server key (the key used to perform FHE operations on ciphertexts) of a FHE key set. More specifically it consists of the following elements:
        * `data_id`: The 20 byte (lower-case) hex encoded handle/ID identifying the key.
        * `param_choice`: An integer representing the choice of parameters used to generate the key. 
        * `signatures`: A list of signatures (one from each MPC party constituting the TKMS backend). Each signature is a hex (lower-case) encoded EIP712 signature on the `safe_serialization` of `CompactPublicKey`.
        * `urls`: A list of URLs where the data can be fetched. The data at the end-point is a `safe_serialization` of `ServerKey`.
- `verf_public_key`: **Deprecated and will be removed in the future. Should instead be fetched directly from the config contract on the TKMS blockchain** A list containing of elements, where each element is the information about a TKMS MPC server's signing key. That is, the key which a server uses to sign requests. More specifically each element of the vector consists of the following:
        * `key_id`: The 20 byte (lower-case) hex encoded handle/ID identifying the key. Currently this value is static for signing keys. That is, it will always be `408d8cbaa51dece7f782fe04ba0b1c1d017b1088`.
        * `server_id`: The integer ID of the server whose key is being described in the current element. This is an integer in the range [1; n], where n is the amount of MPC servers.
        * `verf_public_key_url`: The URL end-point of the given server where a serialization of the signing key can be found. The signing key is a `safe_serialization` of `PublicSigKey`.
        * `verf_public_key_address`: The URL end-point of the given server where a file containing the human-readable Ethereum address of the server's signing key.

For example the following:

```json
{
    "response": {
        "crs": {
            "256": {
                "data_id": "d8d94eb3a23d22d3eb6b5e7b694e8afcd571d906",
                "param_choice": 1,
                "signatures": [
                    "0d13...",
                    "4250...",
                    "a42c...",
                    "fhb5..."
                ],
                "urls": [
                    "https://s3.amazonaws.com/bucket-name-1/PUB-p1/CRS/d8d94eb3a23d22d3eb6b5e7b694e8afcd571d906",
                    "https://s3.amazonaws.com/bucket-name-4/PUB-p4/CRS/d8d94eb3a23d22d3eb6b5e7b694e8afcd571d906",
                    "https://s3.amazonaws.com/bucket-name-2/PUB-p2/CRS/d8d94eb3a23d22d3eb6b5e7b694e8afcd571d906",
                    "https://s3.amazonaws.com/bucket-name-3/PUB-p3/CRS/d8d94eb3a23d22d3eb6b5e7b694e8afcd571d906"
                ]
            }
        },
        "fhe_key_info": [
            {
                "fhe_public_key": {
                    "data_id": "408d8cbaa51dece7f782fe04ba0b1c1d017b1088",
                    "param_choice": 1,
                    "signatures": [
                        "cdff...",
                        "123c...",
                        "00ff...",
                        "a367..."
                    ],
                    "urls": [
                        "https://s3.amazonaws.com/bucket-name-1/PUB-p1/PublicKey/408d8cbaa51dece7f782fe04ba0b1c1d017b1088",
                        "https://s3.amazonaws.com/bucket-name-4/PUB-p4/PublicKey/408d8cbaa51dece7f782fe04ba0b1c1d017b1088",
                        "https://s3.amazonaws.com/bucket-name-2/PUB-p2/PublicKey/408d8cbaa51dece7f782fe04ba0b1c1d017b1088",
                        "https://s3.amazonaws.com/bucket-name-3/PUB-p3/PublicKey/408d8cbaa51dece7f782fe04ba0b1c1d017b1088"
                    ]
                },
                "fhe_server_key": {
                    "data_id": "408d8cbaa51dece7f782fe04ba0b1c1d017b1088",
                    "param_choice": 1,
                    "signatures": [
                        "839b...",
                        "baef...",
                        "55cc...",
                        "81a4..."
                    ],
                    "urls": [
                        "https://s3.amazonaws.com/bucket-name-1/PUB-p1/ServerKey/408d8cbaa51dece7f782fe04ba0b1c1d017b1088",
                        "https://s3.amazonaws.com/bucket-name-4/PUB-p4/ServerKey/408d8cbaa51dece7f782fe04ba0b1c1d017b1088",
                        "https://s3.amazonaws.com/bucket-name-2/PUB-p2/ServerKey/408d8cbaa51dece7f782fe04ba0b1c1d017b1088",
                        "https://s3.amazonaws.com/bucket-name-3/PUB-p3/ServerKey/408d8cbaa51dece7f782fe04ba0b1c1d017b1088"
                    ]
                }
            }
        ],
        "verf_public_key": [
            {
                "key_id": "408d8cbaa51dece7f782fe04ba0b1c1d017b1088",
                "server_id": 1,
                "verf_public_key_address": "https://s3.amazonaws.com/bucket-name-1/PUB-p1/VerfAddress/408d8cbaa51dece7f782fe04ba0b1c1d017b1088",
                "verf_public_key_url": "https://s3.amazonaws.com/bucket-name-1/PUB-p1/VerfKey/408d8cbaa51dece7f782fe04ba0b1c1d017b1088"
            },
            {
                "key_id": "408d8cbaa51dece7f782fe04ba0b1c1d017b1088",
                "server_id": 4,
                "verf_public_key_address": "https://s3.amazonaws.com/bucket-name-4/PUB-p4/VerfAddress/408d8cbaa51dece7f782fe04ba0b1c1d017b1088",
                "verf_public_key_url": "https://s3.amazonaws.com/bucket-name-4//PUB-p4/VerfKey/408d8cbaa51dece7f782fe04ba0b1c1d017b1088"
            },
            {
                "key_id": "408d8cbaa51dece7f782fe04ba0b1c1d017b1088",
                "server_id": 2,
                "verf_public_key_address": "https://s3.amazonaws.com/bucket-name-2/PUB-p2/VerfAddress/408d8cbaa51dece7f782fe04ba0b1c1d017b1088",
                "verf_public_key_url": "https://s3.amazonaws.com/bucket-name-2/PUB-p2/VerfKey/408d8cbaa51dece7f782fe04ba0b1c1d017b1088"
            },
            {
                "key_id": "408d8cbaa51dece7f782fe04ba0b1c1d017b1088",
                "server_id": 3,
                "verf_public_key_address": "https://s3.amazonaws.com/bucket-name-3/PUB-p3/VerfAddress/408d8cbaa51dece7f782fe04ba0b1c1d017b1088",
                "verf_public_key_url": "https://s3.amazonaws.com/bucket-name-3/PUB-p3/VerfKey/408d8cbaa51dece7f782fe04ba0b1c1d017b1088"
            }
        ]
    },
    "status": "success"
}
```

**Error Responses**

| Status Code | Error Code   | Description                                      |
| ----------- | ------------ | ------------------------------------------------ |
| 400         | `BadRequest` | The request is invalid or missing required parameters. |
| 404         | `NotFound`   | The requested resource was not found.            |
| 500         | `ServerError` | An internal server error occurred.                   |

#### Example Error Responses

```json
{
  "error": "BadRequest",
  "message": "The request is invalid or missing required parameters."
}
```

```json
{
  "error": "NotFound",
  "message": "The requested resource was not found."
}
```

```json
{
  "error": "ServerError",
  "message": "An internal server error occurred. Please try again later."
}
```

</details>

<details>
  <summary>POST /verify_proven_ct ---- Input a batch of proven ciphertexts to be validated by the TKMS.</summary>

#### Description

This endpoint returns a JSON object containing all the signatures on the proven ciphertexts from the TKMS servers. Furthermore the response contains some meta-information distinguishing if the response is for the co-processor setting or fhEVM native setting. In case of the co-processor setting, then the ciphertext storage handles and a signature from the co-processor attesting correct storage is also included.

The signatures from the TKMS should be considered as a multi-sig. This means that instead of needing all the signatures to validate the content, only a subset, specifically >1/3 of the total signatures, is required to verify that the content is legitimate. 

#### Query Parameters

Multiple parameters must be supplied in JSON format:
- `contract_address`: An EIP-55 encoded address (that is, including the `0x` prefix) of the contract where the proven ciphertext is to be submitted.
- `caller_address`: An EIP-55 encoded address (that is, including the `0x` prefix) of the user who is providing the encrypted input.
- `crs_id`: The 20 byte (lower-case) hex encoded handle/ID identifying the CRS used to construct the proof.
- `key_id`: The 20 byte (lower-case) hex encoded handle/ID identifying the public key used to encrypt the ciphertext with.
- `ct_proof`: A hex encoding of the serialization of the proven ciphertext. More specifically the TFHE-RS object `ProvenCompactCiphertextList` serialized using `safe_serialization`.

```json
{
    "contract_address": "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed",
    "caller_address": "0xD1220A0cf47c7B9Be7A2E6BA89F429762e7b9aDb" ,
    "crs_id": "d8d94eb3a23d22d3eb6b5e7b694e8afcd571d906",
    "key_id": "408d8cbaa51dece7f782fe04ba0b1c1d017b1088",
    "ct_proof": "cdff...",
}
```

#### Headers

None.

#### Response

**Success (200 OK)**

The request is successful, and the response will include a JSON object with a `status` and a `response` which  consists of the following:

- `handles`: A vector of handles to each of the ciphertexts which have been proven knowledge of. A handle is a 32 byte (lower-case) hex encoded handle/ID identifying the ciphertext.
- `kms_signatures`: A list of signatures (one for each of the TKMS servers that respond to the query). Each signature is a hex (lower-case) encoded EIP712 signature on the `safe_serialization` of `ProvenCompactCiphertextList`.
- `listener_type`: An enum expressing whether the result is for an fhEVM native (`FHEVM_NATIVE`) or co-processor respectively (`COPROCESSOR`).
- `proof_of_storage`: An optional signature from the co-processor. More specifically if fhEVM native is used it will be an empty string, otherwise it will be a hex (lower-case) encoded EIP712 signature on the request. 

For example the following:
```json
{
    "response": {
        "handles": [
            "0748b542afe2353c86cb707e3d21044b0be1fd18efc7cbaa6a415af055bfb358",
            "054ab4515b1541878723431005054f154e15e45e15800adb67879679df670456"
        ],
        "kms_signatures":[
            "15a4f9a8eb61459cfba7d103d8f911fb04ce91ecf841b34c49c0d56a70b896d20cbc31986188f91efc3842b7df215cee8acb40178daedb8b63d0ba5d199bce121c", 
            "118165165165423465234414c4c468a4d9684d8e18186d6f786161b4b436c58787cc68418186d6f786161b4b98461166a6a6668e8e118542c154867aab238abd79", 
            "1c849848940128065242121b2b12121ed876986da251561650c654564d684654e51610879a9a9798b78b78e7f8787d87c8c8894a454547809586616161464cc8a8", 
            "10c864ac145423466798808098c098b09a8908d6432da4544f5e54566b76740454654a54c65454565d65d657e44651241561342441234128888063304854897893", 
        ],
        "listener_type": "COPROCESSOR",
        "proof_of_storage": "17acd15648740c00849f489498489e4600a60a06068d484b084894988333000cff798751651498d68768753567a4356787c45787e79i8f64d128218927897c8789"
    },
    "status": "success"
}
```

**Error Responses**

| Status Code | Error Code   | Description                                      |
| ----------- | ------------ | ------------------------------------------------ |
| 400         | `BadRequest` | The request is invalid or missing required parameters. |
| 404         | `NotFound`   | The requested resource was not found.            |
| 500         | `ServerError` | An internal server error occurred.                   |

#### Example Error Responses

```json
{
  "error": "BadRequest",
  "message": "The request is invalid or missing required parameters."
}
```

```json
{
  "error": "NotFound",
  "message": "The requested resource was not found."
}
```

```json
{
  "error": "ServerError",
  "message": "An internal server error occurred. Please try again later."
}
```

</details>

<details>
  <summary>POST /reencrypt ---- Decrypt a ciphertext under an ephemeral key s.t. a client can learn the decrypted value privately.</summary>

#### Description

This end-point returns a JSON object containing a signcryption of the plaintext value of an FHE ciphertext that has been (obliviously) decrypted by the TKMS. 
More specifically the TKMS servers carry out a partial decryption resulting in each of them knowing a secret share of the plaintext (meaning that multiple servers need to maliciously collude in order to learn the decrypted plaintext). They each then signcrypt their share of the plaintext. The response consists of each of these signcryptions along with meta information about the threshold setup and which server provides each signcrypted share of the result. 
Since the signcryption is based on secret sharing it means that only a subset, specifically >1/3 of the total responses, is required to recover the result (assuming all returned signcryptions are correct). 

#### Query Parameters

Multiple parameters must be supplied in JSON format:
- `signature`: A hex (lower-case) encoded EIP712 signature on the public encryption key, `enc_key`, under which the of the ciphertext in question will be reencrypted. 
- `client_address`: An EIP-55 encoded address (that is, including the `0x` prefix) of the end-user who is supposed to learn the reencrypted response.
- `enc_key`: The hex (lower-case) encoded public encryption key (libsodium) which the reencryption should be signcrypted under.
- `ciphertext_handle`: The 32 byte (lower-case) hex encoded handle/ID identifying the ciphertext and hence allowing the gateway to fetch it.
- `eip712_verifying_contract`: An EIP-55 encoded address (that is, including the `0x` prefix) of the contract holding the ciphertext to reencrypt.
```json
{
     "signature": "15a4f9a8eb61459cfba7d103d8f911fb04ce91ecf841b34c49c0d56a70b896d20cbc31986188f91efc3842b7df215cee8acb40178daedb8b63d0ba5d199bce121c",
     "client_address": "0x17853A630aAe15AED549B2B874de08B73C0F59c5",
     "enc_key": "2000000000000000df2fcacb774f03187f3802a27259f45c06d33cefa68d9c53426b15ad531aa822",
     "ciphertext_handle": "0748b542afe2353c86cb707e3d21044b0be1fd18efc7cbaa6a415af055bfb358",
     "eip712_verifying_contract": "0x66f9664f97F2b50F62D13eA064982f936dE76657"
}
```

#### Headers

None.

#### Response

**Success (200 OK)**

The request is successful, and the response will include a JSON object with a `status` and a `response` which is a list of responses from each TKMS. More specifically each element in the list consists of the following:
- `payload`: A bincode encoding of the signcryption from a single server along with meta information including the server ID, threshold parameter, type of value encrypted and the specific public key of the server supplying the specific response. 
- `signature`: An EIP712 signature which is hex (low-case) encoded.

For example the following:
```json
{
    "response": [
        {
            "payload": "161c5...",
            "signature": "15a4f9a8eb61459cfba7d103d8f911fb04ce91ecf841b34c49c0d56a70b896d20cbc31986188f91efc3842b7df215cee8acb40178daedb8b63d0ba5d199bce121c"
        },
        {
            "payload": "44546...",
            "signature": "118165165165423465234414c4c468a4d9684d8e18186d6f786161b4b436c58787cc68418186d6f786161b4b98461166a6a6668e8e118542c154867aab238abd79"
        },
        {
            "payload": "54cd5...",
            "signature": "1c849848940128065242121b2b12121ed876986da251561650c654564d684654e51610879a9a9798b78b78e7f8787d87c8c8894a454547809586616161464cc8a8"
        },
        {
            "payload": "a516b...",
            "signature": "10c864ac145423466798808098c098b09a8908d6432da4544f5e54566b76740454654a54c65454565d65d657e44651241561342441234128888063304854897893"
        },
    ],
    "status": "success"
}
```

**Error Responses**

| Status Code | Error Code   | Description                                      |
| ----------- | ------------ | ------------------------------------------------ |
| 400         | `BadRequest` | The request is invalid or missing required parameters. |
| 404         | `NotFound`   | The requested resource was not found.            |
| 500         | `ServerError` | An internal server error occurred.                   |

#### Example Error Responses

```json
{
  "error": "BadRequest",
  "message": "The request is invalid or missing required parameters."
}
```

```json
{
  "error": "NotFound",
  "message": "The requested resource was not found."
}
```

```json
{
  "error": "ServerError",
  "message": "An internal server error occurred. Please try again later."
}
```

</details>