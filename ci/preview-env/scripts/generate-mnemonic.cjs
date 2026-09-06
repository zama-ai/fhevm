// Generate a fresh BIP-39 mnemonic and derive the fixed preview-env role
// indices (#0 gateway deployer, #3 relayer/tx-sender, #9 host/ACL owner,
// #1-#4 test signers) from it. Addresses only are logged; keys go to
// GITHUB_OUTPUT / a JSON file (never echo private keys).
const { ethers } = require('ethers');
const fs = require('fs');

const wallet = ethers.Wallet.createRandom();
if (!wallet.mnemonic) {
  throw new Error('ethers.Wallet.createRandom() did not produce a mnemonic');
}
const phrase = wallet.mnemonic.phrase;
const mnemonic = ethers.Mnemonic.fromPhrase(phrase);

const roleIndices = [0, 1, 2, 3, 4, 9];
const roles = {};
for (const index of roleIndices) {
  const w = ethers.HDNodeWallet.fromMnemonic(mnemonic, `m/44'/60'/0'/0/${index}`);
  roles[index] = { index, address: w.address, privateKey: w.privateKey };
}

console.log(
  'Generated preview mnemonic; role addresses:',
  roleIndices.map((i) => `#${i}=${roles[i].address}`).join(', '),
);

const outJson = process.env.ROLES_JSON_PATH || '/tmp/preview-role-wallets.json';
fs.writeFileSync(outJson, JSON.stringify({ mnemonic: phrase, roles }, null, 2));

if (process.env.GITHUB_OUTPUT) {
  const append = (name, value) => {
    fs.appendFileSync(process.env.GITHUB_OUTPUT, `${name}<<EOF\n${value}\nEOF\n`);
  };
  append('mnemonic', phrase);
  fs.appendFileSync(process.env.GITHUB_OUTPUT, `roles_json_path=${outJson}\n`);
  fs.appendFileSync(process.env.GITHUB_OUTPUT, `addr_0=${roles[0].address}\n`);
  fs.appendFileSync(process.env.GITHUB_OUTPUT, `addr_3=${roles[3].address}\n`);
  fs.appendFileSync(process.env.GITHUB_OUTPUT, `addr_9=${roles[9].address}\n`);
}

if (process.env.GITHUB_ENV) {
  fs.appendFileSync(process.env.GITHUB_ENV, `MNEMONIC=${phrase}\n`);
  fs.appendFileSync(process.env.GITHUB_ENV, `DEPLOYER_KEY_0=${roles[0].privateKey}\n`);
  fs.appendFileSync(process.env.GITHUB_ENV, `DEPLOYER_KEY_3=${roles[3].privateKey}\n`);
  fs.appendFileSync(process.env.GITHUB_ENV, `DEPLOYER_KEY_9=${roles[9].privateKey}\n`);
}
