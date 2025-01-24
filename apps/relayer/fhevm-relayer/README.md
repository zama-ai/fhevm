# fhevm-relayer

- Use traits to define interfaces to external systems

- For polyporphism
  - Use generics

- core and traits as lib
- ethereum coprocessor adaptger as a lib (imports traits)
- solana coprocessor adaptger as a lib (imports traits)

- one gateway: import ethereum + core + traits
- second gateway: import solana + core + traits

- test suite
  - for each adapter (all expectations from adapter interface)
  - for core gateway logic

- concurrency model
  - tokio + async/await
  - for end point
    - axum:
      - light wrapper, better support tonic
      - developed by tokio team
      - more open
    - actix-web:
      - more features
      - better performance
      - been there for long
  - tracing/logging 
    - integrated in axum
    - use conf-trace as defined in kms-core
  - testing
    - tokio-test ?
    - rs-test ?
  - core testing (defining mocked adapters)
    - will be suggested by maksym
    
- Notes on persistence
  - purpose: crash recovery
  - persist after and before each network request

