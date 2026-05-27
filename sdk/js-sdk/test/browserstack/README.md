# BrowserStack Tests

## Setup

```bash
export BROWSERSTACK_USERNAME=<username>
export BROWSERSTACK_ACCESS_KEY=<key>
export VITE_ZAMA_API_KEY=<api-key>
export VITE_MNEMONIC=<mnemonic>
```

## Run

```bash
npm run bstack:encrypt          # no secrets needed
npm run bstack:public-decrypt   # needs VITE_ZAMA_API_KEY
npm run bstack:user-decrypt     # needs VITE_ZAMA_API_KEY + VITE_MNEMONIC
npm run bstack:verify-input     # needs VITE_ZAMA_API_KEY + VITE_MNEMONIC
```
