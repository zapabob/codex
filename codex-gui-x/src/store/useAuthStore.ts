import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware';
import type { AuthState, AuthProvider, User } from '../types/mcp';

interface AuthStoreState extends AuthState {
  isLoading: boolean;
  error: string | null;
  pendingOAuthParams: { provider: AuthProvider; code: string; state: string } | null;

  setAuthState: (state: Partial<AuthState>) => void;
  setAccessToken: (token: string | null, expiresAt?: Date | null) => void;
  setRefreshToken: (token: string | null) => void;
  setUser: (user: User | null) => void;
  setProvider: (provider: AuthProvider | null) => void;
  setMode: (mode: 'oauth2' | 'api-key' | 'anonymous') => void;

  setApiKey: (apiKey: string | null) => void;

  setIsLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  setPendingOAuthParams: (params: { provider: AuthProvider; code: string; state: string } | null) => void;

  login: (provider: AuthProvider) => Promise<void>;
  handleOAuthCallback: (params: { code: string; state: string }) => Promise<void>;
  logout: () => void;

  refreshToken: () => Promise<void>;
}

export const useAuthStore = create<AuthStoreState>()(
  devtools(
    persist(
      (set, get) => ({
        isAuthenticated: false,
        mode: 'anonymous',
        provider: null,
        accessToken: null,
        refreshToken: null,
        expiresAt: null,
        user: null,
        apiKey: null,
        isLoading: false,
        error: null,
        pendingOAuthParams: null,

        setAuthState: (state) => set((prev) => ({
          ...prev,
          ...state,
        })),
        setAccessToken: (token, expiresAt) => set({
          accessToken: token,
          expiresAt,
          isAuthenticated: !!token,
        }),
        setRefreshToken: (token) => set({ refreshToken: token }),
        setUser: (user) => set({ user }),
        setProvider: (provider) => set({ provider }),
        setMode: (mode) => set({ mode }),

        setApiKey: (apiKey) => set({
          apiKey,
          isAuthenticated: !!apiKey,
          mode: apiKey ? 'api-key' : 'anonymous',
        }),

        setIsLoading: (loading) => set({ isLoading: loading }),
        setError: (error) => set({ error }),
        setPendingOAuthParams: (params) => set({ pendingOAuthParams: params }),

        login: async (provider) => {
          set({ isLoading: true, error: null });

          try {
            const authUrl = getOAuthAuthorizationUrl(provider);
            window.location.href = authUrl;
          } catch (error) {
            set({
              isLoading: false,
              error: error instanceof Error ? error.message : 'Login failed',
            });
          }
        },

        handleOAuthCallback: async (params) => {
          set({ isLoading: true, error: null });

          try {
            const pending = get().pendingOAuthParams;
            if (!pending || pending.provider !== 'openai') {
              throw new Error('Invalid OAuth state');
            }

            const response = await fetch('/api/auth/oauth/callback', {
              method: 'POST',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify({
                code: params.code,
                state: params.state,
                provider: pending.provider,
              }),
            });

            if (!response.ok) {
              throw new Error('OAuth callback failed');
            }

            const { accessToken, refreshToken, expiresAt, user } = await response.json();

            set({
              isAuthenticated: true,
              mode: 'oauth2',
              provider: pending.provider,
              accessToken,
              refreshToken,
              expiresAt: expiresAt ? new Date(expiresAt) : null,
              user,
              isLoading: false,
              pendingOAuthParams: null,
            });
          } catch (error) {
            set({
              isLoading: false,
              error: error instanceof Error ? error.message : 'Authentication failed',
            });
            throw error;
          }
        },

        logout: () => set({
          isAuthenticated: false,
          mode: 'anonymous',
          provider: null,
          accessToken: null,
          refreshToken: null,
          expiresAt: null,
          user: null,
          apiKey: null,
          error: null,
        }),

        refreshToken: async () => {
          const refreshToken = get().refreshToken;
          if (!refreshToken) {
            throw new Error('No refresh token available');
          }

          set({ isLoading: true, error: null });

          try {
            const response = await fetch('/api/auth/refresh', {
              method: 'POST',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify({ refreshToken }),
            });

            if (!response.ok) {
              throw new Error('Token refresh failed');
            }

            const { accessToken, expiresAt } = await response.json();

            set({
              accessToken,
              expiresAt: expiresAt ? new Date(expiresAt) : null,
              isLoading: false,
            });
          } catch (error) {
            set({
              isLoading: false,
              error: error instanceof Error ? error.message : 'Token refresh failed',
            });
            throw error;
          }
        },
      }),
      {
        name: 'codex-auth-store',
        partialize: (state) => ({
          isAuthenticated: state.isAuthenticated,
          mode: state.mode,
          provider: state.provider,
          user: state.user,
          expiresAt: state.expiresAt,
        }),
      }
    ),
    { name: 'AuthStore' }
  )
);

function getOAuthAuthorizationUrl(provider: AuthProvider): string {
  const endpoints: Record<AuthProvider, string> = {
    openai: 'https://platform.openai.com/oauth',
    google: 'https://accounts.google.com/o/oauth2/v2/auth',
    github: 'https://github.com/login/oauth/authorize',
  };

  const clientId = {
    openai: import.meta.env.VITE_OPENAI_CLIENT_ID,
    google: import.meta.env.VITE_GOOGLE_CLIENT_ID,
    github: import.meta.env.VITE_GITHUB_CLIENT_ID,
  }[provider];

  const redirectUri = `${window.location.origin}/auth/callback/${provider}`;

  const scopes: Record<AuthProvider, string[]> = {
    openai: ['openai', 'profile', 'email'],
    google: ['openid', 'email', 'profile'],
    github: ['read:user', 'user:email'],
  };

  const state = crypto.randomUUID();
  sessionStorage.setItem('oauth_state', state);

  const params = new URLSearchParams({
    client_id: clientId || '',
    redirect_uri: redirectUri,
    response_type: 'code',
    scope: scopes[provider].join(' '),
    state,
  });

  if (provider === 'google') {
    params.set('prompt', 'consent');
  }

  return `${endpoints[provider]}?${params.toString()}`;
}
