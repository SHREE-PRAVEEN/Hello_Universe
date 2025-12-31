'use client';

import * as React from 'react';
import Link from 'next/link';
import dynamic from 'next/dynamic';
import { Button } from '@/components/ui/button';
import { FeatureCard } from '@/components/ui/card';

// Dynamically import 3D components to avoid SSR issues
const Scene = dynamic(() => import('@/components/3d/Scene').then(mod => mod.Scene), {
  ssr: false,
  loading: () => <div className="h-full w-full bg-zinc-900 animate-pulse" />,
});

const PlaceholderRobot = dynamic(
  () => import('@/components/3d/Robot').then(mod => mod.PlaceholderRobot),
  { ssr: false }
);

const Controls = dynamic(() => import('@/components/3d/Controls'), { ssr: false });

// ============================================
// LANDING PAGE
// ============================================

export default function HomePage() {
  return (
    <div className="relative">
      {/* Hero Section */}
      <section className="relative min-h-screen overflow-hidden">
        {/* 3D Background */}
        <div className="absolute inset-0 z-0">
          <Scene backgroundColor="#09090b">
            <PlaceholderRobot position={[0, -0.5, 0]} animate />
            <Controls autoRotate autoRotateSpeed={0.3} />
          </Scene>
        </div>

        {/* Gradient Overlay */}
        <div className="absolute inset-0 z-10 bg-gradient-to-b from-transparent via-zinc-950/50 to-zinc-950" />

        {/* Hero Content */}
        <div className="relative z-20 flex min-h-screen flex-col items-center justify-center px-4 pt-16 text-center">
          <div className="animate-fade-in">
            {/* Badge */}
            <div className="mb-6 inline-flex items-center gap-2 rounded-full border border-cyan-500/30 bg-cyan-500/10 px-4 py-1.5 text-sm text-cyan-400">
              <span className="h-2 w-2 animate-pulse rounded-full bg-cyan-500" />
              Now in Public Beta
            </div>

            {/* Heading */}
            <h1 className="mb-6 bg-gradient-to-r from-white via-zinc-300 to-zinc-500 bg-clip-text text-5xl font-bold tracking-tight text-transparent sm:text-7xl md:text-8xl">
              Hello Universe
            </h1>

            {/* Subheading */}
            <p className="mx-auto mb-8 max-w-2xl text-lg text-zinc-400 sm:text-xl">
              Pioneering the future of robotics with AI-powered automation and
              blockchain technology. Build, deploy, and manage intelligent robots
              at scale.
            </p>

            {/* CTA Buttons */}
            <div className="flex flex-col items-center justify-center gap-4 sm:flex-row">
              <Link href="/auth/signup">
                <Button variant="primary" size="lg">
                  Get Started
                  <ArrowRightIcon className="ml-2 h-4 w-4" />
                </Button>
              </Link>
              <Link href="/ai-labs">
                <Button variant="outline" size="lg">
                  Explore AI Labs
                </Button>
              </Link>
            </div>
          </div>

          {/* Scroll Indicator */}
          <div className="absolute bottom-8 left-1/2 -translate-x-1/2">
            <div className="flex flex-col items-center gap-2 text-zinc-500">
              <span className="text-xs">Scroll to explore</span>
              <div className="h-6 w-4 rounded-full border border-zinc-700">
                <div className="mx-auto mt-1 h-1.5 w-1 animate-bounce rounded-full bg-zinc-500" />
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section id="features" className="bg-zinc-950 py-24">
        <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          <div className="text-center">
            <h2 className="text-3xl font-bold text-white sm:text-4xl">
              Built for the Future
            </h2>
            <p className="mt-4 text-lg text-zinc-400">
              Everything you need to build and deploy intelligent robotic systems
            </p>
          </div>

          <div className="mt-16 grid gap-8 md:grid-cols-2 lg:grid-cols-3">
            <FeatureCard
              icon={<RobotIcon className="h-6 w-6" />}
              title="AI-Powered Robots"
              description="Deploy intelligent robots with state-of-the-art machine learning models for autonomous decision making."
            />
            <FeatureCard
              icon={<BlockchainIcon className="h-6 w-6" />}
              title="Blockchain Verified"
              description="All robot actions are recorded on-chain for complete transparency and verifiable automation."
            />
            <FeatureCard
              icon={<CodeIcon className="h-6 w-6" />}
              title="Developer First"
              description="Comprehensive APIs and SDKs to integrate robotics capabilities into your applications."
            />
            <FeatureCard
              icon={<CloudIcon className="h-6 w-6" />}
              title="Cloud Native"
              description="Scale your robot fleet globally with our distributed cloud infrastructure."
            />
            <FeatureCard
              icon={<ShieldIcon className="h-6 w-6" />}
              title="Enterprise Security"
              description="Bank-grade security with end-to-end encryption and secure enclaves."
            />
            <FeatureCard
              icon={<ZapIcon className="h-6 w-6" />}
              title="Real-time Control"
              description="Sub-millisecond latency for real-time robot control and telemetry."
            />
          </div>
        </div>
      </section>

      {/* Stats Section */}
      <section className="border-y border-zinc-800 bg-zinc-900/50 py-16">
        <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          <div className="grid gap-8 md:grid-cols-4">
            {[
              { value: '10K+', label: 'Active Robots' },
              { value: '99.9%', label: 'Uptime SLA' },
              { value: '50M+', label: 'Tasks Completed' },
              { value: '150+', label: 'Enterprise Clients' },
            ].map((stat) => (
              <div key={stat.label} className="text-center">
                <div className="text-4xl font-bold text-white">{stat.value}</div>
                <div className="mt-1 text-sm text-zinc-400">{stat.label}</div>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="bg-zinc-950 py-24">
        <div className="mx-auto max-w-4xl px-4 text-center sm:px-6 lg:px-8">
          <h2 className="text-3xl font-bold text-white sm:text-4xl">
            Ready to build the future?
          </h2>
          <p className="mt-4 text-lg text-zinc-400">
            Join thousands of developers and enterprises building with Hello Universe
          </p>
          <div className="mt-8 flex flex-col items-center justify-center gap-4 sm:flex-row">
            <Link href="/auth/signup">
              <Button variant="primary" size="lg">
                Start Building for Free
              </Button>
            </Link>
            <Link href="/contact">
              <Button variant="ghost" size="lg">
                Contact Sales
              </Button>
            </Link>
          </div>
        </div>
      </section>
    </div>
  );
}

// ============================================
// ICONS
// ============================================

function ArrowRightIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 8l4 4m0 0l-4 4m4-4H3" />
    </svg>
  );
}

function RobotIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" />
    </svg>
  );
}

function BlockchainIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z" />
    </svg>
  );
}

function CodeIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
    </svg>
  );
}

function CloudIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z" />
    </svg>
  );
}

function ShieldIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
    </svg>
  );
}

function ZapIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
    </svg>
  );
}
