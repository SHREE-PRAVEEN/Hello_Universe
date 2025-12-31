'use client';

// Hello Universe - Blockchain Provider
// Wagmi + Viem configuration for Web3 functionality

import * as React from 'react';
import { WagmiProvider, createConfig, http } from 'wagmi';
import { mainnet, polygon, arbitrum, optimism, base } from 'wagmi/chains';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { injected, walletConnect, coinbaseWallet } from 'wagmi/connectors';

// ============================================
// CONFIGURATION
// ============================================

const projectId = process.env.NEXT_PUBLIC_WALLET_CONNECT_PROJECT_ID || '';

// Configure chains
const chains = [mainnet, polygon, arbitrum, optimism, base] as const;

// Configure connectors
const connectors = [
  injected(),
  walletConnect({ projectId }),
  coinbaseWallet({ appName: 'Hello Universe' }),
];

// Create wagmi config
export const wagmiConfig = createConfig({
  chains,
  connectors,
  transports: {
    [mainnet.id]: http(),
    [polygon.id]: http(),
    [arbitrum.id]: http(),
    [optimism.id]: http(),
    [base.id]: http(),
  },
});

// Create query client
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 60 * 1000, // 1 minute
      gcTime: 10 * 60 * 1000, // 10 minutes (formerly cacheTime)
    },
  },
});

// ============================================
// BLOCKCHAIN PROVIDER COMPONENT
// ============================================

export interface BlockchainProviderProps {
  children: React.ReactNode;
}

export function BlockchainProvider({ children }: BlockchainProviderProps) {
  return (
    <WagmiProvider config={wagmiConfig}>
      <QueryClientProvider client={queryClient}>
        {children}
      </QueryClientProvider>
    </WagmiProvider>
  );
}

// ============================================
// CONTRACT ADDRESSES
// ============================================

export const CONTRACT_ADDRESSES = {
  // Mainnet
  1: {
    robotNFT: '0x0000000000000000000000000000000000000000' as const,
    marketplace: '0x0000000000000000000000000000000000000000' as const,
    staking: '0x0000000000000000000000000000000000000000' as const,
    token: '0x0000000000000000000000000000000000000000' as const,
  },
  // Polygon
  137: {
    robotNFT: '0x0000000000000000000000000000000000000000' as const,
    marketplace: '0x0000000000000000000000000000000000000000' as const,
    staking: '0x0000000000000000000000000000000000000000' as const,
    token: '0x0000000000000000000000000000000000000000' as const,
  },
  // Arbitrum
  42161: {
    robotNFT: '0x0000000000000000000000000000000000000000' as const,
    marketplace: '0x0000000000000000000000000000000000000000' as const,
    staking: '0x0000000000000000000000000000000000000000' as const,
    token: '0x0000000000000000000000000000000000000000' as const,
  },
} as const;

// ============================================
// CONTRACT ABIS (Simplified examples)
// ============================================

export const ROBOT_NFT_ABI = [
  {
    name: 'balanceOf',
    type: 'function',
    stateMutability: 'view',
    inputs: [{ name: 'owner', type: 'address' }],
    outputs: [{ name: '', type: 'uint256' }],
  },
  {
    name: 'ownerOf',
    type: 'function',
    stateMutability: 'view',
    inputs: [{ name: 'tokenId', type: 'uint256' }],
    outputs: [{ name: '', type: 'address' }],
  },
  {
    name: 'tokenURI',
    type: 'function',
    stateMutability: 'view',
    inputs: [{ name: 'tokenId', type: 'uint256' }],
    outputs: [{ name: '', type: 'string' }],
  },
  {
    name: 'mint',
    type: 'function',
    stateMutability: 'payable',
    inputs: [{ name: 'to', type: 'address' }],
    outputs: [{ name: '', type: 'uint256' }],
  },
  {
    name: 'Transfer',
    type: 'event',
    inputs: [
      { name: 'from', type: 'address', indexed: true },
      { name: 'to', type: 'address', indexed: true },
      { name: 'tokenId', type: 'uint256', indexed: true },
    ],
  },
] as const;

export const ERC20_ABI = [
  {
    name: 'name',
    type: 'function',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ name: '', type: 'string' }],
  },
  {
    name: 'symbol',
    type: 'function',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ name: '', type: 'string' }],
  },
  {
    name: 'decimals',
    type: 'function',
    stateMutability: 'view',
    inputs: [],
    outputs: [{ name: '', type: 'uint8' }],
  },
  {
    name: 'balanceOf',
    type: 'function',
    stateMutability: 'view',
    inputs: [{ name: 'account', type: 'address' }],
    outputs: [{ name: '', type: 'uint256' }],
  },
  {
    name: 'transfer',
    type: 'function',
    stateMutability: 'nonpayable',
    inputs: [
      { name: 'to', type: 'address' },
      { name: 'amount', type: 'uint256' },
    ],
    outputs: [{ name: '', type: 'bool' }],
  },
  {
    name: 'approve',
    type: 'function',
    stateMutability: 'nonpayable',
    inputs: [
      { name: 'spender', type: 'address' },
      { name: 'amount', type: 'uint256' },
    ],
    outputs: [{ name: '', type: 'bool' }],
  },
] as const;

// ============================================
// UTILITY FUNCTIONS
// ============================================

export function getContractAddress(
  chainId: keyof typeof CONTRACT_ADDRESSES,
  contract: keyof (typeof CONTRACT_ADDRESSES)[1]
) {
  return CONTRACT_ADDRESSES[chainId]?.[contract];
}

export function getBlockExplorerUrl(chainId: number, hash: string, type: 'tx' | 'address' = 'tx') {
  const explorers: Record<number, string> = {
    1: 'https://etherscan.io',
    137: 'https://polygonscan.com',
    42161: 'https://arbiscan.io',
    10: 'https://optimistic.etherscan.io',
    8453: 'https://basescan.org',
  };

  const baseUrl = explorers[chainId] || explorers[1];
  return `${baseUrl}/${type}/${hash}`;
}

export default BlockchainProvider;
