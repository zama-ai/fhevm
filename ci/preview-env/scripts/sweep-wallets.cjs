// Return whatever is left in the preview's HD wallets to the treasury, before the
// namespace (and with it the only copy of the mnemonic) is deleted.
// Env: MNEMONIC, CHAINS_JSON [{label,rpcUrl,chainId,treasury}], HD_INDICES (csv),
// DUST_WEI (skip balances not worth a transfer). Never logs keys.
// Best-effort by design: a failure here must never block teardown, so every error
// is reported and swallowed.
const { ethers } = require('ethers');

const requireEnv = (name) => {
  const value = (process.env[name] || '').trim();
  if (!value) throw new Error(`${name} is required`);
  return value;
};

const mnemonic = ethers.Mnemonic.fromPhrase(requireEnv('MNEMONIC'));
const chains = JSON.parse(requireEnv('CHAINS_JSON'));
const indices = requireEnv('HD_INDICES')
  .split(',')
  .map((i) => parseInt(i.trim(), 10))
  .filter((i) => Number.isInteger(i));
// A transfer costs 21000 gas; below that plus a margin the sweep would cost more
// than it returns, so leave the dust behind.
const dustWei = BigInt(process.env.DUST_WEI || '1000000000000000'); // 0.001

const wallets = indices.map((index) => ({
  index,
  wallet: ethers.HDNodeWallet.fromMnemonic(mnemonic, `m/44'/60'/0'/0/${index}`),
}));

async function sweepChain(chain) {
  const provider = new ethers.JsonRpcProvider(chain.rpcUrl, Number(chain.chainId), {
    staticNetwork: true,
  });
  let recovered = 0n;
  try {
    const fee = await provider.getFeeData();
    // Drain to the last wei we can: whatever the node might charge for the transfer
    // is maxFeePerGas * 21000, and EIP-1559 refunds the unused part to the treasury
    // anyway, so reserving the ceiling never strands more than the refund.
    const gasPrice = fee.maxFeePerGas ?? fee.gasPrice;
    if (!gasPrice) throw new Error('node returned no fee data');
    const reserve = gasPrice * 21000n;

    for (const { index, wallet } of wallets) {
      try {
        const balance = await provider.getBalance(wallet.address);
        if (balance <= reserve + dustWei) {
          if (balance > 0n) {
            console.log(
              `${chain.label} #${index} ${wallet.address}: ${ethers.formatEther(balance)} - below sweep threshold, leaving`,
            );
          }
          continue;
        }
        const value = balance - reserve;
        const signer = wallet.connect(provider);
        const tx = await signer.sendTransaction({
          to: chain.treasury,
          value,
          gasLimit: 21000n,
        });
        const receipt = await tx.wait(1);
        if (!receipt || receipt.status !== 1) {
          throw new Error(`transfer reverted (tx ${tx.hash})`);
        }
        recovered += value;
        console.log(
          `${chain.label} #${index} ${wallet.address}: returned ${ethers.formatEther(value)} tx=${tx.hash}`,
        );
      } catch (err) {
        console.log(`::warning::${chain.label} #${index} sweep failed: ${err.message}`);
      }
    }
    console.log(`${chain.label}: recovered ${ethers.formatEther(recovered)} to ${chain.treasury}`);
  } finally {
    provider.destroy();
  }
}

(async () => {
  for (const chain of chains) {
    try {
      await sweepChain(chain);
    } catch (err) {
      console.log(`::warning::${chain.label} sweep skipped: ${err.message}`);
    }
  }
})();
