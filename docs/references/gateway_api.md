# Gateway API Specifications

## Endpoints

<details>
  <summary>GET /keys ---- Retrieve public key, CRS (for input proof) and bootstrap key files download URLs</summary>

#### Description

This endpoint returns a JSON object containing URLs from an S3 bucket, allowing the client to download key files such as the blockchain public key, CRS files for input proof generation, and the bootstrap key.

For each file, a list of cryptographic signatures is provided to ensure the integrity and authenticity of the downloaded content. These signatures are generated using a threshold signature scheme. This means that instead of needing all the signatures to validate the content, only a subset—specifically one-third of the total signatures (if n nodes are signing)—is required to verify that the content is legitimate. 

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
    "keyId": "ab12cd",
    "crsId": "34ef67",
    "publicKey": {
        "url": "https://s3.amazonaws.com/bucket-name/pubkey",
        "signatures": [
            "a5d2...",
            "a8cd...",
            "c0ff..."
        ]
    },
    "bootstrapKey": {
        "url": "https://s3.amazonaws.com/bucket-name/bootstrapkey",
        "signatures": [
            "fd7a...",
            "487b...",
            "20fe..."
        ]
    },
    "crs": {
        "2048": {
            "url": "https://s3.amazonaws.com/bucket-name/crs",
            "signatures": [
                "ffee...",
                "012f...",
                "1ab5..."
            ]
        }
    }
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

