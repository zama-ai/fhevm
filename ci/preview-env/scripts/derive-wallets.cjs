// Derive COUNT deterministic HD wallets from the well-known Foundry/Hardhat
// mnemonic (indices HD_OFFSET..HD_OFFSET+COUNT-1) and append
// `<OUTPUT_KEY>=[{party,address,privateKey}]` to $GITHUB_OUTPUT.
// CommonJS so require('ethers') can be satisfied via NODE_PATH.
const { ethers } = require('ethers');
const fs = require('fs');

const mnemonic = ethers.Mnemonic.fromPhrase(
  'test test test test test test test test test test test junk',
);
const n = parseInt(process.env.COUNT, 10);
const offset = parseInt(process.env.HD_OFFSET, 10);
const outputKey = process.env.OUTPUT_KEY || 'wallets_json';
const label = process.env.LABEL || 'tx-sender';

const wallets = [];
for (let i = 0; i < n; i++) {
  const party = i + 1;
  const path = `m/44'/60'/0'/0/${offset + i}`;
  const wallet = ethers.HDNodeWallet.fromMnemonic(mnemonic, path);
  wallets.push({ party, address: wallet.address, privateKey: wallet.privateKey });
}
console.log(
  `Derived ${n} ${label} wallets:`,
  wallets.map((w) => `party ${w.party}: ${w.address}`).join(', '),
);
fs.appendFileSync(process.env.GITHUB_OUTPUT, `${outputKey}=` + JSON.stringify(wallets) + '\n');
