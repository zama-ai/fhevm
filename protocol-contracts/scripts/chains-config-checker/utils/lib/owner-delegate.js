const { ethers } = require('ethers');
const { isValidAddress } = require('../get-deployment-block');

const OAPP_ABI = [
  'function owner() view returns (address)',
  'function endpoint() view returns (address)',
];

const ENDPOINT_ABI = [
  'function delegates(address) view returns (address)',
];

async function getOwnerAndDelegateForChain(chainConfig) {
  const { name, rpcUrl, contractAddress } = chainConfig;

  if (!rpcUrl) throw new Error(`[${name}] RPC URL is not configured`);
  if (!contractAddress) throw new Error(`[${name}] Contract address is not configured`);
  if (!isValidAddress(contractAddress)) throw new Error(`[${name}] Invalid contract address format`);

  const provider = new ethers.JsonRpcProvider(rpcUrl);
  const oapp = new ethers.Contract(contractAddress, OAPP_ABI, provider);

  const [owner, endpointAddress] = await Promise.all([
    oapp.owner(),
    oapp.endpoint(),
  ]);

  const endpoint = new ethers.Contract(endpointAddress, ENDPOINT_ABI, provider);
  const delegate = await endpoint.delegates(contractAddress);

  return {
    name,
    contractAddress,
    owner,
    delegate,
    equal: owner === delegate,
  };
}

module.exports = { OAPP_ABI, ENDPOINT_ABI, getOwnerAndDelegateForChain };
