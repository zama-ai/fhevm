
# Gateway API Specifications

## Endpoints

<details>
  <summary>GET /keys ---- Retrieve public key, CRS (for input proof) and bootstrap key files download URLs</summary>

#### Description

This endpoint returns a JSON object containing URLs from an S3 bucket. These URLs allow the client to download the blockchain public key, the CRS  files needed to compute input proofs, and the bootstrap key. There is no need for parameters because the gateway is already linked to a specific blockchain.

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
  "publicKey": ["https://s3.amazonaws.com/bucket-name/pks"],
  "crs": {
    "2048": ["https://s3.amazonaws.com/bucket-name/crs"]
  },
  "bootstrapKey": ["https://s3.amazonaws.com/bucket-name/sks"]
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
  "message": "The specified key_id does not exist."
}
```

```json
{
  "error": "ServerError",
  "message": "An internal server error occurred. Please try again later."
}
```

</details>

