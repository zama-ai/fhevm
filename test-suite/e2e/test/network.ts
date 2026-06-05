import hre from 'hardhat';

const LIVE_NETWORKS = new Set(['devnet', 'devnetNative', 'zwsDev', 'sepolia', 'mainnet', 'polygonAmoy']);

export const activeNetworkName = () => hre.network.name;

export const isLiveNetwork = () => LIVE_NETWORKS.has(activeNetworkName());
