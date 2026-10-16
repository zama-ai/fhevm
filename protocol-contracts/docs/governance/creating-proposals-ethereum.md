# Creating Ethereum Proposals

Proposals are created and voted on via the Aragon App UI:

- [Mainnet DAO](https://app.aragon.org/dao/ethereum-mainnet/zama.dao.eth/dashboard)
- [Testnet DAO](https://app.aragon.org/dao/ethereum-sepolia/0x08e8a84c3c8c7cba165B1adcf67Ae4639eF84f52/dashboard)
- [Devnet DAO](https://app.aragon.org/dao/ethereum-sepolia/0x41d84D9F00263eaF80f3526C157CD49c263CAd59/dashboard)

**Related guides:**
- [Creating cross-chain (remote) proposals](creating-proposals-remote.md): how to create and submit cross-chain proposals to EVM destinations
- [Reviewing proposals](reviewing-proposals.md): how to verify proposals before approving
- [CLI reference](cli-reference.md): detailed CLI tool documentation

---

## Step 1: Initialize the proposal

1. Open the Aragon DAO dashboard and click **Proposals**.

![Aragon dashboard](images/aragon-dashboard.png)

2. Click **+ Proposal**.

![New proposal button](images/aragon-new-proposal.png)

3. Select the proposal type:
   - **Operators**: requires 9/17 operator approvals.

![Proposal type selection](images/aragon-proposal-type.png)

4. Connect your wallet and fill in:
   - **Title**: short description of the proposal
   - **Summary**: one-line summary
   - **Body**: detailed description. Voters should be able to verify the proposal on their own or in the context of linked resources
   - **Resources**: links to help reviewers understand the proposal. Examples:
      - [protocol-registry repo](https://github.com/zama-ai/protocol-registry)

![Proposal details form 1](images/aragon-proposal-details-1.png)
![Proposal details form 2](images/aragon-proposal-details-2.png)

## Step 2: Add actions

1. Enter the target contract address and wait for all checks to turn green.
   - Exception: simple transfers or Aragon DAO–related actions are directly available.

![Add contract address](images/aragon-add-contract.png)

![Contract checks green](images/aragon-contract-checks.png)

2. Pick the function to call from the dropdown.

![Select function](images/aragon-select-function.png)

3. Fill in function arguments.

> **Warning:** ⚠️ Do not use quotes around string arguments (unlike Remix IDE).

![Function arguments](images/aragon-function-args.png)

4. To add more actions, click **+ Action**.

## Step 3: Simulate and submit

1. Click **Next**.
2. Click **Simulate** to verify actions won't revert on execution.

![Simulate actions](images/aragon-simulate.png)

3. Submit the proposal and notify:
   - First code owners
   - Then DAO members
