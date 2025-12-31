'use client';

import * as React from 'react';
import { useAccount, useConnect, useDisconnect, useBalance, useEnsName, useEnsAvatar, useSendTransaction, useWaitForTransactionReceipt } from 'wagmi';
import type { Address, Hash } from 'viem';
import type { WalletState, WalletStatus } from '@/types/blockchain.types';

// ============================================
// USE WEB3 HOOK
// ============================================

export interface UseWeb3Return {
  // Wallet State
  wallet: WalletState;
  isConnecting: boolean;
  isConnected: boolean;
  
  // Actions
  connect: (connectorId?: string) => Promise<void>;
  disconnect: () => void;
  
  // Balance
  balance: bigint;
  formattedBalance: string;
  
  // ENS
  ensName: string | null;
  ensAvatar: string | null;
  
  // Transaction
  sendTransaction: (params: SendTransactionParams) => Promise<Hash>;
  transactionStatus: TransactionStatus;
}

export interface SendTransactionParams {
  to: Address;
  value: bigint;
  data?: `0x${string}`;
}

type TransactionStatus = 'idle' | 'pending' | 'success' | 'error';

export function useWeb3(): UseWeb3Return {
  // Wagmi hooks
  const { address, isConnected, isConnecting, chain } = useAccount();
  const { connect: wagmiConnect, connectors } = useConnect();
  const { disconnect: wagmiDisconnect } = useDisconnect();
  
  // Balance
  const { data: balanceData } = useBalance({
    address: address,
  });
  
  // ENS
  const { data: ensName } = useEnsName({
    address: address,
    chainId: 1, // ENS is on mainnet
  });
  
  const { data: ensAvatar } = useEnsAvatar({
    name: ensName ?? undefined,
    chainId: 1,
  });
  
  // Transaction
  const { sendTransaction: wagmiSendTx, data: txHash, isPending: isTxPending, isError: isTxError, isSuccess: isTxSuccess } = useSendTransaction();
  
  const { isLoading: isConfirming, isSuccess: isConfirmed } = useWaitForTransactionReceipt({
    hash: txHash,
  });

  // Derive wallet status
  const walletStatus: WalletStatus = React.useMemo(() => {
    if (isConnected) return 'connected';
    if (isConnecting) return 'connecting';
    return 'disconnected';
  }, [isConnected, isConnecting]);

  // Wallet state object
  const wallet: WalletState = React.useMemo(() => ({
    status: walletStatus,
    address: address ?? null,
    chainId: (chain?.id as WalletState['chainId']) ?? null,
    balance: balanceData?.value ?? BigInt(0),
    ensName: ensName ?? null,
    ensAvatar: ensAvatar ?? null,
  }), [walletStatus, address, chain?.id, balanceData?.value, ensName, ensAvatar]);

  // Connect function
  const connect = React.useCallback(async (connectorId?: string) => {
    const connector = connectorId
      ? connectors.find((c) => c.id === connectorId)
      : connectors[0];

    if (!connector) {
      throw new Error('No connector found');
    }

    wagmiConnect({ connector });
  }, [connectors, wagmiConnect]);

  // Disconnect function
  const disconnect = React.useCallback(() => {
    wagmiDisconnect();
  }, [wagmiDisconnect]);

  // Send transaction function
  const sendTransaction = React.useCallback(async (params: SendTransactionParams): Promise<Hash> => {
    return new Promise((resolve, reject) => {
      wagmiSendTx(
        {
          to: params.to,
          value: params.value,
          data: params.data,
        },
        {
          onSuccess: (hash) => resolve(hash),
          onError: (error) => reject(error),
        }
      );
    });
  }, [wagmiSendTx]);

  // Transaction status
  const transactionStatus: TransactionStatus = React.useMemo(() => {
    if (isTxPending || isConfirming) return 'pending';
    if (isTxError) return 'error';
    if (isTxSuccess && isConfirmed) return 'success';
    return 'idle';
  }, [isTxPending, isConfirming, isTxError, isTxSuccess, isConfirmed]);

  // Formatted balance
  const formattedBalance = React.useMemo(() => {
    if (!balanceData) return '0.0000';
    const formatted = Number(balanceData.formatted).toFixed(4);
    return `${formatted} ${balanceData.symbol}`;
  }, [balanceData]);

  return {
    wallet,
    isConnecting,
    isConnected,
    connect,
    disconnect,
    balance: balanceData?.value ?? BigInt(0),
    formattedBalance,
    ensName: ensName ?? null,
    ensAvatar: ensAvatar ?? null,
    sendTransaction,
    transactionStatus,
  };
}

// ============================================
// USE CHAIN ID HOOK
// ============================================

export function useChainId() {
  const { chain } = useAccount();
  return chain?.id ?? null;
}

// ============================================
// USE SWITCH CHAIN HOOK
// ============================================

export function useSwitchChain() {
  const { switchChain } = require('wagmi');
  
  const handleSwitchChain = React.useCallback(async (chainId: number) => {
    try {
      await switchChain({ chainId });
    } catch (error) {
      console.error('Failed to switch chain:', error);
      throw error;
    }
  }, [switchChain]);

  return handleSwitchChain;
}

export default useWeb3;
