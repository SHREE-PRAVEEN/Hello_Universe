'use client';

import * as React from 'react';
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { useChainStore } from '@/store/useChainStore';
import { useWeb3 } from '@/hooks/useWeb3';
import { WalletModal } from '@/components/blockchain/WalletModal';
import { TransactionList } from '@/components/blockchain/TransactionList';
import { truncateAddress, formatCrypto } from '@/lib/utils';

export default function BlockchainPage() {
  const [isWalletModalOpen, setIsWalletModalOpen] = React.useState(false);
  const { isConnected, address, balance, chainId, disconnectWallet } = useWeb3();
  const { selectedChain, tokenBalances, transactions, setSelectedChain } = useChainStore();

  const chains = [
    { id: 1, name: 'Ethereum', icon: '⟠', color: 'from-blue-500 to-purple-600' },
    { id: 137, name: 'Polygon', icon: '⬡', color: 'from-purple-500 to-pink-600' },
    { id: 42161, name: 'Arbitrum', icon: '◈', color: 'from-cyan-500 to-blue-600' },
    { id: 10, name: 'Optimism', icon: '⭕', color: 'from-red-500 to-pink-600' },
    { id: 8453, name: 'Base', icon: '🔵', color: 'from-blue-600 to-cyan-500' },
  ];

  return (
    <div className="min-h-screen bg-zinc-950 pt-20">
      <div className="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
        {/* Header */}
        <div className="mb-8 flex items-center justify-between">
          <div>
            <h1 className="text-3xl font-bold text-white">Blockchain</h1>
            <p className="mt-1 text-zinc-400">
              Manage your crypto assets and transactions.
            </p>
          </div>
          {isConnected ? (
            <div className="flex items-center gap-4">
              <div className="rounded-lg bg-zinc-800 px-4 py-2">
                <p className="text-xs text-zinc-400">Connected</p>
                <p className="font-mono text-sm text-white">
                  {truncateAddress(address || '')}
                </p>
              </div>
              <Button variant="outline" onClick={disconnectWallet}>
                Disconnect
              </Button>
            </div>
          ) : (
            <Button variant="primary" onClick={() => setIsWalletModalOpen(true)}>
              Connect Wallet
            </Button>
          )}
        </div>

        {!isConnected ? (
          /* Not Connected State */
          <Card variant="elevated" className="text-center">
            <CardContent className="py-16">
              <div className="mx-auto mb-6 flex h-20 w-20 items-center justify-center rounded-full bg-gradient-to-br from-cyan-500 to-blue-600">
                <WalletIcon className="h-10 w-10 text-white" />
              </div>
              <h2 className="text-2xl font-bold text-white">Connect Your Wallet</h2>
              <p className="mx-auto mt-2 max-w-md text-zinc-400">
                Connect your Web3 wallet to view your balances, manage assets, and interact
                with the Hello Universe blockchain ecosystem.
              </p>
              <Button
                variant="gradient"
                size="lg"
                className="mt-6"
                onClick={() => setIsWalletModalOpen(true)}
              >
                Get Started
              </Button>
            </CardContent>
          </Card>
        ) : (
          /* Connected State */
          <div className="space-y-8">
            {/* Chain Selector */}
            <div className="flex flex-wrap gap-3">
              {chains.map((chain) => (
                <button
                  key={chain.id}
                  onClick={() => setSelectedChain(chain.id as any)}
                  className={`flex items-center gap-2 rounded-xl border px-4 py-2.5 text-sm font-medium transition-all ${
                    chainId === chain.id
                      ? 'border-cyan-500 bg-cyan-500/10 text-cyan-400'
                      : 'border-zinc-700 text-zinc-400 hover:border-zinc-600 hover:text-white'
                  }`}
                >
                  <span className="text-lg">{chain.icon}</span>
                  {chain.name}
                </button>
              ))}
            </div>

            {/* Stats Grid */}
            <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-4">
              <Card variant="elevated">
                <CardContent className="p-6">
                  <p className="text-sm text-zinc-400">ETH Balance</p>
                  <p className="mt-1 text-2xl font-bold text-white">
                    {balance ? formatCrypto(parseFloat(balance.formatted)) : '0.00'} ETH
                  </p>
                  <p className="mt-1 text-sm text-zinc-500">
                    ≈ ${balance ? (parseFloat(balance.formatted) * 2500).toFixed(2) : '0.00'}
                  </p>
                </CardContent>
              </Card>
              <Card variant="elevated">
                <CardContent className="p-6">
                  <p className="text-sm text-zinc-400">HU Token</p>
                  <p className="mt-1 text-2xl font-bold text-white">12,450 HU</p>
                  <p className="mt-1 text-sm text-zinc-500">≈ $1,245.00</p>
                </CardContent>
              </Card>
              <Card variant="elevated">
                <CardContent className="p-6">
                  <p className="text-sm text-zinc-400">Staked</p>
                  <p className="mt-1 text-2xl font-bold text-white">5,000 HU</p>
                  <p className="mt-1 text-sm text-emerald-400">+12.5% APY</p>
                </CardContent>
              </Card>
              <Card variant="elevated">
                <CardContent className="p-6">
                  <p className="text-sm text-zinc-400">Rewards</p>
                  <p className="mt-1 text-2xl font-bold text-white">234 HU</p>
                  <Button variant="outline" size="sm" className="mt-2">
                    Claim
                  </Button>
                </CardContent>
              </Card>
            </div>

            {/* Main Content */}
            <div className="grid gap-8 lg:grid-cols-3">
              {/* Portfolio */}
              <Card variant="elevated" className="lg:col-span-2">
                <CardHeader>
                  <CardTitle className="text-white">Portfolio</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="space-y-4">
                    {[
                      { name: 'Ethereum', symbol: 'ETH', amount: '2.45', value: '$6,125.00', change: '+3.2%' },
                      { name: 'Hello Universe', symbol: 'HU', amount: '12,450', value: '$1,245.00', change: '+8.5%' },
                      { name: 'USD Coin', symbol: 'USDC', amount: '1,500', value: '$1,500.00', change: '0.0%' },
                      { name: 'Chainlink', symbol: 'LINK', amount: '125', value: '$875.00', change: '-2.1%' },
                    ].map((token) => (
                      <div
                        key={token.symbol}
                        className="flex items-center justify-between rounded-lg border border-zinc-800 p-4"
                      >
                        <div className="flex items-center gap-4">
                          <div className="flex h-10 w-10 items-center justify-center rounded-full bg-gradient-to-br from-cyan-500 to-blue-600 font-bold text-white">
                            {token.symbol.charAt(0)}
                          </div>
                          <div>
                            <p className="font-medium text-white">{token.name}</p>
                            <p className="text-sm text-zinc-500">{token.symbol}</p>
                          </div>
                        </div>
                        <div className="text-right">
                          <p className="font-medium text-white">{token.amount}</p>
                          <p className="text-sm text-zinc-500">{token.value}</p>
                        </div>
                        <span
                          className={`rounded-full px-2 py-0.5 text-xs font-medium ${
                            token.change.startsWith('+')
                              ? 'bg-emerald-500/10 text-emerald-400'
                              : token.change.startsWith('-')
                              ? 'bg-red-500/10 text-red-400'
                              : 'bg-zinc-500/10 text-zinc-400'
                          }`}
                        >
                          {token.change}
                        </span>
                      </div>
                    ))}
                  </div>
                </CardContent>
              </Card>

              {/* Quick Actions */}
              <Card variant="elevated">
                <CardHeader>
                  <CardTitle className="text-white">Quick Actions</CardTitle>
                </CardHeader>
                <CardContent className="space-y-3">
                  <Button variant="outline" className="w-full justify-start">
                    <SendIcon className="mr-3 h-4 w-4" />
                    Send Tokens
                  </Button>
                  <Button variant="outline" className="w-full justify-start">
                    <ReceiveIcon className="mr-3 h-4 w-4" />
                    Receive
                  </Button>
                  <Button variant="outline" className="w-full justify-start">
                    <SwapIcon className="mr-3 h-4 w-4" />
                    Swap
                  </Button>
                  <Button variant="outline" className="w-full justify-start">
                    <StakeIcon className="mr-3 h-4 w-4" />
                    Stake HU Tokens
                  </Button>
                  <Button variant="outline" className="w-full justify-start">
                    <BridgeIcon className="mr-3 h-4 w-4" />
                    Bridge Assets
                  </Button>
                </CardContent>
              </Card>
            </div>

            {/* Transaction History */}
            <Card variant="elevated">
              <CardHeader className="flex flex-row items-center justify-between">
                <CardTitle className="text-white">Transaction History</CardTitle>
                <Button variant="ghost" size="sm">
                  View All
                </Button>
              </CardHeader>
              <CardContent>
                <TransactionList limit={5} />
              </CardContent>
            </Card>
          </div>
        )}
      </div>

      <WalletModal isOpen={isWalletModalOpen} onClose={() => setIsWalletModalOpen(false)} />
    </div>
  );
}

// Icons
function WalletIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 7V4a1 1 0 00-1-1H5a2 2 0 000 4h15a1 1 0 011 1v4h-3a2 2 0 000 4h3a1 1 0 001-1v-2a1 1 0 00-1-1M3 5v14a2 2 0 002 2h15a1 1 0 001-1v-4" />
    </svg>
  );
}

function SendIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
    </svg>
  );
}

function ReceiveIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
    </svg>
  );
}

function SwapIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 16V4m0 0L3 8m4-4l4 4m6 0v12m0 0l4-4m-4 4l-4-4" />
    </svg>
  );
}

function StakeIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
    </svg>
  );
}

function BridgeIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4" />
    </svg>
  );
}
