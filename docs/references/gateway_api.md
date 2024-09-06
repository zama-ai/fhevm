# Gateway API Specifications

## Endpoints

<details>
  <summary>GET /keys ---- Retrieve public key, CRS (for input proof) and bootstrap key files download URLs</summary>

#### Description

This endpoint returns a JSON object containing URLs from an S3 bucket, allowing the client to download important files such as the blockchain public key, CRS files for generating input proofs, and the bootstrap key. 

For each item, multiple URLs are provided to improve availability, but all URLs point to the same file. This ensures redundancy and reliable access. 

Each URL is accompanied by a cryptographic signature, enabling users to verify the integrity and authenticity of the downloaded files. 

No query parameters are needed, as the gateway is already configured for a specific blockchain.



#### Query Parameters

No parameters.

#### Headers

None.

#### Response

**Success (200 OK)**

The request is successful, and the response will include a JSON object with the following structure:

```json
{
  "keyId": "12345",
  "publicKey": [
    {
      "url": "https://s3.amazonaws.com/bucket-name/pks1",
      "signature": "a5d2c5e8f7b60f..."
    },
    {
      "url": "https://s3.amazonaws.com/bucket-name1/pks1",
      "signature": "a5d2c5e8f7b60f..."
    }
  ],
  "crs": {
    "2048": [
      {
        "url": "https://s3.amazonaws.com/bucket-name/crs1",
        "signature": "c3f5d6e1e2f8b3..."
      },
      {
        "url": "https://s3.amazonaws.com/bucket-name1/crs1",
        "signature": "c3f5d6e1e2f8b3..."
      }
    ]
  },
  "bootstrapKey": [
    {
      "url": "https://s3.amazonaws.com/bucket-name/sks1",
      "signature": "b4a1d8f9c7e1a2..."
    },
    {
      "url": "https://s3.amazonaws.com/bucket-name1/sks1",
      "signature": "b4a1d8f9c7e1a2..."
    }
  ]
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

