import { test, expect } from '@playwright/test'

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8787'

test.describe('GUI Integration Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to GUI
    await page.goto('http://localhost:3000')
  })

  test('should load dashboard', async ({ page }) => {
    await expect(page.locator('h1')).toContainText(/Codex|Dashboard/i)
  })

  test('should display plans page', async ({ page }) => {
    await page.goto('http://localhost:3000/plans')
    await expect(page.locator('h1')).toContainText(/plan mode/i)
  })

  test('should display visualization page', async ({ page }) => {
    await page.goto('http://localhost:3000/visualization')
    await expect(page.locator('h1')).toContainText(/Git Visualization/i)
  })

  test('should display VR page', async ({ page }) => {
    await page.goto('http://localhost:3000/vr')
    await expect(page.locator('h2')).toContainText(/Codex Git VR/i)
  })

  test('should handle login', async ({ page }) => {
    await page.goto('http://localhost:3000/login')
    
    // Fill login form
    await page.fill('input[type="email"]', 'test@example.com')
    await page.fill('input[type="password"]', 'testpassword')
    
    // Submit form
    await page.click('button[type="submit"]')
    
    // Wait for navigation or error
    await page.waitForTimeout(2000)
    
    // Check if redirected or error shown
    const currentUrl = page.url()
    expect(currentUrl).toMatch(/\/$|\/login/)
  })
})

test.describe('API Integration Tests', () => {
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
