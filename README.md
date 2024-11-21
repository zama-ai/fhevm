<p align="center">
<!-- product name logo -->
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="./assets/Zama-KMS-fhEVM-dark.png">
  <source media="(prefers-color-scheme: light)" srcset="./assets/Zama-KMS-fhEVM-light.png">
  <img width=600 alt="Zama fhEVM & KMS">
</picture>
</p>

---

<p align="center">
  <a href="./fhevm-whitepaper.pdf"> 📃 Read white paper</a> |<a href="https://docs.zama.ai/fhevm"> 📒 Documentation</a> | <a href="https://zama.ai/community"> 💛 Community support</a> | <a href="https://github.com/zama-ai/awesome-zama"> 📚 FHE resources by Zama</a>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-BSD--3--Clause--Clear-%23ffb243?style=flat-square"></a>
  <a href="https://github.com/zama-ai/bounty-program"><img src="https://img.shields.io/badge/Contribute-Zama%20Bounty%20Program-%23ffd208?style=flat-square"></a>
</p>

> [!Note]
> To try out the Q2 release of our fhEVM-native demo, use the [release/0.5.x branch](https://github.com/zama-ai/fhevm-devops/tree/release/0.5.x).

> [!Warning]
> This demo is an early beta version.

## Table of Contents

- **[About](#about)**
- **[Running the demo](#running-the-demo)**
- **[License](#license)**
- **[FAQ](#faq)**
- **[Support](#support)**

## About

This repository provides a docker based setup to locally run an integration of fhEVM blockchain and Zama KMS (Key Management System).

For overview of the system, architecture and details on individual components, refer to our [documentation](https://docs.zama.ai/fhevm-backend).

fhEVM blockchain comes in two variants:

- Native
- Coprocessor

KMS can be configured to two modes:

- Central
- Threshold

Possible configurations in the demo:

- fhEVM-Native variant supports running KMS in only central mode. Support for threshold mode is coming in next release.
- fhEVM-Coprocessor variant supports running KMS in both central and threshold modes (with 4 parties).

## Running the demo

- For fhEVM Native:

  - Switch to the [release/0.5.x branch](https://github.com/zama-ai/fhevm-devops/tree/release/0.5.x) and follow the steps in the README.
  - This is our Q2 2024 release.
  - [native](native) directory on `main` branch is being actively updated for next release.

- For fhEVM Coprocessor
  - Switch to the [coprocessor](coprocessor) directory on `main` branch and follow the steps in the README.
  - This is the most up to date version.

## License

This software is distributed under the **BSD-3-Clause-Clear** license. Read [this](LICENSE.txt) for more details.

## FAQ

**Is Zama’s technology free to use?**

> Zama’s libraries are free to use under the BSD 3-Clause Clear license only for development, research, prototyping, and experimentation purposes. However, for any commercial use of Zama's open source code, companies must purchase Zama’s commercial patent license.
>
> Everything we do is open source and we are very transparent on what it means for our users, you can read more about how we monetize our open source products at Zama in [this blog post](https://www.zama.ai/post/open-source).

**What do I need to do if I want to use Zama’s technology for commercial purposes?**

> To commercially use Zama’s technology you need to be granted Zama’s patent license. Please contact us hello@zama.ai for more information.

**Do you file IP on your technology?**

> Yes, all Zama’s technologies are patented.

**Can you customize a solution for my specific use case?**

> We are open to collaborating and advancing the FHE space with our partners. If you have specific needs, please email us at hello@zama.ai.

## Support

<a target="_blank" href="https://community.zama.ai">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/zama-ai/tfhe-rs/assets/157474013/08656d0a-3f44-4126-b8b6-8c601dff5380">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/zama-ai/tfhe-rs/assets/157474013/1c9c9308-50ac-4aab-a4b9-469bb8c536a4">
  <img alt="Support">
</picture>
</a>

🌟 If you find this project helpful or interesting, please consider giving it a star on GitHub! Your support helps to grow the community and motivates further development.
