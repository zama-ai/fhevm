# Using Remix

This document provides guidance on using the new Zama plugin for the [the official Remix IDE](https://remix.ethereum.org), which replaces the deprecated Remix fork. This allows you to develop and manage contracts directly in Remix by simply loading the fhEVM plugin.

## Installing the Zama plugin

1. Go to the "Plugin Manager" page
2. Click on "Connect to a Local Plugin"
3. Fill the name with "Zama" and the "Url" with "https://remix.zama.ai/"
4. Keep "Iframe" and "Side panel" and validate

<figure><img src="../../.gitbook/assets/remixide.png" alt="How to install Remix IDE plugin" width="300"><figcaption></figcaption></figure>

## Configuring the Zama plugin

After connecting to the Zama Plugin, follow the steps to configure it:

1. Click on the plugin button located on the left of the screen
2. Add a Gateway URL to be able to request reencryption of ciphertexts, as shown in the picture below.

The default recommended Gateway URL is: `https://gateway.devnet.zama.ai`.

<figure><img src="../../.gitbook/assets/useGateway.png" alt="How to install Remix IDE plugin" width="300"><figcaption></figcaption></figure>

Afterwards, you will be able to deploy and use any contract that you chose to compile via this plugin interface.
