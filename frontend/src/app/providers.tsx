'use client';

import * as React from 'react';
import { BlockchainProvider } from '@/services/blockchain-provider';
import { Navbar } from '@/components/layout/Navbar';
import { Footer } from '@/components/layout/Footer';

// ============================================
// PROVIDERS COMPONENT
// ============================================

export function Providers({ children }: { children: React.ReactNode }) {
  return (
    <BlockchainProvider>
      <div className="flex min-h-screen flex-col">
        <Navbar />
        <main className="flex-1">{children}</main>
        <Footer />
      </div>
    </BlockchainProvider>
  );
}
