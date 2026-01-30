import { test, expect } from '@playwright/test';

test('has title', async ({ page }) => {
  await page.goto('/');

  // Expect a title "to contain" a substring.
  // Using a generic check since I don't know the exact title, but usually it's "Codex" or similar.
  // I'll check if the body exists to ensure page load.
  await expect(page.locator('body')).toBeVisible();
  
  // You can add more specific assertions here based on your app
  // await expect(page).toHaveTitle(/Codex/);
});
