'use client';

import * as React from 'react';
import { cn } from '@/lib/utils';
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card';
import { formatCryptoAmount, formatRelativeTime, truncateAddress } from '@/lib/utils';
import type { Transaction, TransactionStatus } from '@/types/blockchain.types';

// ============================================
// TRANSACTION LIST COMPONENT
// ============================================

export interface TransactionListProps {
  transactions: Transaction[];
  isLoading?: boolean;
  emptyMessage?: string;
  className?: string;
}

export function TransactionList({
  transactions,
  isLoading = false,
  emptyMessage = 'No transactions yet',
  className,
}: TransactionListProps) {
  if (isLoading) {
    return (
      <Card className={className}>
        <CardHeader>
          <CardTitle>Recent Transactions</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            {[...Array(3)].map((_, i) => (
              <TransactionSkeleton key={i} />
            ))}
          </div>
        </CardContent>
      </Card>
    );
  }

  if (transactions.length === 0) {
    return (
      <Card className={className}>
        <CardHeader>
          <CardTitle>Recent Transactions</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex flex-col items-center justify-center py-12">
            <div className="flex h-16 w-16 items-center justify-center rounded-full bg-zinc-100 dark:bg-zinc-800">
              <TransactionIcon className="h-8 w-8 text-zinc-400" />
            </div>
            <p className="mt-4 text-sm text-zinc-500 dark:text-zinc-400">
              {emptyMessage}
            </p>
          </div>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card className={className}>
      <CardHeader>
        <CardTitle>Recent Transactions</CardTitle>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          {transactions.map((tx) => (
            <TransactionItem key={tx.hash} transaction={tx} />
          ))}
        </div>
      </CardContent>
    </Card>
  );
}

// ============================================
// TRANSACTION ITEM
// ============================================

interface TransactionItemProps {
  transaction: Transaction;
}

function TransactionItem({ transaction }: TransactionItemProps) {
  const statusConfig = getStatusConfig(transaction.status);

  return (
    <div className="flex items-center gap-4 rounded-xl border border-zinc-200 p-4 transition-colors hover:bg-zinc-50 dark:border-zinc-800 dark:hover:bg-zinc-800/50">
      {/* Status Icon */}
      <div
        className={cn(
          'flex h-10 w-10 shrink-0 items-center justify-center rounded-full',
          statusConfig.bgColor
        )}
      >
        <statusConfig.icon className={cn('h-5 w-5', statusConfig.iconColor)} />
      </div>

      {/* Transaction Details */}
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-2">
          <p className="font-medium text-zinc-900 dark:text-white truncate">
            {transaction.description || 'Transaction'}
          </p>
          <span
            className={cn(
              'shrink-0 rounded-full px-2 py-0.5 text-xs font-medium',
              statusConfig.badgeColor
            )}
          >
            {statusConfig.label}
          </span>
        </div>
        <div className="mt-1 flex items-center gap-2 text-sm text-zinc-500 dark:text-zinc-400">
          <span>To: {truncateAddress(transaction.to)}</span>
          <span>•</span>
          <span>{formatRelativeTime(transaction.timestamp)}</span>
        </div>
      </div>

      {/* Amount */}
      <div className="text-right">
        <p className="font-mono font-medium text-zinc-900 dark:text-white">
          {formatCryptoAmount(transaction.value)} ETH
        </p>
        <a
          href={`https://etherscan.io/tx/${transaction.hash}`}
          target="_blank"
          rel="noopener noreferrer"
          className="text-xs text-cyan-500 hover:underline"
        >
          View on Explorer
        </a>
      </div>
    </div>
  );
}

// ============================================
// TRANSACTION SKELETON
// ============================================

function TransactionSkeleton() {
  return (
    <div className="flex items-center gap-4 rounded-xl border border-zinc-200 p-4 dark:border-zinc-800">
      <div className="h-10 w-10 animate-pulse rounded-full bg-zinc-200 dark:bg-zinc-700" />
      <div className="flex-1 space-y-2">
        <div className="h-4 w-32 animate-pulse rounded bg-zinc-200 dark:bg-zinc-700" />
        <div className="h-3 w-48 animate-pulse rounded bg-zinc-200 dark:bg-zinc-700" />
      </div>
      <div className="space-y-2 text-right">
        <div className="h-4 w-20 animate-pulse rounded bg-zinc-200 dark:bg-zinc-700" />
        <div className="h-3 w-24 animate-pulse rounded bg-zinc-200 dark:bg-zinc-700" />
      </div>
    </div>
  );
}

// ============================================
// STATUS CONFIG
// ============================================

interface StatusConfig {
  label: string;
  icon: React.ComponentType<{ className?: string }>;
  bgColor: string;
  iconColor: string;
  badgeColor: string;
}

function getStatusConfig(status: TransactionStatus): StatusConfig {
  switch (status) {
    case 'pending':
      return {
        label: 'Pending',
        icon: ClockIcon,
        bgColor: 'bg-yellow-100 dark:bg-yellow-900/30',
        iconColor: 'text-yellow-600 dark:text-yellow-400',
        badgeColor: 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-400',
      };
    case 'confirmed':
      return {
        label: 'Confirmed',
        icon: CheckIcon,
        bgColor: 'bg-emerald-100 dark:bg-emerald-900/30',
        iconColor: 'text-emerald-600 dark:text-emerald-400',
        badgeColor: 'bg-emerald-100 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-400',
      };
    case 'failed':
      return {
        label: 'Failed',
        icon: XIcon,
        bgColor: 'bg-red-100 dark:bg-red-900/30',
        iconColor: 'text-red-600 dark:text-red-400',
        badgeColor: 'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400',
      };
    case 'cancelled':
      return {
        label: 'Cancelled',
        icon: XIcon,
        bgColor: 'bg-zinc-100 dark:bg-zinc-800',
        iconColor: 'text-zinc-500 dark:text-zinc-400',
        badgeColor: 'bg-zinc-100 text-zinc-600 dark:bg-zinc-800 dark:text-zinc-400',
      };
    default:
      return {
        label: 'Unknown',
        icon: TransactionIcon,
        bgColor: 'bg-zinc-100 dark:bg-zinc-800',
        iconColor: 'text-zinc-500 dark:text-zinc-400',
        badgeColor: 'bg-zinc-100 text-zinc-600 dark:bg-zinc-800 dark:text-zinc-400',
      };
  }
}

// ============================================
// ICONS
// ============================================

function TransactionIcon({ className }: { className?: string }) {
  return (
    <svg
      className={className}
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <path d="M12 2v20M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6" />
    </svg>
  );
}

function ClockIcon({ className }: { className?: string }) {
  return (
    <svg
      className={className}
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <circle cx="12" cy="12" r="10" />
      <polyline points="12 6 12 12 16 14" />
    </svg>
  );
}

function CheckIcon({ className }: { className?: string }) {
  return (
    <svg
      className={className}
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <polyline points="20 6 9 17 4 12" />
    </svg>
  );
}

function XIcon({ className }: { className?: string }) {
  return (
    <svg
      className={className}
      xmlns="http://www.w3.org/2000/svg"
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
  );
}

export default TransactionList;
