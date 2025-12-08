import { test, expect } from '@playwright/test';

test.describe('Codex GUI Basic Functionality', () => {
  test.beforeEach(async ({ page }) => {
    // Wait for the server to be ready
    await page.waitForTimeout(2000);
  });

  test('should load main page', async ({ page }) => {
    await page.goto('/');
    await expect(page).toHaveTitle(/Codex/);
    await expect(page.locator('h1')).toContainText('Codex');
  });

  test('should navigate to agents page', async ({ page }) => {
    await page.goto('/');
    await page.click('text=Agents');
    await expect(page).toHaveURL(/.*agents/);
    await expect(page.locator('h1')).toContainText('AI Agents');
  });

  test('should navigate to AI tools page', async ({ page }) => {
    await page.goto('/');
    await page.click('text=AI Tools');
    await expect(page).toHaveURL(/.*ai-tools/);
    await expect(page.locator('h1')).toContainText('AI Tools');
  });

  test('should navigate to code execution page', async ({ page }) => {
    await page.goto('/');
    await page.click('text=Code');
    await expect(page).toHaveURL(/.*code/);
    await expect(page.locator('h1')).toContainText('Code Execution');
  });

  test('should navigate to MCP management page', async ({ page }) => {
    await page.goto('/');
    await page.click('text=MCP');
    await expect(page).toHaveURL(/.*mcp/);
    await expect(page.locator('h1')).toContainText('MCP Servers');
  });

  test('should navigate to quality control page', async ({ page }) => {
    await page.goto('/');
    await page.click('text=QC');
    await expect(page).toHaveURL(/.*qc/);
    await expect(page.locator('h1')).toContainText('Quality Control');
  });

  test('should navigate to task management page', async ({ page }) => {
    await page.goto('/');
    await page.click('text=Tasks');
    await expect(page).toHaveURL(/.*tasks/);
    await expect(page.locator('h1')).toContainText('Task Management');
  });

  test('should navigate to security page', async ({ page }) => {
    await page.goto('/');
    await page.click('text=Security');
    await expect(page).toHaveURL(/.*security/);
    await expect(page.locator('h1')).toContainText('Security');
  });

  test('should navigate to virtual OS page', async ({ page }) => {
    await page.goto('/');
    await page.click('text=Virtual OS');
    await expect(page).toHaveURL(/.*virtual-os/);
    await expect(page.locator('h1')).toContainText('Virtual OS');
  });

  test('should test AI agent execution', async ({ page }) => {
    await page.goto('/agents');

    // Wait for agents to load
    await page.waitForSelector('[data-testid="agent-card"]', { timeout: 5000 });

    // Click on first agent
    await page.click('[data-testid="agent-card"]:first-child');

    // Check if execution dialog opens
    await expect(page.locator('[role="dialog"]')).toBeVisible();

    // Test execution button
    const executeButton = page.locator('button:has-text("Execute")');
    await expect(executeButton).toBeEnabled();
  });

  test('should test code execution functionality', async ({ page }) => {
    await page.goto('/code');

    // Wait for code editor to load
    await page.waitForSelector('[data-testid="code-editor"]', { timeout: 5000 });

    // Type some code
    await page.fill('[data-testid="code-input"]', 'console.log("Hello from Playwright!");');

    // Click execute button
    await page.click('[data-testid="execute-button"]');

    // Check output
    await expect(page.locator('[data-testid="output"]')).toContainText('Hello from Playwright!');
  });

  test('should test MCP server management', async ({ page }) => {
    await page.goto('/mcp');

    // Wait for MCP servers to load
    await page.waitForSelector('[data-testid="mcp-server"]', { timeout: 5000 });

    // Check server status
    const serverStatus = page.locator('[data-testid="server-status"]');
    await expect(serverStatus).toBeVisible();
  });

  test('should test quality control dashboard', async ({ page }) => {
    await page.goto('/qc');

    // Wait for QC dashboard to load
    await page.waitForSelector('[data-testid="qc-dashboard"]', { timeout: 5000 });

    // Check statistical data
    await expect(page.locator('[data-testid="anova-results"]')).toBeVisible();
    await expect(page.locator('[data-testid="control-charts"]')).toBeVisible();
  });

  test('should test task management with Kanban', async ({ page }) => {
    await page.goto('/tasks');

    // Wait for task board to load
    await page.waitForSelector('[data-testid="kanban-board"]', { timeout: 5000 });

    // Check columns exist
    await expect(page.locator('[data-testid="column-todo"]')).toBeVisible();
    await expect(page.locator('[data-testid="column-in-progress"]')).toBeVisible();
    await expect(page.locator('[data-testid="column-done"]')).toBeVisible();
  });

  test('should test security dashboard', async ({ page }) => {
    await page.goto('/security');

    // Wait for security dashboard to load
    await page.waitForSelector('[data-testid="security-dashboard"]', { timeout: 5000 });

    // Check security metrics
    await expect(page.locator('[data-testid="malware-scan-status"]')).toBeVisible();
    await expect(page.locator('[data-testid="threat-level"]')).toBeVisible();
  });

  test('should test virtual OS environment', async ({ page }) => {
    await page.goto('/virtual-os');

    // Wait for virtual OS to load
    await page.waitForSelector('[data-testid="virtual-os-container"]', { timeout: 5000 });

    // Check virtual components
    await expect(page.locator('[data-testid="code-editor"]')).toBeVisible();
    await expect(page.locator('[data-testid="terminal"]')).toBeVisible();
    await expect(page.locator('[data-testid="browser"]')).toBeVisible();
  });

  test('should handle responsive design', async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 }); // Tablet size
    await page.goto('/');

    // Check mobile menu
    const menuButton = page.locator('[data-testid="mobile-menu"]');
    await expect(menuButton).toBeVisible();

    await page.setViewportSize({ width: 375, height: 667 }); // Mobile size
    await expect(menuButton).toBeVisible();
  });

  test('should test error handling', async ({ page }) => {
    await page.goto('/nonexistent');

    // Should show 404 or redirect to main page
    await expect(page.locator('text=Not Found')).toBeVisible();
  });
});
