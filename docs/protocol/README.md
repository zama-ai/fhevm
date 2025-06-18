---
description: >-
  FHEVM is a technology that enables confidential smart contracts on the EVM using Fully Homomorphic Encryption (FHE).
layout:
  title:
    visible: true
  description:
    visible: true
  tableOfContents:
    visible: true
  outline:
    visible: true
  pagination:
    visible: false
---

# Welcome to FHEVM

## Why FHEVM?

<table data-view="cards"><thead><tr><th></th><th></th><th data-hidden data-card-cover data-type="files"></th></tr></thead><tbody><tr><td><strong>Guard Privacy</strong></td><td>Keep data encrypted during onchain computation.</td><td></td></tr><tr><td><strong>Build Fast</strong></td><td>Use Solidity, SDKs, templates, etc.</td><td></td></tr><tr><td><strong>Deploy Anywhere</strong></td><td>Compatible with all EVM chains.</td><td></td></tr></tbody></table>

<a href="./#path-to-build" class="button primary">Start Building</a>

## Explore the doc

<table data-view="cards"><thead><tr><th></th><th></th><th data-hidden data-card-cover data-type="files"></th><th data-hidden data-card-target data-type="content-ref"></th></tr></thead><tbody><tr><td><strong>Solidity Guides</strong></td><td>Write encrypted logic with Solidity tools.</td><td><a href=".gitbook/assets/Zama-Gitbook_Cover_Test.png">Gitbook-cover.png</a></td><td><a href="https://docs.zama.ai/doc-ui/IoEzE96rh6dKmIRhgam5/solidity-guides">Solidity</a></td></tr><tr><td><strong>SDK Guides</strong></td><td>Build frontends with encrypted user data.</td><td><a href=".gitbook/assets/Zama-Gitbook_Cover_Test.png">Gitbook-cover.png</a></td><td><a href="https://docs.zama.ai/doc-ui/IoEzE96rh6dKmIRhgam5/sdk-guides">SDK</a></td></tr><tr><td><strong>Examples</strong></td><td>Explore real dApps and code templates.</td><td><a href=".gitbook/assets/Zama-Gitbook_Cover_Test.png">Gitbook-cover.png</a></td><td><a href="/examples">Examples</a></td></tr></tbody></table>

## Path to Build

{% stepper %} {% step %}

### **Set up your development environment**

Use the official Hardhat template from Zama that includes all necessary configurations and dependencies to start
developing confidential smart contracts.

<a href="https://github.com/zama-ai/fhevm-hardhat-template" class="button secondary">Clone the template</a>
{% endstep %}

{% step %}

### Write your first confidential smart contract

Use the provided contract examples, like `MyCounter`, to begin. A basic counter contract might look like this:

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { FHE, euint64 } from "@fhevm/solidity/lib/FHE.sol";
import { SepoliaConfig } from "@fhevm/solidity/config/ZamaConfig.sol";

contract MyCounter is ZamaConfig {
  euint64 counter;
  constructor() {
    counter = FHE.asEuint64(0);
  }

  function add() public {
    counter = FHE.add(counter, 1);
    FHE.allowThis(counter);
  }
}
```

<a href="https://docs.zama.ai/doc-ui/IoEzE96rh6dKmIRhgam5/solidity-guides/get-started/overview" class="button primary">See
the full Solidity guide</a> {% endstep %}

{% step %}

### Build your frontend with `@zama-ai/relayer-sdk`

Start from Zama's ready-to-use React template.

<a href="https://docs.zama.ai/doc-ui/IoEzE96rh6dKmIRhgam5/sdk-guides" class="button primary">See the full SDK guide</a>
<a href="https://github.com/zama-ai/fhevm-react-template" class="button secondary">Clone the template</a> ;
{% endstep %}

{% step %}

### To continue

<table data-view="cards"><thead><tr><th></th><th></th><th data-hidden data-card-cover data-type="files"></th><th data-hidden data-card-target data-type="content-ref"></th></tr></thead><tbody><tr><td><strong>Architecture</strong></td><td>Understand how the system fits together.</td><td></td><td><a href="https://docs.zama.ai/doc-ui/IoEzE96rh6dKmIRhgam5/architecture/architecture_overview">Broken link</a></td></tr><tr><td><strong>Dapps Demos</strong></td><td>Try real projects built with fhevm.</td><td></td><td></td></tr><tr><td><strong>White paper</strong></td><td>Dive deep into the cryptographic design.</td><td></td><td></td></tr></tbody></table>
{% endstep %}
{% endstepper %}

## Help Center

Ask technical questions and discuss with the community.

- [Community forum](https://community.zama.ai/c/fhevm/15)
- [Discord channel](https://discord.com/invite/fhe-org)
- [Telegram](https://t.me/+Ojt5y-I7oR42MTkx)
