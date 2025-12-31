'use client';

import * as React from 'react';
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input, PasswordInput, Textarea } from '@/components/ui/input';
import { useUserStore } from '@/store/useUserStore';

type TabId = 'profile' | 'security' | 'preferences' | 'wallet';

export default function SettingsPage() {
  const { user, updateUser, updatePreferences } = useUserStore();
  const [activeTab, setActiveTab] = React.useState<TabId>('profile');
  const [isSaving, setIsSaving] = React.useState(false);

  const tabs: { id: TabId; label: string; icon: React.ReactNode }[] = [
    { id: 'profile', label: 'Profile', icon: <UserIcon className="h-4 w-4" /> },
    { id: 'security', label: 'Security', icon: <ShieldIcon className="h-4 w-4" /> },
    { id: 'preferences', label: 'Preferences', icon: <SettingsIcon className="h-4 w-4" /> },
    { id: 'wallet', label: 'Wallet', icon: <WalletIcon className="h-4 w-4" /> },
  ];

  const handleSave = async () => {
    setIsSaving(true);
    // Simulate API call
    await new Promise((resolve) => setTimeout(resolve, 1000));
    setIsSaving(false);
  };

  return (
    <div className="min-h-screen bg-zinc-950 pt-20">
      <div className="mx-auto max-w-4xl px-4 py-8 sm:px-6 lg:px-8">
        <h1 className="mb-8 text-3xl font-bold text-white">Settings</h1>

        <div className="flex flex-col gap-8 lg:flex-row">
          {/* Sidebar */}
          <div className="lg:w-56">
            <nav className="flex flex-row gap-1 lg:flex-col">
              {tabs.map((tab) => (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id)}
                  className={`flex items-center gap-3 rounded-lg px-4 py-2.5 text-sm font-medium transition-colors ${
                    activeTab === tab.id
                      ? 'bg-zinc-800 text-white'
                      : 'text-zinc-400 hover:bg-zinc-800/50 hover:text-white'
                  }`}
                >
                  {tab.icon}
                  <span className="hidden sm:inline">{tab.label}</span>
                </button>
              ))}
            </nav>
          </div>

          {/* Content */}
          <div className="flex-1">
            {activeTab === 'profile' && (
              <Card variant="elevated">
                <CardHeader>
                  <CardTitle className="text-white">Profile Settings</CardTitle>
                </CardHeader>
                <CardContent className="space-y-6">
                  {/* Avatar */}
                  <div className="flex items-center gap-6">
                    <div className="flex h-20 w-20 items-center justify-center rounded-full bg-gradient-to-br from-cyan-500 to-blue-600 text-2xl font-bold text-white">
                      {user?.username?.charAt(0).toUpperCase() || 'U'}
                    </div>
                    <div>
                      <Button variant="outline" size="sm">
                        Change Avatar
                      </Button>
                      <p className="mt-1 text-xs text-zinc-500">
                        JPG, PNG or GIF. Max 2MB.
                      </p>
                    </div>
                  </div>

                  <Input
                    label="Username"
                    defaultValue={user?.username || ''}
                    placeholder="johndoe"
                  />

                  <Input
                    label="Email"
                    type="email"
                    defaultValue={user?.email || ''}
                    placeholder="you@example.com"
                  />

                  <Textarea
                    label="Bio"
                    placeholder="Tell us about yourself..."
                    hint="Max 500 characters"
                  />

                  <div className="flex justify-end">
                    <Button variant="primary" onClick={handleSave} isLoading={isSaving}>
                      Save Changes
                    </Button>
                  </div>
                </CardContent>
              </Card>
            )}

            {activeTab === 'security' && (
              <Card variant="elevated">
                <CardHeader>
                  <CardTitle className="text-white">Security Settings</CardTitle>
                </CardHeader>
                <CardContent className="space-y-6">
                  <div className="rounded-lg border border-zinc-800 p-4">
                    <h3 className="font-medium text-white">Change Password</h3>
                    <p className="mt-1 text-sm text-zinc-400">
                      Update your password to keep your account secure.
                    </p>
                    <div className="mt-4 space-y-4">
                      <PasswordInput
                        label="Current Password"
                        placeholder="Enter current password"
                      />
                      <PasswordInput
                        label="New Password"
                        placeholder="Enter new password"
                      />
                      <PasswordInput
                        label="Confirm New Password"
                        placeholder="Confirm new password"
                      />
                    </div>
                  </div>

                  <div className="rounded-lg border border-zinc-800 p-4">
                    <div className="flex items-center justify-between">
                      <div>
                        <h3 className="font-medium text-white">Two-Factor Authentication</h3>
                        <p className="mt-1 text-sm text-zinc-400">
                          Add an extra layer of security to your account.
                        </p>
                      </div>
                      <Button variant="outline" size="sm">
                        Enable
                      </Button>
                    </div>
                  </div>

                  <div className="rounded-lg border border-red-900/50 bg-red-950/20 p-4">
                    <h3 className="font-medium text-red-400">Danger Zone</h3>
                    <p className="mt-1 text-sm text-zinc-400">
                      Permanently delete your account and all associated data.
                    </p>
                    <Button variant="destructive" size="sm" className="mt-4">
                      Delete Account
                    </Button>
                  </div>

                  <div className="flex justify-end">
                    <Button variant="primary" onClick={handleSave} isLoading={isSaving}>
                      Save Changes
                    </Button>
                  </div>
                </CardContent>
              </Card>
            )}

            {activeTab === 'preferences' && (
              <Card variant="elevated">
                <CardHeader>
                  <CardTitle className="text-white">Preferences</CardTitle>
                </CardHeader>
                <CardContent className="space-y-6">
                  <div className="rounded-lg border border-zinc-800 p-4">
                    <h3 className="font-medium text-white">Theme</h3>
                    <p className="mt-1 text-sm text-zinc-400">
                      Choose your preferred color scheme.
                    </p>
                    <div className="mt-4 flex gap-3">
                      {['Light', 'Dark', 'System'].map((theme) => (
                        <button
                          key={theme}
                          className={`rounded-lg border px-4 py-2 text-sm font-medium transition-colors ${
                            theme === 'Dark'
                              ? 'border-cyan-500 bg-cyan-500/10 text-cyan-400'
                              : 'border-zinc-700 text-zinc-400 hover:border-zinc-600'
                          }`}
                        >
                          {theme}
                        </button>
                      ))}
                    </div>
                  </div>

                  <div className="rounded-lg border border-zinc-800 p-4">
                    <div className="flex items-center justify-between">
                      <div>
                        <h3 className="font-medium text-white">Email Notifications</h3>
                        <p className="mt-1 text-sm text-zinc-400">
                          Receive updates about your robots and account.
                        </p>
                      </div>
                      <label className="relative inline-flex cursor-pointer items-center">
                        <input type="checkbox" className="peer sr-only" defaultChecked />
                        <div className="peer h-6 w-11 rounded-full bg-zinc-700 after:absolute after:left-[2px] after:top-[2px] after:h-5 after:w-5 after:rounded-full after:bg-white after:transition-all peer-checked:bg-cyan-500 peer-checked:after:translate-x-full"></div>
                      </label>
                    </div>
                  </div>

                  <div className="rounded-lg border border-zinc-800 p-4">
                    <div className="flex items-center justify-between">
                      <div>
                        <h3 className="font-medium text-white">Newsletter</h3>
                        <p className="mt-1 text-sm text-zinc-400">
                          Get the latest news and updates from Hello Universe.
                        </p>
                      </div>
                      <label className="relative inline-flex cursor-pointer items-center">
                        <input type="checkbox" className="peer sr-only" />
                        <div className="peer h-6 w-11 rounded-full bg-zinc-700 after:absolute after:left-[2px] after:top-[2px] after:h-5 after:w-5 after:rounded-full after:bg-white after:transition-all peer-checked:bg-cyan-500 peer-checked:after:translate-x-full"></div>
                      </label>
                    </div>
                  </div>

                  <div className="flex justify-end">
                    <Button variant="primary" onClick={handleSave} isLoading={isSaving}>
                      Save Changes
                    </Button>
                  </div>
                </CardContent>
              </Card>
            )}

            {activeTab === 'wallet' && (
              <Card variant="elevated">
                <CardHeader>
                  <CardTitle className="text-white">Connected Wallet</CardTitle>
                </CardHeader>
                <CardContent className="space-y-6">
                  <div className="rounded-lg border border-zinc-800 p-6 text-center">
                    <WalletIcon className="mx-auto h-12 w-12 text-zinc-500" />
                    <h3 className="mt-4 font-medium text-white">No Wallet Connected</h3>
                    <p className="mt-1 text-sm text-zinc-400">
                      Connect your wallet to access blockchain features.
                    </p>
                    <Button variant="primary" className="mt-4">
                      Connect Wallet
                    </Button>
                  </div>
                </CardContent>
              </Card>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

// Icons
function UserIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
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

function SettingsIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
    </svg>
  );
}

function WalletIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 7V4a1 1 0 00-1-1H5a2 2 0 000 4h15a1 1 0 011 1v4h-3a2 2 0 000 4h3a1 1 0 001-1v-2a1 1 0 00-1-1M3 5v14a2 2 0 002 2h15a1 1 0 001-1v-4" />
    </svg>
  );
}
