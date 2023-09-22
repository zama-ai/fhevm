# Metamask

Here are the main steps from the [official guide](https://support.metamask.io/hc/en-us/articles/360043227612-How-to-add-a-custom-network-RPC) provided by Metamask:

<figure><img src=".gitbook/assets/metamask_add_network.gif" alt=""><figcaption>
1) From the homepage of your wallet, click on the network selector in the top left, and then on 'Add network'
</figcaption></figure>

<figure><img src=".gitbook/assets/metamask_add_network2.gif" alt=""><figcaption>
2) MetaMask will open in a new tab in fullscreen mode. From here, find and the 'Add network manually' button at the bottom of the network list.</figcaption></figure>

Add these informations to access to blockchain
{% tabs %}
{% tab title="Zama devnet" %}

| Fields                        | Value                         |
| ----------------------------- | ----------------------------- |
| Network Name                  | Zama Network                  |
| New RPC URL                   | https://devnet.zama.ai        |
| Chain ID                      | 8009                          |
| Currency symbol               | ZAMA                          |
| Block explorer URL (Optional) | https://main.explorer.zama.ai |

{% endtab %}
{% tab title="Local devnet" %}

| Fields                        | Value                  |
| ----------------------------- | ---------------------- |
| Network Name                  | Zama Local             |
| New RPC URL                   | http://localhost:8545/ |
| Chain ID                      | 9000                   |
| Currency symbol               | ZAMA                   |
| Block explorer URL (Optional) |                        |

{% endtab %}
{% endtabs %}
