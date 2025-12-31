// Hello Universe - Blockchain Types
// TypeScript definitions for Web3 and blockchain interactions

import type { Address, Hash } from 'viem';

// ============================================
// CHAIN & NETWORK TYPES
// ============================================

export type SupportedChainId = 1 | 137 | 42161 | 10 | 8453; // Mainnet, Polygon, Arbitrum, Optimism, Base

export interface ChainInfo {
  id: SupportedChainId;
  name: string;
  network: string;
  nativeCurrency: {
    name: string;
    symbol: string;
    decimals: number;
  };
  rpcUrls: string[];
  blockExplorerUrls: string[];
  iconUrl?: string;
}

// ============================================
// WALLET TYPES
// ============================================

export type WalletStatus = 'disconnected' | 'connecting' | 'connected' | 'error';

export interface WalletState {
  status: WalletStatus;
  address: Address | null;
  chainId: SupportedChainId | null;
  balance: bigint;
  ensName: string | null;
  ensAvatar: string | null;
}

export interface WalletConnector {
  id: string;
  name: string;
  icon: string;
  ready: boolean;
}

// ============================================
// TRANSACTION TYPES
// ============================================

export type TransactionStatus = 'pending' | 'confirmed' | 'failed' | 'cancelled';

export interface Transaction {
  hash: Hash;
  from: Address;
  to: Address;
  value: bigint;
  chainId: SupportedChainId;
  status: TransactionStatus;
  timestamp: number;
  blockNumber?: number;
  gasUsed?: bigint;
  gasPrice?: bigint;
  data?: string;
  description?: string;
}

export interface TransactionReceipt {
  hash: Hash;
  blockNumber: number;
  blockHash: Hash;
  status: 'success' | 'reverted';
  gasUsed: bigint;
  effectiveGasPrice: bigint;
  logs: TransactionLog[];
}

export interface TransactionLog {
  address: Address;
  topics: Hash[];
  data: string;
  blockNumber: number;
  transactionHash: Hash;
  logIndex: number;
}

// ============================================
// SMART CONTRACT TYPES
// ============================================

export interface ContractConfig {
  address: Address;
  abi: readonly object[];
  chainId: SupportedChainId;
}

export interface ContractCall {
  contractAddress: Address;
  functionName: string;
  args: unknown[];
  value?: bigint;
}

export interface ContractWriteResult {
  hash: Hash;
  wait: () => Promise<TransactionReceipt>;
}

// ============================================
// ROBOT NFT TYPES (Hello Universe Specific)
// ============================================

export interface RobotNFT {
  tokenId: bigint;
  owner: Address;
  name: string;
  description: string;
  image: string;
  model3D: string; // URL to GLB file
  attributes: RobotAttribute[];
  createdAt: number;
  rarity: 'common' | 'rare' | 'epic' | 'legendary';
}

export interface RobotAttribute {
  trait_type: string;
  value: string | number;
  display_type?: 'number' | 'boost_percentage' | 'date';
}

export interface RobotMetadata {
  name: string;
  description: string;
  image: string;
  animation_url?: string;
  external_url?: string;
  attributes: RobotAttribute[];
}

// ============================================
// MARKETPLACE TYPES
// ============================================

export interface Listing {
  id: bigint;
  seller: Address;
  tokenId: bigint;
  price: bigint;
  currency: Address; // ERC20 token address or zero address for native
  expiresAt: number;
  isActive: boolean;
}

export interface Offer {
  id: bigint;
  buyer: Address;
  tokenId: bigint;
  amount: bigint;
  currency: Address;
  expiresAt: number;
  isActive: boolean;
}

// ============================================
// STAKING TYPES
// ============================================

export interface StakeInfo {
  user: Address;
  amount: bigint;
  rewardDebt: bigint;
  pendingRewards: bigint;
  lockEndTime: number;
  lockDuration: number;
}

export interface StakingPool {
  id: number;
  name: string;
  totalStaked: bigint;
  rewardRate: bigint;
  lockPeriod: number;
  apy: number;
  isActive: boolean;
}

// ============================================
// EVENT TYPES
// ============================================

export interface BlockchainEvent {
  name: string;
  args: Record<string, unknown>;
  blockNumber: number;
  transactionHash: Hash;
  logIndex: number;
}

export interface TransferEvent extends BlockchainEvent {
  name: 'Transfer';
  args: {
    from: Address;
    to: Address;
    tokenId: bigint;
  };
}

export interface ApprovalEvent extends BlockchainEvent {
  name: 'Approval';
  args: {
    owner: Address;
    approved: Address;
    tokenId: bigint;
  };
}
