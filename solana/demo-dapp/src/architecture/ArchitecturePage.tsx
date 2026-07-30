import mermaid from 'mermaid';
import { useEffect } from 'react';

type ArchitectureFrame = {
  readonly title: string;
  readonly statement: string;
  readonly diagram: string;
};

const frames: readonly ArchitectureFrame[] = [
  {
    title: 'Encrypted computation',
    statement: 'Applications compute on encrypted values.',
    diagram: String.raw`
flowchart TB
    subgraph input["Input"]
        direction LR
        Value["User value"] --> Encrypt["SDK encrypts"] --> Ciphertext["Ciphertext"]
    end

    subgraph execution["Execution"]
        direction LR
        Rules["Solana enforces<br/>rules and permissions"] --> Compute["Coprocessor computes<br/>over ciphertexts"] --> Result["Encrypted result"]
    end

    subgraph release["Release"]
        direction LR
        Authorized{"Decryption scope"}
        Authorized -->|"one user"| User["User decrypt"]
        Authorized -->|"the program"| Public["Certified public decrypt"]
    end

    Ciphertext --> Rules
    Result --> Authorized
`,
  },
  {
    title: 'Shared network key',
    statement: 'Applications encrypt with one shared public key.',
    diagram: String.raw`
flowchart LR
    PublicKey["Network public key"] --> AppA["App A encrypts"]
    PublicKey --> AppB["App B encrypts"]
    AppA --> Shared["Compatible ciphertexts"]
    AppB --> Shared
    Shared --> Compose["Encrypted results<br/>move between apps"]

    subgraph kms["Production key service"]
        Share1["Secret share 1"]
        Share2["Secret share 2"]
        Share3["Secret share 3"]
        Quorum["Several parties<br/>must approve"]
        Share1 --> Quorum
        Share2 --> Quorum
        Share3 --> Quorum
    end

    Compose --> Quorum
    Quorum --> Decrypt["Authorized result"]
`,
  },
  {
    title: 'Protocol architecture',
    statement: 'Requests move between the user, host chain and encrypted services.',
    diagram: String.raw`
flowchart TB
    subgraph client["User side"]
        direction LR
        User["User"] --> SDK["SDK<br/>encrypts inputs and opens results"]
        SDK --> Relayer["Relayer<br/>carries requests and results"]
    end

    subgraph hostchain["Host chain"]
        Host["Host contract / program<br/>records operations and permissions"]
    end

    subgraph network["Encrypted network"]
        direction LR
        Coprocessor["Coprocessor<br/>computes over ciphertexts"]
        Gateway["Gateway<br/>coordinates encrypted material<br/>and decryption work"]
        KMS["KMS<br/>checks rights and releases<br/>authorized results"]
        Coprocessor --> Gateway
    end

    SDK -->|"encrypted transaction"| Host
    Host -->|"confirmed operations"| Coprocessor
    Relayer <-->|"request / protected result"| Gateway
    Gateway <-->|"authorized decrypt"| KMS
`,
  },
  {
    title: 'Solana architecture',
    statement: 'A cUSDC deposit updates three accounts across three programs.',
    diagram: String.raw`
flowchart TB
    subgraph action["One atomic transaction"]
        direction TB
        Wallet["Wallet approves<br/>Deposit 100 cUSDC"]
        Batcher["Batcher validates<br/>deposit eligibility"]
        Token["Confidential token moves<br/>the encrypted amount"]
        Host["Zama host records<br/>the encrypted calculation"]

        Wallet --> Batcher
        Batcher -->|"program call · CPI"| Token
        Token -->|"requests encrypted arithmetic"| Host
    end

    subgraph state["Updated accounts"]
        direction TB
        UserBalance["cUSDC balance<br/>encrypted"]
        JoinedAmount["Joined deposit<br/>encrypted"]
        BatchState["Batch status<br/>public"]
    end

    Token --> UserBalance
    Batcher --> JoinedAmount
    Batcher --> BatchState
`,
  },
  {
    title: 'Shield',
    statement: 'Shielding locks USDC and credits an encrypted cUSDC balance.',
    diagram: String.raw`
sequenceDiagram
    participant Wallet
    participant Token as Confidential token
    participant SPL as SPL Token
    participant Host as Zama host

    Wallet->>Token: Shield public USDC
    Token->>SPL: Move USDC into wrapper custody
    Token->>Host: Encrypt the public amount
    Token->>Host: Add to encrypted balance and supply
    Host-->>Token: New balance and supply handles
`,
  },
  {
    title: 'Join',
    statement: 'Joining transfers encrypted cUSDC into the current batch.',
    diagram: String.raw`
sequenceDiagram
    participant Browser
    participant Input as Input verification
    participant Wallet
    participant Batch as Batcher
    participant Token as Confidential token
    participant Host as Zama host

    Browser->>Input: Encrypted deposit amount
    Input-->>Browser: Signed verification
    Wallet->>Batch: Join batch
    Batch->>Token: Confidential transfer by CPI
    Token->>Host: Verify input and record operations
    Host-->>Batch: New encrypted joined amount
`,
  },
  {
    title: 'Encrypted state',
    statement: 'One EncryptedValue account tracks the current handle and permission history.',
    diagram: String.raw`
flowchart LR
    Value["EncryptedValue PDA<br/>stable logical balance"]
    Value --> Current["Current handle H3"]
    Value --> Subjects["Allowed users and programs"]
    Value --> Peaks["Compact history peaks"]
    H1["Old handle H1"] --> Peaks
    H2["Old handle H2"] --> Peaks
    Current --> Material["Encrypted material<br/>stored off-chain"]
`,
  },
  {
    title: 'Compute pipeline',
    statement: 'The listener converts confirmed host instructions into coprocessor jobs.',
    diagram: String.raw`
flowchart LR
    Tx["Confirmed transaction"] --> Yellowstone["Yellowstone stream"]
    Yellowstone --> Listener["Host listener"]
    Listener --> Job["Reconstructed computation"]
    Job --> Worker["Encrypted compute worker"]
    Worker --> Ciphertext["Encrypted result"]
    Worker --> Material["Gateway material record"]
`,
  },
  {
    title: 'Public settlement',
    statement: 'Settlement decrypts the batch total and deposits it into the vault.',
    diagram: String.raw`
sequenceDiagram
    participant Batch as Batcher
    participant Host as Zama host
    participant Proof as Proof service
    participant Keys as Key service
    participant Settle as Solana settlement
    participant Vault

    Batch->>Host: Burn encrypted batch balance
    Host->>Host: Release the exact burned handle
    Proof-->>Keys: History inclusion proof
    Keys-->>Settle: Clear total and signed certificate
    Settle->>Settle: Verify proof and certificate
    Settle->>Vault: Deposit public batch total
`,
  },
  {
    title: 'Claim',
    statement: 'A claim computes and transfers each encrypted share allocation.',
    diagram: String.raw`
flowchart LR
    Deposit["Encrypted user deposit"] --> Formula["deposit × batch shares ÷ batch total"]
    Shares["Public batch shares"] --> Formula
    Total["Public batch total"] --> Formula
    Formula --> Claim["Encrypted user allocation"]
    Claim --> Transfer["Confidential transfer"]
    Transfer --> Balance["User cShares"]
`,
  },
  {
    title: 'User decrypt',
    statement: 'User decryption returns plaintext only to the authorized browser.',
    diagram: String.raw`
sequenceDiagram
    participant Browser
    participant Wallet as Phantom
    participant Relayer as Request service
    participant Keys as Key service

    Browser->>Browser: Create fresh transport key
    Wallet->>Browser: Sign decryption request
    Browser->>Relayer: Signed request and public key
    Relayer->>Keys: Check Solana authorization
    Keys-->>Relayer: Signcrypted response
    Relayer-->>Browser: Opaque response
    Browser->>Browser: Recover the clear value
`,
  },
  {
    title: 'Redeem',
    statement: 'Redemption converts encrypted cShares back into encrypted cUSDC.',
    diagram: String.raw`
flowchart LR
    Shares["User cShares"] -->|"encrypted amount"| Batch["Redeem batch"]
    Batch -->|"public share total"| Vault["Public vault"]
    Vault -->|"public USDC payout"| Batch
    Batch -->|"encrypted allocation"| Cusdc["User cUSDC"]
`,
  },
  {
    title: 'POC boundary',
    statement: 'The POC runs real protocol components with local keys, assets and yield.',
    diagram: String.raw`
flowchart LR
    subgraph real["Runs for real"]
        Transactions["Solana transactions"]
        Programs["On-chain programs"]
        Encryption["Encrypted computation"]
        Authorization["Authorization proofs"]
        Decryption["User and public decrypt"]
        Accounting["Vault accounting"]
    end

    subgraph poc["POC setting"]
        Localnet["Local validator"]
        TestAssets["Test assets and keys"]
        Centralized["One key service"]
        ToyVault["Toy public vault"]
        Keeper["Local keeper"]
        Yield["Demo-funded yield"]
    end

    Transactions --- Localnet
    Programs --- ToyVault
    Encryption --- TestAssets
    Authorization --- Keeper
    Decryption --- Centralized
    Accounting --- Yield
`,
  },
];

const frameNumber = (index: number): string => String(index + 1).padStart(2, '0');

export function ArchitecturePage() {
  useEffect(() => {
    void mermaid.run({ querySelector: '.mermaid', suppressErrors: false }).catch((error: unknown) => {
      document.body.dataset.mermaidError = 'true';
      console.error('Failed to render architecture diagrams', error);
    });
  }, []);

  return (
    <main className="architecture-page">
      {frames.map((frame, index) => (
        <section className="architecture-frame" id={`frame-${index + 1}`} key={frame.title}>
          <header className="frame-heading">
            <p className="frame-label">
              {frameNumber(index)} / {frames.length} · {frame.title}
            </p>
            <h1>{frame.statement}</h1>
          </header>
          <div className="diagram-shell">
            <pre className="mermaid">{frame.diagram}</pre>
          </div>
        </section>
      ))}
    </main>
  );
}
