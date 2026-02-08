import type { AuthState, AuthProvider, TokenPayload, User } from '../types/mcp';

interface OAuth2Config {
  clientId: string;
  clientSecret?: string;
  redirectUri: string;
  scope: string[];
  authorizationEndpoint: string;
  tokenEndpoint: string;
}

interface AuthConfig {
  providers: Record<AuthProvider, OAuth2Config>;
  storageKey: string;
  tokenRefreshThresholdMs: number;
}

const DEFAULT_CONFIG: AuthConfig = {
  providers: {
    openai: {
      clientId: import.meta.env.VITE_OPENAI_CLIENT_ID || '',
      clientSecret: import.meta.env.VITE_OPENAI_CLIENT_SECRET || '',
      redirectUri: `${window.location.origin}/auth/callback/openai`,
      scope: ['openai', 'profile', 'email'],
      authorizationEndpoint: 'https://platform.openai.com/oauth',
      tokenEndpoint: 'https://api.openai.com/v1/oauth/token',
    },
    google: {
      clientId: import.meta.env.VITE_GOOGLE_CLIENT_ID || '',
      redirectUri: `${window.location.origin}/auth/callback/google`,
      scope: ['openid', 'email', 'profile'],
      authorizationEndpoint: 'https://accounts.google.com/o/oauth2/v2/auth',
      tokenEndpoint: 'https://oauth2.googleapis.com/token',
    },
    github: {
      clientId: import.meta.env.VITE_GITHUB_CLIENT_ID || '',
      redirectUri: `${window.location.origin}/auth/callback/github`,
      scope: ['read:user', 'user:email'],
      authorizationEndpoint: 'https://github.com/login/oauth/authorize',
      tokenEndpoint: 'https://github.com/login/oauth/access_token',
    },
  },
  storageKey: 'codex_auth_state',
  tokenRefreshThresholdMs: 300000,
};

interface PendingOAuthParams {
  code: string;
  state: string;
  provider: AuthProvider;
}

export class AuthService {
  private config: AuthConfig;
  private authState: AuthState;
  private refreshTimer: ReturnType<typeof setTimeout> | null = null;
  private listeners: Set<(state: AuthState) => void> = new Set();
  private pkceStates: Map<string, { provider: AuthProvider; nonce: string }> = new Map();

  constructor(config?: Partial<AuthConfig>) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    this.authState = this.loadState();
  }

  private loadState(): AuthState {
    try {
      const stored = localStorage.getItem(this.config.storageKey);
      if (stored) {
        const parsed = JSON.parse(stored);
        return {
          isAuthenticated: parsed.isAuthenticated ?? false,
          mode: parsed.mode ?? 'anonymous',
          provider: parsed.provider ?? null,
          accessToken: parsed.accessToken ?? null,
          refreshToken: parsed.refreshToken ?? null,
          expiresAt: parsed.expiresAt ?? null,
          user: parsed.user ?? null,
          apiKey: parsed.apiKey ?? null,
        };
      }
    } catch (error) {
      console.error('[Auth] Failed to load state:', error);
    }

    return {
      isAuthenticated: false,
      mode: 'anonymous',
      provider: null,
      accessToken: null,
      refreshToken: null,
      expiresAt: null,
      user: null,
      apiKey: null,
    };
  }

  private saveState(): void {
    try {
      localStorage.setItem(this.config.storageKey, JSON.stringify({
        isAuthenticated: this.authState.isAuthenticated,
        mode: this.authState.mode,
        provider: this.authState.provider,
        accessToken: this.authState.accessToken,
        refreshToken: this.authState.refreshToken,
        expiresAt: this.authState.expiresAt,
        user: this.authState.user,
        apiKey: this.authState.apiKey,
      }));
    } catch (error) {
      console.error('[Auth] Failed to save state:', error);
    }
  }

  private notifyListeners(): void {
    this.listeners.forEach(listener => listener(this.authState));
  }

  private async generateCodeVerifier(): Promise<string> {
    const array = new Uint8Array(32);
    crypto.getRandomValues(array);
    return btoa(String.fromCharCode(...array))
      .replace(/\+/g, '-')
      .replace(/\//g, '_')
      .replace(/=/g, '');
  }

  private async generateCodeChallenge(verifier: string): Promise<string> {
    const encoder = new TextEncoder();
    const data = encoder.encode(verifier);
    const hash = await crypto.subtle.digest('SHA-256', data);
    return btoa(String.fromCharCode(...new Uint8Array(hash)))
      .replace(/\+/g, '-')
      .replace(/\//g, '_')
      .replace(/=/g, '');
  }

  private generateNonce(): string {
    const array = new Uint8Array(16);
    crypto.getRandomValues(array);
    return Array.from(array, byte => byte.toString(16).padStart(2, '0')).join('');
  }

  getAuthUrl(provider: AuthProvider): string {
    const providerConfig = this.config.providers[provider];
    if (!providerConfig) {
      throw new Error(`Unknown auth provider: ${provider}`);
    }

    const state = crypto.randomUUID();
    const nonce = this.generateNonce();

    this.pkceStates.set(state, { provider, nonce });

    const params = new URLSearchParams({
      client_id: providerConfig.clientId,
      redirect_uri: providerConfig.redirectUri,
      response_type: 'code',
      scope: providerConfig.scope.join(' '),
      state,
      nonce,
    });

    if (provider === 'google') {
      params.set('prompt', 'consent');
    }

    return `${providerConfig.authorizationEndpoint}?${params.toString()}`;
  }

  async handleOAuthCallback(params: PendingOAuthParams): Promise<void> {
    const storedState = this.pkceStates.get(params.state);
    if (!storedState) {
      throw new Error('Invalid OAuth state');
    }
    this.pkceStates.delete(params.state);

    const providerConfig = this.config.providers[params.provider];
    if (!providerConfig) {
      throw new Error(`Unknown auth provider: ${params.provider}`);
    }

    const tokenResponse = await fetch(providerConfig.tokenEndpoint, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
      },
      body: JSON.stringify({
        grant_type: 'authorization_code',
        code: params.code,
        client_id: providerConfig.clientId,
        client_secret: providerConfig.clientSecret,
        redirect_uri: providerConfig.redirectUri,
      }),
    });

    if (!tokenResponse.ok) {
      const error = await tokenResponse.json();
      throw new Error(error.error_description || 'OAuth token exchange failed');
    }

    const tokens = await tokenResponse.json();

    const user = await this.fetchUserProfile(params.provider, tokens.access_token);

    this.authState = {
      isAuthenticated: true,
      mode: 'oauth2',
      provider: params.provider,
      accessToken: tokens.access_token,
      refreshToken: tokens.refresh_token ?? null,
      expiresAt: tokens.expires_at ? new Date(tokens.expires_at * 1000) : null,
      user,
      apiKey: null,
    };

    if (tokens.refresh_token) {
      this.scheduleTokenRefresh(tokens.expires_in);
    }

    this.saveState();
    this.notifyListeners();
  }

  private async fetchUserProfile(provider: AuthProvider, accessToken: string): Promise<User> {
    const endpoints: Record<AuthProvider, string> = {
      openai: 'https://api.openai.com/v1/me',
      google: 'https://www.googleapis.com/oauth2/v2/userinfo',
      github: 'https://api.github.com/user',
    };

    const response = await fetch(endpoints[provider], {
      headers: {
        'Authorization': `Bearer ${accessToken}`,
        'Accept': 'application/json',
      },
    });

    if (!response.ok) {
      throw new Error('Failed to fetch user profile');
    }

    const profile = await response.json();

    return {
      id: profile.id,
      email: profile.email,
      name: profile.name || profile.login,
      avatar: profile.picture || profile.avatar_url,
      provider,
    };
  }

  setApiKey(apiKey: string): void {
    this.authState = {
      isAuthenticated: true,
      mode: 'api-key',
      provider: null,
      accessToken: null,
      refreshToken: null,
      expiresAt: null,
      user: null,
      apiKey,
    };
    this.saveState();
    this.notifyListeners();
  }

  async refreshAccessToken(): Promise<void> {
    if (!this.authState.refreshToken || !this.authState.provider) {
      throw new Error('No refresh token available');
    }

    const providerConfig = this.config.providers[this.authState.provider];
    if (!providerConfig) {
      throw new Error(`Unknown auth provider: ${this.authState.provider}`);
    }

    const tokenResponse = await fetch(providerConfig.tokenEndpoint, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
      },
      body: JSON.stringify({
        grant_type: 'refresh_token',
        refresh_token: this.authState.refreshToken,
        client_id: providerConfig.clientId,
        client_secret: providerConfig.clientSecret,
      }),
    });

    if (!tokenResponse.ok) {
      this.logout();
      throw new Error('Token refresh failed');
    }

    const tokens = await tokenResponse.json();

    this.authState = {
      ...this.authState,
      accessToken: tokens.access_token,
      refreshToken: tokens.refresh_token ?? this.authState.refreshToken,
      expiresAt: tokens.expires_at ? new Date(tokens.expires_at * 1000) : null,
    };

    if (tokens.refresh_token) {
      this.scheduleTokenRefresh(tokens.expires_in);
    }

    this.saveState();
    this.notifyListeners();
  }

  private scheduleTokenRefresh(expiresInSeconds: number): void {
    this.stopTokenRefresh();

    const refreshTime = (expiresInSeconds * 1000) - this.config.tokenRefreshThresholdMs;
    if (refreshTime > 0) {
      this.refreshTimer = setTimeout(() => {
        this.refreshAccessToken().catch(error => {
          console.error('[Auth] Auto-refresh failed:', error);
        });
      }, refreshTime);
    }
  }

  private stopTokenRefresh(): void {
    if (this.refreshTimer) {
      clearTimeout(this.refreshTimer);
      this.refreshTimer = null;
    }
  }

  logout(): void {
    this.authState = {
      isAuthenticated: false,
      mode: 'anonymous',
      provider: null,
      accessToken: null,
      refreshToken: null,
      expiresAt: null,
      user: null,
      apiKey: null,
    };
    this.stopTokenRefresh();
    localStorage.removeItem(this.config.storageKey);
    this.notifyListeners();
  }

  getState(): AuthState {
    return { ...this.authState };
  }

  getAccessToken(): string | null {
    return this.authState.accessToken;
  }

  getApiKey(): string | null {
    return this.authState.apiKey;
  }

  getAuthorizationHeader(): string | null {
    if (this.authState.mode === 'oauth2' && this.authState.accessToken) {
      return `Bearer ${this.authState.accessToken}`;
    }
    if (this.authState.mode === 'api-key' && this.authState.apiKey) {
      return `Bearer ${this.authState.apiKey}`;
    }
    return null;
  }

  isAuthenticated(): boolean {
    return this.authState.isAuthenticated;
  }

  subscribe(listener: (state: AuthState) => void): () => void {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }

  destroy(): void {
    this.stopTokenRefresh();
    this.listeners.clear();
  }
}

let authInstance: AuthService | null = null;

export function getAuth(config?: Partial<AuthConfig>): AuthService {
  if (!authInstance) {
    authInstance = new AuthService(config);
  }
  return authInstance;
}

export function resetAuth(): void {
  authInstance?.destroy();
  authInstance = null;
}

export { AuthService as default };
