'use client';

import * as React from 'react';
import Link from 'next/link';
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { StatsCard } from '@/components/ui/card';
import { useUserStore } from '@/store/useUserStore';

export default function DashboardPage() {
  const { user } = useUserStore();

  return (
    <div className="min-h-screen bg-zinc-950 pt-20">
      <div className="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
        {/* Header */}
        <div className="mb-8">
          <h1 className="text-3xl font-bold text-white">
            Welcome back{user?.username ? `, ${user.username}` : ''}!
          </h1>
          <p className="mt-1 text-zinc-400">
            Here&apos;s what&apos;s happening with your robots today.
          </p>
        </div>

        {/* Stats Grid */}
        <div className="mb-8 grid gap-6 sm:grid-cols-2 lg:grid-cols-4">
          <StatsCard
            label="Active Robots"
            value="12"
            change={{ value: 8, trend: 'up' }}
            icon={<RobotIcon className="h-5 w-5" />}
          />
          <StatsCard
            label="Tasks Completed"
            value="1,234"
            change={{ value: 12, trend: 'up' }}
            icon={<CheckIcon className="h-5 w-5" />}
          />
          <StatsCard
            label="Total Earnings"
            value="$4,521"
            change={{ value: 3.2, trend: 'up' }}
            icon={<DollarIcon className="h-5 w-5" />}
          />
          <StatsCard
            label="System Health"
            value="99.9%"
            change={{ value: 0, trend: 'neutral' }}
            icon={<HeartIcon className="h-5 w-5" />}
          />
        </div>

        {/* Main Content Grid */}
        <div className="grid gap-8 lg:grid-cols-3">
          {/* Recent Activity */}
          <Card variant="elevated" className="lg:col-span-2">
            <CardHeader>
              <CardTitle className="text-white">Recent Activity</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                {[
                  { action: 'Robot HU-001 completed task', time: '2 minutes ago', status: 'success' },
                  { action: 'New staking rewards available', time: '15 minutes ago', status: 'info' },
                  { action: 'Robot HU-003 started patrol', time: '1 hour ago', status: 'info' },
                  { action: 'System update deployed', time: '3 hours ago', status: 'success' },
                  { action: 'Robot HU-002 maintenance complete', time: '5 hours ago', status: 'success' },
                ].map((activity, index) => (
                  <div
                    key={index}
                    className="flex items-center justify-between rounded-lg border border-zinc-800 p-4"
                  >
                    <div className="flex items-center gap-3">
                      <div
                        className={`h-2 w-2 rounded-full ${
                          activity.status === 'success'
                            ? 'bg-emerald-500'
                            : 'bg-cyan-500'
                        }`}
                      />
                      <span className="text-zinc-300">{activity.action}</span>
                    </div>
                    <span className="text-sm text-zinc-500">{activity.time}</span>
                  </div>
                ))}
              </div>
              <Button variant="ghost" className="mt-4 w-full">
                View All Activity
              </Button>
            </CardContent>
          </Card>

          {/* Quick Actions */}
          <Card variant="elevated">
            <CardHeader>
              <CardTitle className="text-white">Quick Actions</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-3">
                <Link href="/ai-labs" className="block">
                  <Button variant="outline" className="w-full justify-start">
                    <BrainIcon className="mr-3 h-4 w-4" />
                    Open AI Labs
                  </Button>
                </Link>
                <Link href="/blockchain" className="block">
                  <Button variant="outline" className="w-full justify-start">
                    <BlockchainIcon className="mr-3 h-4 w-4" />
                    View Blockchain
                  </Button>
                </Link>
                <Button variant="outline" className="w-full justify-start">
                  <PlusIcon className="mr-3 h-4 w-4" />
                  Deploy New Robot
                </Button>
                <Button variant="outline" className="w-full justify-start">
                  <ChartIcon className="mr-3 h-4 w-4" />
                  View Analytics
                </Button>
                <Link href="/user/settings" className="block">
                  <Button variant="outline" className="w-full justify-start">
                    <SettingsIcon className="mr-3 h-4 w-4" />
                    Account Settings
                  </Button>
                </Link>
              </div>
            </CardContent>
          </Card>
        </div>

        {/* Robot Fleet Section */}
        <div className="mt-8">
          <Card variant="elevated">
            <CardHeader className="flex flex-row items-center justify-between">
              <CardTitle className="text-white">Your Robot Fleet</CardTitle>
              <Button variant="primary" size="sm">
                <PlusIcon className="mr-2 h-4 w-4" />
                Add Robot
              </Button>
            </CardHeader>
            <CardContent>
              <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                {[
                  { id: 'HU-001', name: 'Alpha Bot', status: 'active', tasks: 156 },
                  { id: 'HU-002', name: 'Beta Bot', status: 'idle', tasks: 89 },
                  { id: 'HU-003', name: 'Gamma Bot', status: 'active', tasks: 234 },
                ].map((robot) => (
                  <div
                    key={robot.id}
                    className="rounded-xl border border-zinc-800 bg-zinc-900/50 p-4 transition-colors hover:border-cyan-500/50"
                  >
                    <div className="flex items-center justify-between">
                      <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-gradient-to-br from-cyan-500 to-blue-600">
                        <RobotIcon className="h-5 w-5 text-white" />
                      </div>
                      <span
                        className={`rounded-full px-2 py-0.5 text-xs font-medium ${
                          robot.status === 'active'
                            ? 'bg-emerald-500/10 text-emerald-400'
                            : 'bg-zinc-500/10 text-zinc-400'
                        }`}
                      >
                        {robot.status}
                      </span>
                    </div>
                    <h3 className="mt-3 font-medium text-white">{robot.name}</h3>
                    <p className="text-sm text-zinc-500">{robot.id}</p>
                    <div className="mt-2 text-sm text-zinc-400">
                      {robot.tasks} tasks completed
                    </div>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}

// Icons
function RobotIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" />
    </svg>
  );
}

function CheckIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
    </svg>
  );
}

function DollarIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
    </svg>
  );
}

function HeartIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
    </svg>
  );
}

function BrainIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
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

function PlusIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
    </svg>
  );
}

function ChartIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
    </svg>
  );
}

function SettingsIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
    </svg>
  );
}
