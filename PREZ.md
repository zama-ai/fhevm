# Why a new `@fhevm/sdk`?

## 1. Modular architecture — pay only for what you use

- **Monolithic → composable**: a small core that you extend with the actions you need
- **Purpose-built clients**: `encrypt-only`, `decrypt-only`, `base` (EIP-712 only), or full
- **Tree-shakable**: scenarios that don't need heavy crypto stay lightweight in the bundle
- **Lazy initialization**: WASM modules load on first use, not at client creation

## 2. Developer experience — built for app developers, not protocol engineers

- **TypeScript-first**: strict typings, `readonly` parameters, generics that infer the FHE type
- **Clearer API surface**: `EncryptedValue` instead of `handle`, consistent `<verb><Noun>` naming, Zama's internal plumbing hidden behind the public actions
- **Web3-library agnostic**: identical API for `ethers.js v6` and `viem` — pick your stack, the SDK adapts
- **Zero-config defaults**: built-in chain definitions, sensible threading, works out of the box

## 3. WASM — flexible distribution, verifiable integrity

- **Two loading strategies**: URL-based (fetch `.wasm` from any origin you control) or base64-embedded (no external requests)
- **Self-hostable**: deploy with or without external URLs to fit your network/security policy
- **Hash-validated binaries**: every WASM file is verified against a pinned hash before execution

---

## 🎁 One more thing… jsDelivr out of the box

- **Zero infrastructure**: point the SDK at jsDelivr and ship — no `.wasm` to host, no build step
- **Versioned & immutable**: every release is pinned to a content-addressed URL on the CDN
- **Still verified**: hash validation runs on the CDN payload too — trust the URL, prove the bytes

- Add a bullet that says it supports cleartext out of the box with no architecture sacrifice -> it comes straight from the modulare new architeture
