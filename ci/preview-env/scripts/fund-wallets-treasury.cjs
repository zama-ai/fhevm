// Top up preview EOAs on public testnets from one treasury key, from the runner (public RPCs reachable).
// Env: FUNDER_PRIVATE_KEY, CHAINS_JSON [{label,rpcUrl,chainId}], ADDRESSES (newline-separated),
// DEPLOYER_ADDRESS (gets DEPLOYER_FLOOR_WEI), FLOOR_WEI. Never logs the key.
const { ethers } = require('ethers');

const requireEnv = (name) => {
  const value = (process.env[name] || '').trim();
  if (!value) throw new Error(`${name} is required`);
  return value;
};

const funderKey = requireEnv('FUNDER_PRIVATE_KEY');
const chains = JSON.parse(requireEnv('CHAINS_JSON'));
const addresses = requireEnv('ADDRESSES')
  .split('\n')
  .map((a) => a.trim())
  .filter(Boolean);
const deployerAddress = requireEnv('DEPLOYER_ADDRESS').toLowerCase();
// 0.2 per test signer; 1.0 for #9, which pays ~10 upgradeable contracts + keygen per chain.
const floorWei = BigInt(process.env.FLOOR_WEI || '200000000000000000');
const deployerFloorWei = BigInt(process.env.DEPLOYER_FLOOR_WEI || '1000000000000000000');

if (addresses.length === 0) throw new Error('ADDRESSES is empty');

const floorFor = (addr) => (addr.toLowerCase() === deployerAddress ? deployerFloorWei : floorWei);

async function fundChain(chain) {
  const provider = new ethers.JsonRpcProvider(chain.rpcUrl, Number(chain.chainId), { staticNetwork: true });
  const funder = new ethers.Wallet(funderKey, provider);
  try {
    const net = await provider.getNetwork();
    if (Number(net.chainId) !== Number(chain.chainId)) {
      throw new Error(`${chain.label}: RPC reports chainId ${net.chainId}, expected ${chain.chainId}`);
    }
    // Plan before sending so a short treasury fails with nothing spent.
    const plan = [];
    for (const addr of addresses) {
      const balance = await provider.getBalance(addr);
      const floor = floorFor(addr);
      const shortfall = balance < floor ? floor - balance : 0n;
      console.log(`${chain.label} ${addr} balance=${ethers.formatEther(balance)} floor=${ethers.formatEther(floor)} topup=${ethers.formatEther(shortfall)}`);
      if (shortfall > 0n) plan.push({ addr, shortfall });
    }
    const total = plan.reduce((acc, p) => acc + p.shortfall, 0n);
    const funderBalance = await provider.getBalance(funder.address);
    // 21k gas x 100 gwei per transfer: a deliberately pessimistic gas reserve.
    const gasReserve = BigInt(plan.length) * 21000n * 100000000000n;
    console.log(`${chain.label} treasury ${funder.address} balance=${ethers.formatEther(funderBalance)} needed=${ethers.formatEther(total + gasReserve)} (${plan.length} transfers)`);
    if (funderBalance < total + gasReserve) {
      throw new Error(`${chain.label}: treasury ${funder.address} holds ${ethers.formatEther(funderBalance)} but ${ethers.formatEther(total + gasReserve)} is needed - top it up`);
    }
    let nonce = await provider.getTransactionCount(funder.address, 'pending');
    for (const { addr, shortfall } of plan) {
      const tx = await funder.sendTransaction({ to: addr, value: shortfall, nonce });
      nonce += 1;
      console.log(`${chain.label} -> ${addr} ${ethers.formatEther(shortfall)} tx=${tx.hash}`);
      const receipt = await tx.wait(1);
      if (!receipt || receipt.status !== 1) {
        throw new Error(`${chain.label}: transfer to ${addr} failed (tx ${tx.hash})`);
      }
    }
    console.log(`${chain.label}: funding complete`);
  } finally {
    provider.destroy();
  }
}

(async () => {
  for (const chain of chains) {
    await fundChain(chain);
  }
})().catch((err) => {
  console.error(`::error::${err.message}`);
  process.exit(1);
});
