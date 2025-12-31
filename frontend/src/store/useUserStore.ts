// Hello Universe - User Store
// Zustand store for user authentication and profile state

import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';
import type { User, UserPreferences } from '@/types/index.d';

// ============================================
// USER STORE STATE
// ============================================

export interface UserState {
  // User data
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  setUser: (user: User | null) => void;
  updateUser: (updates: Partial<User>) => void;
  updatePreferences: (preferences: Partial<UserPreferences>) => void;
  setLoading: (isLoading: boolean) => void;
  setError: (error: string | null) => void;
  logout: () => void;
  
  // Auth actions
  login: (email: string, password: string) => Promise<void>;
  signup: (email: string, password: string, username: string) => Promise<void>;
  refreshSession: () => Promise<void>;
}

// ============================================
// DEFAULT VALUES
// ============================================

const defaultPreferences: UserPreferences = {
  theme: 'system',
  notifications: true,
  language: 'en',
  newsletter: false,
};

// ============================================
// USER STORE
// ============================================

export const useUserStore = create<UserState>()(
  persist(
    (set, get) => ({
      // Initial state
      user: null,
      isAuthenticated: false,
      isLoading: false,
      error: null,

      // Set user
      setUser: (user) =>
        set({
          user,
          isAuthenticated: !!user,
          error: null,
        }),

      // Update user fields
      updateUser: (updates) =>
        set((state) => ({
          user: state.user
            ? { ...state.user, ...updates, updatedAt: new Date() }
            : null,
        })),

      // Update preferences
      updatePreferences: (preferences) =>
        set((state) => ({
          user: state.user
            ? {
                ...state.user,
                preferences: { ...state.user.preferences, ...preferences },
                updatedAt: new Date(),
              }
            : null,
        })),

      // Set loading state
      setLoading: (isLoading) => set({ isLoading }),

      // Set error state
      setError: (error) => set({ error }),

      // Logout
      logout: () =>
        set({
          user: null,
          isAuthenticated: false,
          error: null,
        }),

      // Login action
      login: async (email, password) => {
        set({ isLoading: true, error: null });

        try {
          const response = await fetch('/api/auth/login', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ email, password }),
          });

          if (!response.ok) {
            const data = await response.json();
            throw new Error(data.message || 'Login failed');
          }

          const { user } = await response.json();
          
          set({
            user: {
              ...user,
              preferences: user.preferences || defaultPreferences,
            },
            isAuthenticated: true,
            isLoading: false,
          });
        } catch (error) {
          set({
            error: error instanceof Error ? error.message : 'Login failed',
            isLoading: false,
          });
          throw error;
        }
      },

      // Signup action
      signup: async (email, password, username) => {
        set({ isLoading: true, error: null });

        try {
          const response = await fetch('/api/auth/signup', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ email, password, username }),
          });

          if (!response.ok) {
            const data = await response.json();
            throw new Error(data.message || 'Signup failed');
          }

          const { user } = await response.json();
          
          set({
            user: {
              ...user,
              preferences: defaultPreferences,
            },
            isAuthenticated: true,
            isLoading: false,
          });
        } catch (error) {
          set({
            error: error instanceof Error ? error.message : 'Signup failed',
            isLoading: false,
          });
          throw error;
        }
      },

      // Refresh session
      refreshSession: async () => {
        try {
          const response = await fetch('/api/auth/session');
          
          if (!response.ok) {
            get().logout();
            return;
          }

          const { user } = await response.json();
          
          if (user) {
            set({
              user: {
                ...user,
                preferences: user.preferences || defaultPreferences,
              },
              isAuthenticated: true,
            });
          } else {
            get().logout();
          }
        } catch {
          get().logout();
        }
      },
    }),
    {
      name: 'hello-universe-user',
      storage: createJSONStorage(() => localStorage),
      partialize: (state) => ({
        user: state.user,
        isAuthenticated: state.isAuthenticated,
      }),
    }
  )
);

// ============================================
// SELECTORS
// ============================================

export const selectUser = (state: UserState) => state.user;
export const selectIsAuthenticated = (state: UserState) => state.isAuthenticated;
export const selectUserPreferences = (state: UserState) => state.user?.preferences;

export default useUserStore;
