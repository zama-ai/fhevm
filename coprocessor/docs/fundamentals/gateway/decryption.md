# Decryption

Everything in fhEVM is encrypted, at some point one could need to decrypt somes values. Let's give as illustration a blind auction application.
After reaching the end of the auction, one need to discover (only) the winner, here is where a asynchronous decrypt could appear. 


> :warning: **Decryption is public**: It means everyone will be able to see the value. If this is a personal information see [Reencryption](./reencryption.md)

## How it's working

The Gateway acts as an oracle service: it will listen to decryption request events and return the decrypted value through a callback function.
The responsabilities of the Gateway are:
- Listening decryption request from fhEVM that contains a handle `h` that corresponds to a  ciphertext `C`
- Computing a storage proof `P` to attest h (i.e. C)  is decryptable
- Retrieve C from fhEVM using `h` as key
- Send a decyption request to TKMS which in turn is running an internal blockchain aka `KMS BC`
- Wait and listen for `decyptionResponse` (containing the plaitext and a few signatures from KMS to attest the integrity of the palintext) event from `KMS BC`
- Return `decyptionResponse` through the callback function

## High level overview of the decryption flow 

We allow explicit decryption requests for any encrypted type. The values are decrypted with the network private key.

![](asyncDecrypt.png)






