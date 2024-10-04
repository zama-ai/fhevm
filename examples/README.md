> Here will go all information about specific contracts, how are they designed and what are the considerations

### Overview of Encrypted and Decrypted Data Flow

```mermaid
graph TD
    A[User's Plaintext Input] -->|Encryption| B(Encrypted Input)
    B -->|Stored in Contract| C{Encrypted State Variables}

    %% Using encrypted data in the contract %%
    C -->|Operations on Encrypted Data| D[Contract Logic and Functions]
    D -->|Maintain Confidentiality| C

    %% Encryption and Decryption operations %%
    D -->|Decrypt when Necessary| E[Decrypted Values for Computations]
    E -->|Logic/Verification| D
```

[needs work]
```mermaid
graph TD
    %% User Interaction %%
    subgraph User Actions
        U[User] -->|1. Encrypted Input| A[User's Plaintext Input]
        U -->|2. Request Re-encryption| F[Direct Re-encryption Request]
        U -->|3. Trigger Decryption| K[Decryption Request]
    end

    %% Encryption Flow %%
    A -->|Sends Input to Contract| B(Encrypted Input)
    B -->|Stored in Contract| C{Encrypted State Variables}

    %% Using encrypted data in the contract %%
    C -->|Operations on Encrypted Data| D[Contract Logic and Functions]
    D -->|Maintain Confidentiality| C

    %% Decryption operations %%
    D -->|Decrypt when Necessary| E[Decrypted Values for Computations]
    E -->|Logic/Verification| D
    K -->|Decrypt Request to Contract| C
    C -->|Forwards Request to| G[Gateway Service]
    G -->|Return Decrypted Value| E
    G <--> N{KMS Service}

    %% Re-encryption Flow %%
    F -->|User Calls Gateway Directly| G
    G -->|Re-encrypt with dApp's Public Key| J[Re-encrypted Data]
    J -->|Return to User| M{User's Encrypted Data}
    M -->|User Sends to Contract| C

```

```mermaid
graph TD
    %% User Interaction %%
    subgraph User Actions
        U[User] -->|1. Encrypted Input| A[User's Plaintext Input]
        U -->|2. Request Re-encryption| F[Direct Re-encryption Request]
        U -->|3. Trigger Decryption| K[Decryption Request]
    end

    %% Smart Contract Operations %%
    subgraph Smart Contract Operations
        C{Encrypted State Variables} -->|Operations on Encrypted Data| D[Contract Logic and Functions]
        D -->|Maintain Confidentiality| C
        A -->|Sends Input to Contract| B(Encrypted Input)
        B -->|Stored in Contract| C
        K -->|Decrypt Request to Contract| C
        C -->|Forwards Decrypt Request to| GC[Gateway Call from Contract]
        J -->|Return to User| M{User's Encrypted Data}
        %% Contract Decryption Flow %%
        GC -->|Return Decrypted Data to Contract| E[Decrypted Values for Computations]
        E -->|Logic/Verification| D

    end

        GC -->|Forward Request to| G[Gateway Service]
        G -->|Return Decrypted Value| GC
        G <--> N{KMS Service}
        F -->|User Calls Gateway Directly| G
        G -->|Re-encrypt with dApp's Public Key| J[Re-encrypted Data]

    %% Re-encrypted Data Flow %%
    M -->|User Sends to Contract| C



```


# Counter.sol
The `Counter` smart contract is a simple contract implemented in Solidity, designed to demonstrate basic state manipulation. Its main purpose is to maintain a counter (value) and provide functions to increment and view the current counter value.

## EncryptedERC20.sol
This contract implements an encrypted ERC20-like token with confidential balances using Zama's FHE (Fully Homomorphic Encryption) library.
It supports typical ERC20 functionality such as transferring tokens, minting, and setting allowances, but uses encrypted data types.

### Approval and Transfer Operations

Here's a high-level overview of what is encrypted and decrypted in the `EncryptedERC20` smart contract:

```mermaid
graph TD
    subgraph User Inputs
        X1(Encrypted Amount)
        X2(Encrypted Allowance)
    end
    subgraph Contract Logic
        Y1[Check Allowance & Balance]
        Y2[Update Encrypted Allowance]
        Y3[Transfer Encrypted Amount]
    end
    X1 --> Y1
    X2 --> Y1
    Y1 --> Y2
    Y1 --> Y3
```

