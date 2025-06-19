{% stepper %}

<!-- Step 1 -->
{% step %}
## Install a Node.js TLS version

Make sure **Node.js** is installed on your machine. If it isn’t, download and install the recommended LTS (Long-Term Support) version from the official site [https://nodejs.org](https://nodejs.org).

You can verify your installation by running:

```sh
node -v
npm -v
```

Ensure that you are using an even-numbered Node.js version, as these correspond to LTS releases.

{% hint style="warning" %}
**Hardhat** does not support odd-numbered Node.js versions. If you’re using one (e.g., v21.x, v23.x), Hardhat will display a persistent warning message and may behave unexpectedly.
{% endhint %}
{% endstep %}
<!-- End Step 1 -->

<!-- Step 2 -->
{% step %}
## Create a new GitHub repository from the FHEVM Hardhat template.

1. On GitHub, navigate to the main page of the [FHEVM Hardhat template](https://github.com/zama-ai/fhevm-hardhat-template) repository.
2. Above the file list, click the green **Use this template** button.
3. Follow the instructions to create a new repository from the FHEVM Hardhat template.

{% hint style="info" %}
📘 See Github doc: [Creating a repository from a template](https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template#creating-a-repository-from-a-template)
{% endhint %}
{% endstep %}
<!-- End Step 2 -->

<!-- Step 3 -->
{% step %}
## Clone your newly created GitHub repository locally

Now that your GitHub repository has been created, you can clone it to your local machine:

```sh
cd <your-preferred-location>
git clone <url-to-your-new-repo>

# Navigate to the root of your new FHEVM Hardhat project
cd <your-new-repo-name>
```

{% hint style="success" %}
**🎉 Congratulations!** you've successfully created your own FHEVM Hardhat project based on the [FHEVM Hardhat template](https://github.com/zama-ai/fhevm-hardhat-template).
{% endhint %}

Next, let’s install your local Hardhat development environment.
{% endstep %}
<!-- End Step 3 -->

<!-- Step 4 -->
{% step %}
## Install your FHEVM Hardhat project dependencies

From the project root directory, run:

```sh
npm install
```

This will install all required dependencies defined in your `package.json`, setting up your local FHEVM Hardhat development environment.
{% endstep %}
<!-- End Step 4 -->

<!-- Step 5 -->
{% step %}
## (Optional) Set up the [Hardhat Configuration Variables](https://hardhat.org/hardhat-runner/docs/guides/configuration-variables).

This step is optional — you can skip it if you don't plan to deploy or interact with your FHEVM contracts on Sepolia
Ethereum Testnet.

If you do plan to deploy to the Sepolia Ethereum Testnet, you'll need to set up the following Hardhat Configuration
Variables:

- `MNEMONIC`
- `INFURA_API_KEY`

---

### 🔐 Don’t have these credentials?

- `MNEMONIC` — Generate one using [Metamask](https://metamask.io) or your favorite mnemonic generator.
- `INFURA_API_KEY` — Follow the [Infura + Metamask guide](https://docs.metamask.io/services/get-started/infura/) to obtain your key.

---

### 🛠 How to Set the Configuration Variables

Set the `MNEMONIC`

```sh
npx hardhat vars set MNEMONIC
```

Set the `INFURA_API_KEY`

```sh
npx hardhat vars set INFURA_API_KEY
```

---

### 📦 Default Values (if not set)

If you skip this step, Hardhat will fall back to these defaults:

- `MNEMONIC` = "test test test test test test test test test test test junk" 
- `INFURA_API_KEY` = "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz"

⚠️ These defaults are not suitable for real deployments.

---

{% hint style="warning" %}
## ❌ Hardhat Error Message (Missing Variables)

If any of the requested **Hardhat Configuration** Variables is missing, you'll get an error message like this one:
`Error HH1201: Cannot find a value for the configuration variable 'MNEMONIC'. Use 'npx hardhat vars set MNEMONIC' to set it or 'npx hardhat vars setup' to list all the configuration variables used by this project.`
{% endhint %}
{% endstep %}
<!-- End Step 5 -->

<!-- Step 6 -->
{% step %}
## (Optional) Set up VSCode with recommanded extensions

To improve your development experience, you can install the following Visual Studio Code extensions:
- [Prettier - Code formatter by prettier.io](https://marketplace.visualstudio.com/items?itemName=esbenp.prettier-vscode) — ID:`esbenp.prettier-vscode`,
- [ESLint by Microsoft](https://marketplace.visualstudio.com/items?itemName=dbaeumer.vscode-eslint) — ID:`dbaeumer.vscode-eslint`

To add Solidity support to VSCode, choose one of the following Solidity extensions:
- [solidity by Juan Blanco](https://marketplace.visualstudio.com/items?itemName=JuanBlanco.solidity) — ID:`juanblanco.solidity`

or
  
- [Solidity by Nomic Foundation](https://marketplace.visualstudio.com/items?itemName=NomicFoundation.hardhat-solidity)  — ID:`nomicfoundation.hardhat-solidity`
  
If you install both of them, VSCode will raise an error and ask you to pick only one.
{% endstep %}
<!-- End Step 6 -->

{% endstepper %}
