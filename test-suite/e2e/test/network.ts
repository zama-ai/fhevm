import hardhat from 'hardhat';

const { network } = hardhat;

const LIVE_NETWORKS = new Set(['sepolia', 'mainnet', 'polygon', 'polygonAmoy', 'bnb', 'bnbTestnet', 'hoodi']);

export const activeNetworkName = () => network.name;

export const isLiveNetwork = () => LIVE_NETWORKS.has(activeNetworkName());
