---
description: >-
  fhevm is a technology that enables confidential smart contracts on the EVM
  using Fully Homomorphic Encryption (FHE).
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

# Welcome to fhevm

## Why fhevm?

<table data-view="cards"><thead><tr><th></th><th></th><th data-hidden data-card-cover data-type="files"></th></tr></thead><tbody><tr><td><strong>Guard Privacy</strong></td><td>Keep data encrypted during onchain computation.</td><td></td></tr><tr><td><strong>Build Fast</strong></td><td>Use Solidity, SDKs, templates, etc.</td><td></td></tr><tr><td><strong>Deploy Anywhere</strong></td><td>Compatible with all EVM chains.</td><td></td></tr></tbody></table>

<a href="./#path-to-build" class="button primary">Start Building</a>

## Explore the doc

<table data-view="cards"><thead><tr><th></th><th></th><th data-hidden data-card-cover data-type="files"></th><th data-hidden data-card-target data-type="content-ref"></th></tr></thead><tbody><tr><td><strong>Solidity Guides</strong></td><td>Write encrypted logic with Solidity tools.</td><td><a href=".gitbook/assets/Zama-Gitbook_Cover_Test.png">Gitbook-cover.png</a></td><td><a href="https://app.gitbook.com/o/-MIF05xPVoj0l_wnOGB7/s/mIaweK9iiWMF773uuVPP/">Solidity</a></td></tr><tr><td><strong>SKD Guides</strong></td><td>Build frontends with encrypted user data.</td><td><a href=".gitbook/assets/Zama-Gitbook_Cover_Test.png">Gitbook-cover.png</a></td><td><a href="https://app.gitbook.com/o/-MIF05xPVoj0l_wnOGB7/s/2wDODARNL7cfrn1fsPcS/">SDK</a></td></tr><tr><td><strong>Examples</strong></td><td>Explore real dApps and code templates.</td><td><a href=".gitbook/assets/Zama-Gitbook_Cover_Test.png">Gitbook-cover.png</a></td><td><a href="https://app.gitbook.com/o/-MIF05xPVoj0l_wnOGB7/s/hBX9KO0dtJnFLDRNYCZX/">Examples</a></td></tr></tbody></table>

## Path to Build

{% stepper %}
{% step %}
### **Set up your development environment**

Use the official Hardhat template from Zama that includes all necessary configurations and dependencies to start developing confidential smart contracts.

<a href="https://github.com/zama-ai/fhevm-hardhat-template" class="button secondary">Clone the template</a>
{% endstep %}

{% step %}
### **Install dependencies**

Navigate into your project directory and run:

```solidity
npm install
```

Also, install the fhevm Solidity library:

```solidity
npm install fhevm-contracts
```
{% endstep %}

{% step %}
### Write your first confidential smart contract

Use the provided contract examples, like `ConfidentialERC20`, to begin. A basic confidential token contract might look like this:

```solidity
// SPDX-License-Identifier: BSD-3-Clause-Clear
pragma solidity ^0.8.24;

import { SepoliaZamaFHEVMConfig } from "fhevm/config/ZamaFHEVMConfig.sol";
import { ConfidentialERC20 } from "fhevm-contracts/contracts/token/ERC20/ConfidentialERC20.sol";

contract MyERC20 is SepoliaZamaFHEVMConfig, ConfidentialERC20 {
    constructor() ConfidentialERC20("MyToken", "MYTOKEN") {
        _unsafeMint(1000000, msg.sender);
    }
}
```

<a href="https://app.gitbook.com/o/-MIF05xPVoj0l_wnOGB7/s/mIaweK9iiWMF773uuVPP/" class="button primary">See the full Solidity guide</a>
{% endstep %}

{% step %}
### Build your frontend with `fhevmjs`

Start from Zama's ready-to-use React template.

<a href="https://app.gitbook.com/o/-MIF05xPVoj0l_wnOGB7/s/2wDODARNL7cfrn1fsPcS/" class="button primary">See the full SDK guide</a>    <a href="https://github.com/zama-ai/fhevm-react-template" class="button secondary">Clone the template</a>   ;
{% endstep %}

{% step %}
### To continue

<table data-view="cards"><thead><tr><th></th><th></th><th data-hidden data-card-cover data-type="files"></th><th data-hidden data-card-target data-type="content-ref"></th></tr></thead><tbody><tr><td><strong>Architecture</strong></td><td>Understand how the system fits together.</td><td></td><td><a href="broken-reference">Broken link</a></td></tr><tr><td><strong>Dapps Demos</strong></td><td>Try real projects built with fhevm.</td><td></td><td></td></tr><tr><td><strong>White paper</strong></td><td>Dive deep into the cryptographic design.</td><td></td><td></td></tr></tbody></table>
{% endstep %}
{% endstepper %}

## Help Center

Ask technical questions and discuss with the community.

* [Community forum](https://community.zama.ai/c/fhevm/15)
* [Discord channel](https://discord.com/invite/fhe-org)
* [Telegram](https://t.me/+Ojt5y-I7oR42MTkx)
