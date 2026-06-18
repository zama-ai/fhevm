import hardhat from 'hardhat';

const { network } = hardhat;

const LIVE_NETWORKS = new Set(['devnet', 'devnetNative', 'zwsDev', 'sepolia', 'mainnet', 'polygonAmoy']);

export const activeNetworkName = () => network.name;

export const isLiveNetwork = () => LIVE_NETWORKS.has(activeNetworkName());
