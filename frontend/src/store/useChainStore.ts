// Hello Universe - Chain Store
// Zustand store for blockchain and wallet state

import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';
import type {
  WalletState,
  WalletStatus,
  Transaction,
  SupportedChainId,
  ChainInfo,
} from '@/types/blockchain.types';
import type { Address } from 'viem';

// ============================================
// SUPPORTED CHAINS
// ============================================

export const SUPPORTED_CHAINS: Record<SupportedChainId, ChainInfo> = {
  1: {
    id: 1,
    name: 'Ethereum',
    network: 'mainnet',
    nativeCurrency: { name: 'Ether', symbol: 'ETH', decimals: 18 },
    rpcUrls: ['https://eth.llamarpc.com'],
    blockExplorerUrls: ['https://etherscan.io'],
    iconUrl: '/assets/logos/ethereum.svg',
  },
  137: {
    id: 137,
    name: 'Polygon',
    network: 'polygon',
    nativeCurrency: { name: 'MATIC', symbol: 'MATIC', decimals: 18 },
    rpcUrls: ['https://polygon-rpc.com'],
    blockExplorerUrls: ['https://polygonscan.com'],
    iconUrl: '/assets/logos/polygon.svg',
  },
  42161: {
    id: 42161,
    name: 'Arbitrum One',
    network: 'arbitrum',
    nativeCurrency: { name: 'Ether', symbol: 'ETH', decimals: 18 },
    rpcUrls: ['https://arb1.arbitrum.io/rpc'],
    blockExplorerUrls: ['https://arbiscan.io'],
    iconUrl: '/assets/logos/arbitrum.svg',
  },
  10: {
    id: 10,
    name: 'Optimism',
    network: 'optimism',
    nativeCurrency: { name: 'Ether', symbol: 'ETH', decimals: 18 },
    rpcUrls: ['https://mainnet.optimism.io'],
    blockExplorerUrls: ['https://optimistic.etherscan.io'],
    iconUrl: '/assets/logos/optimism.svg',
  },
  8453: {
    id: 8453,
    name: 'Base',
    network: 'base',
    nativeCurrency: { name: 'Ether', symbol: 'ETH', decimals: 18 },
    rpcUrls: ['https://mainnet.base.org'],
    blockExplorerUrls: ['https://basescan.org'],
    iconUrl: '/assets/logos/base.svg',
  },
};

// ============================================
// CHAIN STORE STATE
// ============================================

export interface ChainState {
  // Wallet state
  wallet: WalletState;
  
  // Chain state
  currentChain: ChainInfo | null;
  supportedChains: ChainInfo[];
  
  // Transactions
  transactions: Transaction[];
  pendingTransactions: Transaction[];
  
  // UI state
  isWalletModalOpen: boolean;
  isChainSwitching: boolean;
  
  // Actions
  setWalletStatus: (status: WalletStatus) => void;
  setWalletAddress: (address: Address | null) => void;
  setChainId: (chainId: SupportedChainId | null) => void;
  setBalance: (balance: bigint) => void;
  setEnsName: (name: string | null) => void;
  setEnsAvatar: (avatar: string | null) => void;
  
  // Wallet actions
  connectWallet: (connectorId: string) => Promise<void>;
  disconnectWallet: () => Promise<void>;
  switchChain: (chainId: SupportedChainId) => Promise<void>;
  
  // Transaction actions
  addTransaction: (tx: Transaction) => void;
  updateTransaction: (hash: string, updates: Partial<Transaction>) => void;
  clearTransactions: () => void;
  
  // UI actions
  setWalletModalOpen: (isOpen: boolean) => void;
}

// ============================================
// INITIAL WALLET STATE
// ============================================

const initialWalletState: WalletState = {
  status: 'disconnected',
  address: null,
  chainId: null,
  balance: BigInt(0),
  ensName: null,
  ensAvatar: null,
};

// ============================================
// CHAIN STORE
// ============================================

export const useChainStore = create<ChainState>()(
  persist(
    (set, get) => ({
      // Initial state
      wallet: initialWalletState,
      currentChain: null,
      supportedChains: Object.values(SUPPORTED_CHAINS),
      transactions: [],
      pendingTransactions: [],
      isWalletModalOpen: false,
      isChainSwitching: false,

      // Wallet state setters
      setWalletStatus: (status) =>
        set((state) => ({
          wallet: { ...state.wallet, status },
        })),

      setWalletAddress: (address) =>
        set((state) => ({
          wallet: { ...state.wallet, address },
        })),

      setChainId: (chainId) =>
        set((state) => ({
          wallet: { ...state.wallet, chainId },
          currentChain: chainId ? SUPPORTED_CHAINS[chainId] || null : null,
        })),

      setBalance: (balance) =>
        set((state) => ({
          wallet: { ...state.wallet, balance },
        })),

      setEnsName: (ensName) =>
        set((state) => ({
          wallet: { ...state.wallet, ensName },
        })),

      setEnsAvatar: (ensAvatar) =>
        set((state) => ({
          wallet: { ...state.wallet, ensAvatar },
        })),

      // Connect wallet
      connectWallet: async (connectorId) => {
        set((state) => ({
          wallet: { ...state.wallet, status: 'connecting' },
        }));

        try {
          // In a real implementation, this would use wagmi's connect function
          // This is a placeholder for the actual connection logic
          console.log('Connecting with:', connectorId);
          
          // Simulate connection delay
          await new Promise((resolve) => setTimeout(resolve, 1000));
          
          // For demo purposes, we'll just set a mock address
          // In production, this would be replaced with actual wagmi integration
          set((state) => ({
            wallet: {
              ...state.wallet,
              status: 'connected',
              // Mock address for demo
              address: '0x1234567890123456789012345678901234567890' as Address,
              chainId: 1,
              balance: BigInt(0),
            },
            currentChain: SUPPORTED_CHAINS[1],
          }));
        } catch (error) {
          set((state) => ({
            wallet: { ...state.wallet, status: 'error' },
          }));
          throw error;
        }
      },

      // Disconnect wallet
      disconnectWallet: async () => {
        set({
          wallet: initialWalletState,
          currentChain: null,
        });
      },

      // Switch chain
      switchChain: async (chainId) => {
        set({ isChainSwitching: true });

        try {
          // In a real implementation, this would use wagmi's switchChain
          console.log('Switching to chain:', chainId);
          
          await new Promise((resolve) => setTimeout(resolve, 500));

          set((state) => ({
            wallet: { ...state.wallet, chainId },
            currentChain: SUPPORTED_CHAINS[chainId],
            isChainSwitching: false,
          }));
        } catch (error) {
          set({ isChainSwitching: false });
          throw error;
        }
      },

      // Transaction actions
      addTransaction: (tx) =>
        set((state) => ({
          transactions: [tx, ...state.transactions],
          pendingTransactions:
            tx.status === 'pending'
              ? [tx, ...state.pendingTransactions]
              : state.pendingTransactions,
        })),

      updateTransaction: (hash, updates) =>
        set((state) => ({
          transactions: state.transactions.map((tx) =>
            tx.hash === hash ? { ...tx, ...updates } : tx
          ),
          pendingTransactions:
            updates.status && updates.status !== 'pending'
              ? state.pendingTransactions.filter((tx) => tx.hash !== hash)
              : state.pendingTransactions,
        })),

      clearTransactions: () =>
        set({
          transactions: [],
          pendingTransactions: [],
        }),

      // UI actions
      setWalletModalOpen: (isOpen) =>
        set({ isWalletModalOpen: isOpen }),
    }),
    {
      name: 'hello-universe-chain',
      storage: createJSONStorage(() => localStorage),
      partialize: (state) => ({
        // Only persist transactions
        transactions: state.transactions.slice(0, 50), // Keep last 50
      }),
    }
  )
);

// ============================================
// SELECTORS
// ============================================

export const selectWallet = (state: ChainState) => state.wallet;
export const selectIsConnected = (state: ChainState) => state.wallet.status === 'connected';
export const selectCurrentChain = (state: ChainState) => state.currentChain;
export const selectTransactions = (state: ChainState) => state.transactions;
export const selectPendingTransactions = (state: ChainState) => state.pendingTransactions;

export default useChainStore;
