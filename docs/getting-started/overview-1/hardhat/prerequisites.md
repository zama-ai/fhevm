# Prerequisites

This guide will walk you through installing all necessary dependencies from scratch, preparing you for developing and deploying fhEVM smart contracts using Hardhat.

To use Hardhat to build and deploy confidential smart contract, you need to have the following:

- **Node.js (v20 or later)**
- **A package manager**: `npm`, `yarn`, or `pnpm`
- **Git** for version control
- **Foundry** for using the command `cast`

If you already have these installed, skip ahead to [1. Setting Up Hardhat.](1.-setting-up-hardhat.md)

---

## Step 1: Open your terminal

To run the installation commands, open a terminal:

- Use your system's built-in terminal (e.g., Terminal on macOS, Command Prompt or PowerShell on Windows, or a Linux shell).
- Alternatively, use the integrated terminal in a IDE such as [Visual Studio Code (VS Code)](https://code.visualstudio.com/).

{% hint style="info" %}
VS code provide the official [Hardhat extension](https://hardhat.org/hardhat-vscode) for an enhanced development experience with Hardhat.
{% endhint %}

## Step 2. Install Node.js and a package manager&#x20;

Instead of installing Node.js directly, we recommend using Node Version Manager (`nvm`), which allows you to manage multiple versions of Node.js:

1. **Install `nvm`** following the recommended tutorials:
   - [Linux](https://tecadmin.net/how-to-install-nvm-on-ubuntu-20-04/)
   - [Windows ](https://github.com/coreybutler/nvm-windows/blob/master/README.md#installation--upgrades)
   - [Mac OS](https://tecadmin.net/install-nvm-macos-with-homebrew/)
2. **Install Node.js** using nvm. This installs the latest version of Node.js:

```
nvm install node
```

3. **Install pnpm** (optional but recommended for better speed and efficiency). After installing Node.js, you automatically have npm, so run:

```
npm install -g pnpm
```

&#x20; Alternatively, you can continue using `npm` or `yarn` if you prefer.

## Step 3. Install Git

Git is required for managing source code, cloning repositories, and handling version control. Install Git based on your operating system:

- **Linux**: Use your distribution's package manager (e.g., `sudo apt-get install git` on Ubuntu/Debian).
- **Windows**: [Download Git for Windows](https://git-scm.com/download/win).
- **macOS**: Install via [Homebrew](https://brew.sh/) with `brew install git` or [download Git](https://git-scm.com/download/mac).

## Step 4. Install Foundry

We will need the command `cast` in our tutorial. This command is available with installing Foundry.

1. **Install Foundry** using the official installer:

```bash
curl -L https://foundry.paradigm.xyz | bash
```

2. **Reload your terminal configuration**:

```bash
source ~/.bashrc  # Linux/WSL
# or
source ~/.zshrc   # macOS/zsh
```

3. **Run the Foundry installer**:

```bash
foundryup
```

This will install all Foundry components, including `cast`, `forge`, `anvil`, and `chisel`.

To verify the installation, run:

```bash
cast --version
```

## Next Steps

You now have all the necessary dependencies installed. Continue to [**1. Setting Up Hardhat**](1.-setting-up-hardhat.md) to begin creating and deploying your FHE smart contracts!
