import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests/playwright',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'html',
  use: {
    baseURL: 'http://localhost:3001',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
    {
      name: 'cursor',
      use: {
        ...devices['Desktop Chrome'],
        channel: 'chrome',
        executablePath: process.env.CURSOR_EXECUTABLE_PATH || undefined,
      },
    },
  ],
  webServer: {
    command: 'codex-gui-new.exe --port 8787',
    url: 'http://localhost:8787',
    reuseExistingServer: !process.env.CI,
  },
});
