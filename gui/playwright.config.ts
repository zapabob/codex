import { defineConfig, devices } from '@playwright/test';

/**
 * Playwright configuration for GUI testing with Cursor browser
 * @see https://playwright.dev/docs/test-configuration
 */

export default defineConfig({
  testDir: 'tests',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'html',
  use: {
    baseURL: process.env.GUI_URL || 'http://localhost:1919',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
  },

  projects: [
    {
      name: 'cursor',
      use: {
        ...devices['Desktop Chrome'],
        headless: false,
        viewport: { width: 1920, height: 1080 },
      },
    },
  ],

  // Skip web server since GUI is already running
  webServer: undefined,
});

