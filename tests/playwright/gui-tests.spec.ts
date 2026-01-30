import { test, expect } from '@playwright/test';

test.describe('Codex GUI Tests', () => {
  test.beforeEach(async ({ page }) => {
    // GUIサーバーがlocalhost:8787で起動している前提 (baseURLを使用)
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test('should load GUI homepage', async ({ page }) => {
    await expect(page).toHaveTitle(/Codex/);
    console.log(' GUIホームページ読み込み成功');
  });

  test('should have navigation elements', async ({ page }) => {
    // ナビゲーション要素の存在確認
    const navElements = [
      'Dashboard',
      'Agents', 
      'Tasks',
      'QC',
      'Security',
      'Virtual OS',
      'AI Tools',
      'MCP',
      'Code'
    ];

    for (const element of navElements) {
      const navItem = page.locator(`text=${element}`);
      await expect(navItem).toBeVisible();
    }
    console.log(' ナビゲーション要素確認成功');
  });

  test('should navigate to agents page', async ({ page }) => {
    await page.click('text=Agents');
    await expect(page.locator('text=AI Agents')).toBeVisible();
    console.log(' Agentsページ遷移成功');
  });

  test('should navigate to tasks page', async ({ page }) => {
    await page.click('text=Tasks');
    await expect(page.locator('text=Task Management')).toBeVisible();
    console.log(' Tasksページ遷移成功');
  });

  test('should navigate to QC page', async ({ page }) => {
    await page.click('text=QC');
    await expect(page.locator('text=Quality Control')).toBeVisible();
    console.log(' QCページ遷移成功');
  });

  test('should navigate to security page', async ({ page }) => {
    await page.click('text=Security');
    await expect(page.locator('text=Security Dashboard')).toBeVisible();
    console.log(' Securityページ遷移成功');
  });

  test('should navigate to virtual OS page', async ({ page }) => {
    await page.click('text=Virtual OS');
    await expect(page.locator('text=Virtual Environment')).toBeVisible();
    console.log(' Virtual OSページ遷移成功');
  });

  test('should navigate to AI tools page', async ({ page }) => {
    await page.click('text=AI Tools');
    await expect(page.locator('text=AI Tool Orchestration')).toBeVisible();
    console.log(' AI Toolsページ遷移成功');
  });

  test('should navigate to MCP page', async ({ page }) => {
    await page.click('text=MCP');
    await expect(page.locator('text=MCP Server Management')).toBeVisible();
    console.log(' MCPページ遷移成功');
  });

  test('should navigate to code page', async ({ page }) => {
    await page.click('text=Code');
    await expect(page.locator('text=Code Execution')).toBeVisible();
    console.log(' Codeページ遷移成功');
  });

  test('should test ANOVA dashboard functionality', async ({ page }) => {
    await page.click('text=QC');
    // ANOVAダッシュボードの要素確認
    await expect(page.locator('text=Statistical Analysis')).toBeVisible();
    await expect(page.locator('text=ANOVA Results')).toBeVisible();
    console.log(' ANOVAダッシュボード機能テスト成功');
  });

  test('should test Git4D visualization', async ({ page }) => {
    await page.click('text=Code');
    // Git4D可視化の要素確認
    await expect(page.locator('text=Git 4D Visualization')).toBeVisible();
    console.log(' Git4D可視化テスト成功');
  });

  test('should test VR/AR toggle', async ({ page }) => {
    await page.click('text=Code');
    const vrToggle = page.locator('text=Enable VR Mode');
    const arToggle = page.locator('text=Enable AR Mode');
    await expect(vrToggle.or(arToggle)).toBeVisible();
    console.log(' VR/ARトグルテスト成功');
  });
});
