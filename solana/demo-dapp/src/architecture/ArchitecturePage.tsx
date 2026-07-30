import mermaid from 'mermaid';
import { useEffect } from 'react';

type ArchitectureFrame = {
  readonly title: string;
  readonly statement: string;
  readonly diagram: string;
};

const frames: readonly ArchitectureFrame[] = [
  {
    title: 'Protocol overview',
    statement: 'Zama end-to-end flow',
    diagram: String.raw`
sequenceDiagram
    box User device
        participant Client as Wallet + SDK
    end
    box Solana
        participant Host as Host program
    end
    box Protocol services
        participant Copro as Coprocessor
        participant Relayer
        participant Gateway
        participant KMS as KMS network
    end

    Note over Client,Copro: Input proof and attestation
    Client->>Relayer: Encrypted input and proof
    Relayer->>Gateway: Verification request
    Gateway->>Copro: Input proof
    Copro->>Copro: Verify input proof
    Copro-->>Gateway: Ciphertext attestation
    Gateway-->>Relayer: Attestation
    Relayer-->>Client: Attestation

    Note over Client,Gateway: On-chain encrypted computation
    Client->>Host: Encrypted action and attestation
    Host->>Host: Verify coprocessor attestation and record operation
    Host-->>Copro: Confirmed operation
    Copro-->>Gateway: FHE result

    Note over Client,KMS: Authorized decryption
    Client->>Relayer: Signed decryption request
    Relayer->>Gateway: Decryption request
    Gateway->>KMS: Authorized decrypt
    KMS-->>Gateway: User-protected result
    Gateway-->>Relayer: Protected result
    Relayer-->>Client: Protected result
    Client->>Client: Recover plaintext
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
    box User device
        participant Wallet
    end
    box Solana
        participant Token as Confidential token
        participant SPL as SPL Token
        participant Host as Zama host
    end

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
    box User device
        participant Wallet
        participant SDK
    end
    box Solana
        participant Batch as Batcher
        participant Token as Confidential token
        participant Host as Zama host
    end

    SDK->>Wallet: Build join with encrypted amount and attestation
    Wallet->>Batch: Sign and submit join
    Batch->>Token: Confidential transfer by CPI
    Token->>Host: Verify coprocessor attestation inside transfer
    Token->>Host: Record encrypted operations
    Host-->>Token: New balance handles
    Token-->>Batch: New encrypted joined amount
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
    statement: 'The wallet authorizes the request; the SDK recovers the plaintext.',
    diagram: String.raw`
sequenceDiagram
    box User device
        participant Wallet as Phantom
        participant SDK
    end
    box Protocol services
        participant Relayer
        participant Gateway
        participant KMS as KMS network
    end

    SDK->>SDK: Create fresh transport key
    SDK->>Wallet: Request authorization
    Wallet-->>SDK: Signed decryption request
    SDK->>Relayer: Signed request and public key
    Relayer->>Gateway: Locate encrypted result
    Gateway->>KMS: Check authorization and decrypt
    KMS-->>Gateway: Result protected for SDK
    Gateway-->>Relayer: Protected result
    Relayer-->>SDK: Protected result
    SDK->>SDK: Recover plaintext
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
