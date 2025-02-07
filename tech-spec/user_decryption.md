# User Decryption

[HTTPZ Flow](https://github.com/zama-ai/tech-spec/blob/main/architecture/user_decryption.md)

## Flow

- Dapp user calls Dapp backend (web2 or web3) to get an authorization to decrypt a given ciphertext
    - Must be something that we verify
- Dapp user calls ZWS http relayer endpoint with both authorization + ciphertext handle + public key, that sends a message through SNS to the orchestrator
    - We’ll need to implement the defined relayer spec here
    - We’ll probably need to implement this with 2 enpoints post-request/get-result to avoid maintaining the connexion for two long or just cache the query and get spammed by the user until the whole flow succeeded
- ZWS orchestrator has to retrieve which contract whitelisted said ciphertext and verify that it is in fact whitelist on ZWS.
    - if not then break flow here
    - we may have an issue with composability here
- Then the following is more or less the same as the public-decryption but with the result cached in the HTTP relayer service (with some expiry date)
