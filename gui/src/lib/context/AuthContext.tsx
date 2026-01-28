'use client'

/**
 * Auth Context Provider
 * 
 * Provides authentication state and methods throughout the app
 * Uses Rust backend API instead of Supabase
 */

import { createContext, useContext, useEffect, useState } from 'react'
import { useRouter } from 'next/navigation'
import { apiClient } from '@/lib/api/client'

interface User {
  id: string
  email: string
  name?: string
}

interface Session {
  token: string
  user: User
  expires_at: string
}

interface AuthContextType {
  user: User | null
  session: Session | null
  loading: boolean
  signIn: (email: string, password: string) => Promise<void>
  signUp: (email: string, password: string, name?: string) => Promise<void>
  signOut: () => Promise<void>
}

const AuthContext = createContext<AuthContextType | undefined>(undefined)

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<User | null>(null)
  const [session, setSession] = useState<Session | null>(null)
  const [loading, setLoading] = useState(true)
  const router = useRouter()

  useEffect(() => {
    // Check for existing session token
    const token = localStorage.getItem('codex_auth_token')
    if (token) {
      loadSession(token)
    } else {
      setLoading(false)
    }
  }, [])

  const loadSession = async (token: string) => {
    try {
      const sessionInfo = await apiClient.getSession(token)
      setSession({
        token,
        user: sessionInfo.user,
        expires_at: sessionInfo.expires_at,
      })
      setUser(sessionInfo.user)
    } catch (error) {
      // Invalid token, clear it
      localStorage.removeItem('codex_auth_token')
      setSession(null)
      setUser(null)
    } finally {
      setLoading(false)
    }
  }

  const signIn = async (email: string, password: string) => {
    try {
      const response = await apiClient.login({ email, password })
      
      // Store token
      localStorage.setItem('codex_auth_token', response.token)
      
      // Set session
      const session: Session = {
        token: response.token,
        user: response.user,
        expires_at: new Date(Date.now() + 24 * 60 * 60 * 1000).toISOString(), // 24 hours
      }
      
      setSession(session)
      setUser(response.user)
      
      router.push('/')
      router.refresh()
    } catch (error) {
      throw error
    }
  }

  const signUp = async (email: string, password: string, name?: string) => {
    try {
      const response = await apiClient.register({ email, password, name })
      
      // Store token
      localStorage.setItem('codex_auth_token', response.token)
      
      // Set session
      const session: Session = {
        token: response.token,
        user: response.user,
        expires_at: new Date(Date.now() + 24 * 60 * 60 * 1000).toISOString(), // 24 hours
      }
      
      setSession(session)
      setUser(response.user)
      
      router.push('/')
      router.refresh()
    } catch (error) {
      throw error
    }
  }

  const signOut = async () => {
    try {
      if (session) {
        await apiClient.logout({ session_id: session.token })
      }
    } catch (error) {
      console.error('Logout error:', error)
    } finally {
      localStorage.removeItem('codex_auth_token')
      setSession(null)
      setUser(null)
      router.push('/login')
    }
  }

  return (
    <AuthContext.Provider
      value={{
        user,
        session,
        loading,
        signIn,
        signUp,
        signOut,
      }}
    >
      {children}
    </AuthContext.Provider>
  )
}

export function useAuth() {
  const context = useContext(AuthContext)
  if (context === undefined) {
    throw new Error('useAuth must be used within AuthProvider')
  }
  return context
}
