const OWNABLE2STEP_ABI = [
  'function owner() view returns (address)',
  'function pendingOwner() view returns (address)',
];

const ZERO_ADDRESS = '0x0000000000000000000000000000000000000000';

async function getOwnerAndPending(contract) {
  const [owner, pendingOwner] = await Promise.all([
    contract.owner(),
    contract.pendingOwner(),
  ]);
  return { owner, pendingOwner };
}

module.exports = { OWNABLE2STEP_ABI, ZERO_ADDRESS, getOwnerAndPending };
