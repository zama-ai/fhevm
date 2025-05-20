# Reencryption


Reencryption is performed on the client side by calling the gateway service using the [fhevmjs](https://github.com/zama-ai/fhevmjs/) library. To do this, you need to provide a view function that returns the ciphertext to be reencrypted.

1. The dApp retrieves the ciphertext from the view function (e.g., balanceOf).
2. The dApp generates a keypair for the user and requests the user to sign the public key.
3. The dApp calls the gateway, providing the ciphertext, public key, user address, contract address, and the user's signature.
4. The dApp decrypts the received value with the private key.