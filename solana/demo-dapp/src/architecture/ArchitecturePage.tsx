import mermaid from 'mermaid';
import { useEffect } from 'react';

type DiagramFrame = {
  readonly title: string;
  readonly statement: string;
  readonly diagram: string;
};

type GlossaryFrame = {
  readonly title: string;
  readonly statement: string;
  readonly terms: readonly {
    readonly term: string;
    readonly definition: string;
  }[];
};

type ArchitectureFrame = DiagramFrame | GlossaryFrame;

const frames: readonly ArchitectureFrame[] = [
  {
    title: 'First principle',
    statement: 'Applications compute on encrypted values without exposing them.',
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
        Authorized{"Who may learn it?"}
        Authorized -->|"one user"| User["User decrypt"]
        Authorized -->|"the program"| Public["Certified public decrypt"]
    end

    Ciphertext --> Rules
    Result --> Authorized
`,
  },
  {
    title: 'Shared key',
    statement: 'One public key connects encrypted applications; no single party holds the secret.',
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
    title: 'Protocol glossary',
    statement: 'Each protocol layer has one job.',
    terms: [
      {
        term: 'SDK',
        definition: 'Encrypts inputs, builds requests and opens user-decrypt results.',
      },
      {
        term: 'Host contract / program',
        definition: 'Records encrypted operations and enforces who may use their results.',
      },
      {
        term: 'Relayer',
        definition: 'Carries SDK requests to protocol services and returns their results.',
      },
      {
        term: 'Gateway',
        definition: 'Keeps encrypted material and decryption work synchronized across the protocol.',
      },
      {
        term: 'Coprocessor',
        definition: 'Reconstructs confirmed operations and computes them over ciphertexts.',
      },
      {
        term: 'KMS',
        definition: 'Checks decryption rights and combines secret-key shares to release a result.',
      },
    ],
  },
  {
    title: 'Solana glossary',
    statement: 'Solana gives every rule and encrypted value a concrete execution path.',
    terms: [
      {
        term: 'Program',
        definition: 'Executes code; mutable state lives in separate accounts.',
      },
      {
        term: 'PDA',
        definition: 'Gives a program a deterministic address with no private key.',
      },
      {
        term: 'CPI',
        definition: 'Lets one program call another with the authority already present.',
      },
      {
        term: 'Handle',
        definition: 'Identifies one ciphertext and its metadata; it is not the ciphertext.',
      },
      {
        term: 'EncryptedValue',
        definition: 'Keeps the current handle, permissions and compact history at one stable PDA.',
      },
      {
        term: 'MMR',
        definition: 'Commits old permissions into compact, append-only history.',
      },
    ],
  },
  {
    title: 'Complete system',
    statement: 'Solana records the rules; encrypted services execute the work.',
    diagram: String.raw`
flowchart LR
    subgraph browser["Browser"]
        Dapp["Demo dApp"]
        Wallet["Phantom"]
        Dapp <--> Wallet
    end

    subgraph solana["Solana"]
        Validator["Validator"]
        Programs["Vault · Batcher · Token · Host"]
        State["Accounts and PDAs"]
        Validator --> Programs --> State
    end

    subgraph services["Encrypted services"]
        Yellowstone["Yellowstone"]
        Listener["Host listener"]
        Compute["Encrypted compute"]
        Storage["Encrypted data"]
        Proof["History proofs"]
        Relayer["Request service"]
        Keys["Key service"]
        Yellowstone --> Listener --> Compute --> Storage
        Proof --> Keys
        Relayer <--> Keys
    end

    Dapp --> Validator
    Validator --> Yellowstone
    Dapp --> Relayer
    State -. authorization .-> Proof
`,
  },
  {
    title: 'Vault design',
    statement: 'The batcher hides each amount and reveals only the total.',
    diagram: String.raw`
flowchart LR
    Alice["Alice<br/>encrypted amount"] --> Batch["Confidential batcher"]
    Bob["Bob<br/>encrypted amount"] --> Batch
    Batch -->|"public total"| Vault["Public vault"]
    Vault -->|"normal shares"| Batch
    Batch -->|"encrypted allocation"| AliceShares["Alice cShares"]
    Batch -->|"encrypted allocation"| BobShares["Bob cShares"]
`,
  },
  {
    title: 'Shield',
    statement: 'Public USDC enters custody and becomes private spending power.',
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
    statement: 'A verified encrypted amount moves into the private batch.',
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
    statement: 'The account stays fixed while its encrypted value changes.',
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
    statement: 'Confirmed Solana instructions become encrypted computation jobs.',
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
    statement: 'A proven batch total crosses into the public vault.',
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
    title: 'History',
    statement: 'Compact history keeps old decrypt rights verifiable.',
    diagram: String.raw`
flowchart LR
    H1["Handle H1"] --> L1["Alice could access H1"]
    H2["Handle H2"] --> L2["Alice could access H2"]
    H3["Current handle H3"] --> Value["EncryptedValue PDA"]
    L1 --> MMR["MMR peaks on Solana"]
    L2 --> MMR
    MMR --> Proof["Small inclusion proof"]
    Proof --> Check["Verified before decryption"]
`,
  },
  {
    title: 'Claim',
    statement: 'Public ratios allocate encrypted shares without revealing deposits.',
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
    statement: 'The wallet authorizes a value that opens only in the browser.',
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
    statement: 'Encrypted shares return through the same private doorway.',
    diagram: String.raw`
flowchart LR
    Shares["User cShares"] -->|"encrypted amount"| Batch["Redeem batch"]
    Batch -->|"public share total"| Vault["Public vault"]
    Vault -->|"public USDC payout"| Batch
    Batch -->|"encrypted allocation"| Cusdc["User cUSDC"]
`,
  },
  {
    title: 'Trust boundary',
    statement: 'The private path is real; the keys, assets, vault and yield source are local.',
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
          {'diagram' in frame ? (
            <div className="diagram-shell">
              <pre className="mermaid">{frame.diagram}</pre>
            </div>
          ) : (
            <dl className="glossary-grid">
              {frame.terms.map(({ term, definition }) => (
                <div className="glossary-entry" key={term}>
                  <dt>{term}</dt>
                  <dd>{definition}</dd>
                </div>
              ))}
            </dl>
          )}
        </section>
      ))}
    </main>
  );
}
