import { ethers } from 'hardhat';
import { sepolia } from '@fhevm/sdk/chains';

async function main() {
  const [signer] = await ethers.getSigners();
  console.log(`Signer: ${signer.address}`);
  console.log(`FHEVM chain: ${sepolia.id} (id ${sepolia.id})`);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
