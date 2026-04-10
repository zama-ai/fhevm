import { network } from 'hardhat';

const LIVE_NETWORKS = new Set(['devnet', 'devnetNative', 'zwsDev', 'sepolia', 'mainnet']);

export const activeNetworkName = () => network.name;

export const isLiveNetwork = () => LIVE_NETWORKS.has(activeNetworkName());
