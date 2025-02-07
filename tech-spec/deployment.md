# Deployment

ZWS Console deployment requirements.

## Oracle contract

The ZWS Console needs a stable address for its Public Decryption Oracle Contract.
This can be achieved with a proxy contract.

The Oracle Contract needs to hold one or multiple public keys, that ZWS can update, against which to verify contract whitelisting.

It should be callable by a user to register their contract.
