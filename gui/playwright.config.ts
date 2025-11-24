import { defineConfig, devices } from '@playwright/test';

/**
 * Playwright configuration for GUI testing with Cursor browser
 * @see https://playwright.dev/docs/test-configuration
 */
export default defineConfig({
  testDir: './tests',
  /* Run tests in files in parallel */
  fullyParallel: true,
  /* Fail the build on CI if you accidentally left test.only in the source code. */
  forbidOnly: !!process.env.CI,
  /* Retry on CI only */
  retries: process.env.CI ? 2 : 0,
  /* Opt out of parallel tests on CI. */
  workers: process.env.CI ? 1 : undefined,
  /* Reporter to use. See https://playwright.dev/docs/test-reporters */
  reporter: 'html',
  /* Shared settings for all the projects below. See https://playwright.dev/docs/api/class-testoptions. */
  use: {
    /* Base URL to use in actions like `await page.goto('/')`. */
    baseURL: process.env.GUI_URL || 'http://localhost:3000',
    /* Collect trace when retrying the failed test. See https://playwright.dev/docs/trace-viewer */
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
  },

  /* Configure projects for major browsers */
  projects: [
    {
      name: 'cursor',
      use: {
        ...devices['Desktop Chrome'],
        // Use Cursor browser executable
        channel: undefined,
        // Cursorブラウザのパス（Chromiumベースなので通常のChromiumを使用）
        // CursorブラウザはChromiumベースなので、Chromiumを使用
        // executablePath: process.env.CURSOR_BROWSER_PATH || 
        //   (process.platform === 'win32' 
        //     ? 'C:\\Users\\downl\\AppData\\Local\\Programs\\cursor\\resources\\app\\bin\\cursor.exe'
        //     : process.platform === 'darwin'
        //     ? '/Applications/Cursor.app/Contents/MacOS/Cursor'
        //     : '/usr/bin/cursor'),
        headless: false, // Show browser for GUI testing
        viewport: { width: 1920, height: 1080 },
      },
    },
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
    {
      name: 'firefox',
      use: { ...devices['Desktop Firefox'] },
    },
    {
      name: 'webkit',
      use: { ...devices['Desktop Safari'] },
    },
  ],

  /* Run your local dev server before starting the tests */
  // 既存のGUIサーバーを使用する場合は、webServerを無効化
  webServer: process.env.SKIP_WEBSERVER === '1' ? undefined : {
    command: 'npm run dev',
    url: 'http://localhost:3000',
    reuseExistingServer: true, // 既存のサーバーを再利用
    timeout: 120 * 1000,
    stdout: 'ignore',
    stderr: 'pipe',
  },
});

