# Gateway API Specifications

## Endpoints

<details>
  <summary>GET /keyurl ---- Retrieve links for retrieving the public keys and CRS' in the system.</summary>

#### Description

This endpoint returns a JSON object containing URLs from an S3 bucket, allowing the client to download key files such as the blockchain public key, CRS files for input proof generation, the bootstrap key, and the address and public verification keys for each of the MPC servers running the TKMS.

For each file (with the exception of the verification key and address), a list of cryptographic signatures is provided to ensure the integrity and authenticity of the downloaded content. These signatures are generated using a threshold signature scheme. This means that instead of needing all the signatures to validate the content, only a subset, specifically >1/3 of the total signatures (if n nodes are signing), is required to verify that the content is legitimate. 

No query parameters are required, as the gateway is already preconfigured for a specific blockchain.



#### Query Parameters

No parameters.

#### Headers

None.

#### Response

**Success (200 OK)**

The request is successful, and the response will include a JSON object with the following structure:

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

