'use client';

import * as React from 'react';
import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card';
import { useChainStore } from '@/store/useChainStore';
import { truncateAddress } from '@/lib/utils';
import type { WalletConnector } from '@/types/blockchain.types';

// ============================================
// WALLET CONNECTORS
// ============================================

const walletConnectors: WalletConnector[] = [
  {
    id: 'metamask',
    name: 'MetaMask',
    icon: '/assets/logos/metamask.svg',
    ready: true,
  },
  {
    id: 'walletconnect',
    name: 'WalletConnect',
    icon: '/assets/logos/walletconnect.svg',
    ready: true,
  },
  {
    id: 'coinbase',
    name: 'Coinbase Wallet',
    icon: '/assets/logos/coinbase.svg',
    ready: true,
  },
  {
    id: 'rainbow',
    name: 'Rainbow',
    icon: '/assets/logos/rainbow.svg',
    ready: true,
  },
];

// ============================================
// WALLET MODAL COMPONENT
// ============================================

export interface WalletModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export function WalletModal({ isOpen, onClose }: WalletModalProps) {
  const { wallet, connectWallet, disconnectWallet } = useChainStore();
  const [isConnecting, setIsConnecting] = React.useState<string | null>(null);
  const [error, setError] = React.useState<string | null>(null);

  const handleConnect = async (connectorId: string) => {
    setIsConnecting(connectorId);
    setError(null);

    try {
      await connectWallet(connectorId);
      onClose();
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to connect wallet');
    } finally {
      setIsConnecting(null);
    }
  };

  const handleDisconnect = async () => {
    await disconnectWallet();
    onClose();
  };

  // Close modal on escape key
  React.useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose();
    };

    if (isOpen) {
      document.addEventListener('keydown', handleEscape);
      document.body.style.overflow = 'hidden';
    }

    return () => {
      document.removeEventListener('keydown', handleEscape);
      document.body.style.overflow = '';
    };
  }, [isOpen, onClose]);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-black/60 backdrop-blur-sm"
        onClick={onClose}
      />

      {/* Modal */}
      <Card
        variant="elevated"
        className="relative z-10 w-full max-w-md animate-in fade-in zoom-in-95 duration-200"
      >
        {/* Close button */}
        <button
          onClick={onClose}
          className="absolute right-4 top-4 flex h-8 w-8 items-center justify-center rounded-lg text-zinc-400 transition-colors hover:bg-zinc-100 hover:text-zinc-900 dark:hover:bg-zinc-800 dark:hover:text-white"
          aria-label="Close"
        >
          <svg
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
          >
            <path d="M18 6 6 18" />
            <path d="m6 6 12 12" />
          </svg>
        </button>

        <CardHeader>
          <CardTitle>
            {wallet.status === 'connected' ? 'Wallet Connected' : 'Connect Wallet'}
          </CardTitle>
        </CardHeader>

        <CardContent>
          {wallet.status === 'connected' && wallet.address ? (
            // Connected State
            <div className="space-y-4">
              <div className="flex items-center gap-4 rounded-xl bg-zinc-100 p-4 dark:bg-zinc-800">
                <div className="flex h-12 w-12 items-center justify-center rounded-full bg-gradient-to-br from-cyan-500 to-blue-600">
                  <WalletIcon className="h-6 w-6 text-white" />
                </div>
                <div>
                  <p className="text-sm text-zinc-500 dark:text-zinc-400">
                    Connected Address
                  </p>
                  <p className="font-mono text-lg font-medium text-zinc-900 dark:text-white">
                    {truncateAddress(wallet.address, 6)}
                  </p>
                </div>
              </div>

              {wallet.ensName && (
                <div className="rounded-xl bg-zinc-100 p-4 dark:bg-zinc-800">
                  <p className="text-sm text-zinc-500 dark:text-zinc-400">ENS Name</p>
                  <p className="font-medium text-zinc-900 dark:text-white">
                    {wallet.ensName}
                  </p>
                </div>
              )}

              <Button
                variant="destructive"
                className="w-full"
                onClick={handleDisconnect}
              >
                Disconnect Wallet
              </Button>
            </div>
          ) : (
            // Not Connected State
            <div className="space-y-4">
              {error && (
                <div className="rounded-xl bg-red-50 p-4 text-sm text-red-600 dark:bg-red-900/20 dark:text-red-400">
                  {error}
                </div>
              )}

              <div className="space-y-2">
                {walletConnectors.map((connector) => (
                  <button
                    key={connector.id}
                    onClick={() => handleConnect(connector.id)}
                    disabled={!connector.ready || isConnecting !== null}
                    className={cn(
                      'flex w-full items-center gap-4 rounded-xl border border-zinc-200 p-4 transition-all duration-200',
                      'hover:border-cyan-500 hover:bg-zinc-50 dark:border-zinc-700 dark:hover:border-cyan-500 dark:hover:bg-zinc-800',
                      'disabled:cursor-not-allowed disabled:opacity-50',
                      isConnecting === connector.id && 'border-cyan-500 bg-cyan-50 dark:bg-cyan-900/20'
                    )}
                  >
                    <div className="flex h-10 w-10 items-center justify-center rounded-xl bg-zinc-100 dark:bg-zinc-800">
                      {/* Placeholder for wallet icon */}
                      <WalletIcon className="h-5 w-5 text-zinc-600 dark:text-zinc-400" />
                    </div>
                    <span className="flex-1 text-left font-medium text-zinc-900 dark:text-white">
                      {connector.name}
                    </span>
                    {isConnecting === connector.id && (
                      <div className="h-5 w-5 animate-spin rounded-full border-2 border-cyan-500 border-t-transparent" />
                    )}
                  </button>
                ))}
              </div>

              <p className="text-center text-xs text-zinc-500 dark:text-zinc-400">
                By connecting, you agree to our{' '}
                <a href="/terms" className="text-cyan-500 hover:underline">
                  Terms of Service
                </a>{' '}
                and{' '}
                <a href="/privacy" className="text-cyan-500 hover:underline">
                  Privacy Policy
                </a>
              </p>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}

// ============================================
// CONNECT WALLET BUTTON
// ============================================

export function ConnectWalletButton({ className }: { className?: string }) {
  const [isModalOpen, setIsModalOpen] = React.useState(false);
  const { wallet } = useChainStore();

  return (
    <>
      <Button
        variant={wallet.status === 'connected' ? 'outline' : 'primary'}
        className={className}
        onClick={() => setIsModalOpen(true)}
      >
        {wallet.status === 'connected' && wallet.address ? (
          <>
            <WalletIcon className="h-4 w-4" />
            {truncateAddress(wallet.address)}
          </>
        ) : (
          'Connect Wallet'
        )}
      </Button>

      <WalletModal isOpen={isModalOpen} onClose={() => setIsModalOpen(false)} />
    </>
  );
}

// ============================================
// WALLET ICON
// ============================================

function WalletIcon({ className }: { className?: string }) {
  return (
    <svg
      className={className}
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <path d="M19 7V4a1 1 0 0 0-1-1H5a2 2 0 0 0 0 4h15a1 1 0 0 1 1 1v4h-3a2 2 0 0 0 0 4h3a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1" />
      <path d="M3 5v14a2 2 0 0 0 2 2h15a1 1 0 0 0 1-1v-4" />
    </svg>
  );
}

export default WalletModal;
