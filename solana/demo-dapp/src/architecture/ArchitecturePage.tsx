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
flowchart LR
    subgraph input["Input"]
        direction TB
        Value["User value"] --> Encrypt["SDK<br/>encrypts inputs"] --> Ciphertext["Ciphertext"]
    end

    subgraph execution["Execution"]
        direction TB
        Host["Host program<br/>records operations and permissions"] --> Compute["Coprocessor<br/>computes over ciphertexts"] --> Result["Encrypted result"]
    end

    subgraph release["Release"]
        direction TB
        Gateway["Gateway<br/>coordinates encrypted material<br/>and decryption work"]
        KMS["KMS<br/>checks rights and releases results"]
        Authorized{"Decryption scope"}
        Relayer["Relayer<br/>returns a protected user result"]
        Browser["Browser opens result"]

        Gateway --> KMS --> Authorized
        Authorized -->|"one user"| Relayer --> Browser
        Authorized -->|"the program"| Public["Certified public decrypt"]
    end

    Ciphertext --> Host
    Result --> Gateway
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
    title: 'Encrypted execution',
    statement: 'Host instructions define handles; the coprocessor materializes their ciphertexts.',
    diagram: String.raw`
flowchart TB
    subgraph solana["Solana"]
        direction LR
        Tx["Confirmed host instruction"]
        Value["EncryptedValue PDA<br/>permissions and history"]
        Handle["Current handle"]
        Tx --> Value --> Handle
    end

    subgraph services["Encrypted services"]
        direction LR
        Yellowstone["Yellowstone stream"]
        Listener["Host listener"]
        Worker["Encrypted compute"]
        Material["Encrypted material"]
        Yellowstone --> Listener --> Worker --> Material
    end

    Tx --> Yellowstone
    Value --> History["Old permissions<br/>MMR peaks"]
    Handle -.->|"identifies"| Material
`,
  },
  {
    title: 'Settle and claim',
    statement: 'The public vault receives the batch total; encrypted shares are allocated per deposit.',
    diagram: String.raw`
flowchart TB
    subgraph settlement["Settle public total"]
        direction LR
        Burn["Batcher burns<br/>encrypted batch balance"]
        Proof["Proof service proves<br/>the released handle"]
        KMS["KMS returns total<br/>and signed certificate"]
        Solana["Solana verifies<br/>proof and certificate"]
        Vault["Vault deposits total<br/>and returns public shares"]
        Burn --> Proof --> KMS --> Solana --> Vault
    end

    subgraph claim["Claim encrypted shares"]
        direction LR
        Deposit["Encrypted deposit"]
        Formula["deposit × shares ÷ total"]
        Allocation["Encrypted cShares allocation"]
        Deposit --> Formula --> Allocation
    end

    Vault -->|"public shares"| Formula
    KMS -->|"public total"| Formula
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
