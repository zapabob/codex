import { test, expect } from '@playwright/test'

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8787'
const BACKEND_AVAILABLE = process.env.BACKEND_AVAILABLE === 'true'

test.describe('GUI Integration Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Use baseURL from playwright config (localhost:1919)
    await page.goto('/')
  })

  test('should load dashboard', async ({ page }) => {
    // Dashboard uses MUI components without h1, check main container
    await expect(page.locator('body')).toBeVisible()
    // Check page loaded (any meaningful content)
    await page.waitForLoadState('networkidle')
    const content = await page.locator('main, [role="main"], .MuiContainer-root').first()
    await expect(content).toBeVisible()
  })

  test('should display plans page', async ({ page }) => {
    await page.goto('/plans')
    await page.waitForLoadState('networkidle')
    // Check page rendered
    await expect(page.locator('body')).toBeVisible()
  })

  test('should display visualization page', async ({ page }) => {
    await page.goto('/visualization')
    await page.waitForLoadState('networkidle')
    await expect(page.locator('body')).toBeVisible()
  })

  test('should display VR page', async ({ page }) => {
    await page.goto('/vr')
    await page.waitForLoadState('networkidle')
    await expect(page.locator('body')).toBeVisible()
  })

  test('should handle login page render', async ({ page }) => {
    await page.goto('/login')
    await page.waitForLoadState('networkidle')
    // Check login page has form elements
    const emailInput = page.locator('input[type="email"], input[type="text"]').first()
    const passwordInput = page.locator('input[type="password"]').first()
    // At least one input should be visible
    const hasEmail = await emailInput.isVisible().catch(() => false)
    const hasPassword = await passwordInput.isVisible().catch(() => false)
    expect(hasEmail || hasPassword).toBeTruthy()
  })
})

test.describe('API Integration Tests', () => {
  test.skip(!BACKEND_AVAILABLE, 'Rust backend not running (set BACKEND_AVAILABLE=true to enable)')

  test('should connect to Rust backend', async ({ request }) => {
    const response = await request.get(`${API_URL}/api/actions`)
    expect(response.ok()).toBeTruthy()
  })

  test('should get system metrics', async ({ request }) => {
    const response = await request.get(`${API_URL}/api/system/metrics`)
    expect(response.ok()).toBeTruthy()

    const data = await response.json()
    expect(data).toHaveProperty('cpu_usage')
    expect(data).toHaveProperty('memory_usage')
  })

  test('should list plans', async ({ request }) => {
    const response = await request.get(`${API_URL}/api/plans`)
    expect(response.ok()).toBeTruthy()

    const data = await response.json()
    expect(Array.isArray(data)).toBeTruthy()
  })

  test('should get VR status', async ({ request }) => {
    const response = await request.get(`${API_URL}/api/vr/status`)
    expect(response.ok()).toBeTruthy()

    const data = await response.json()
    expect(data).toHaveProperty('supported')
  })
})
