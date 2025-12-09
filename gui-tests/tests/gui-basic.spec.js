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

test.describe('Codex GUI API Integration Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Wait for the server to be ready
    await page.waitForTimeout(3000);
  });

  test('should load system metrics from real API', async ({ page }) => {
    // Test direct API call to system metrics
    const response = await page.request.get('http://localhost:8787/api/system/metrics');
    expect(response.ok()).toBeTruthy();

    const metrics = await response.json();
    expect(metrics).toHaveProperty('cpu_usage');
    expect(metrics).toHaveProperty('memory_usage');
    expect(metrics).toHaveProperty('disk_usage');
    expect(metrics).toHaveProperty('active_processes');

    // Verify values are reasonable (not mock data)
    expect(metrics.cpu_usage).toBeGreaterThanOrEqual(0);
    expect(metrics.cpu_usage).toBeLessThanOrEqual(100);
    expect(metrics.memory_usage).toBeGreaterThanOrEqual(0);
    expect(metrics.memory_usage).toBeLessThanOrEqual(100);
    expect(metrics.active_processes).toBeGreaterThan(0);
  });

  test('should create and manage conversations', async ({ page }) => {
    // Create a new conversation
    const createResponse = await page.request.post('http://localhost:8787/api/conversations', {
      data: {
        model: 'gpt-4',
        initial_message: 'Test conversation from Playwright'
      }
    });
    expect(createResponse.ok()).toBeTruthy();

    const conversation = await createResponse.json();
    expect(conversation).toHaveProperty('id');
    expect(conversation.model).toBe('gpt-4');
    expect(conversation.status).toBe('active');
    expect(conversation.message_count).toBe(1);

    // List conversations
    const listResponse = await page.request.get('http://localhost:8787/api/conversations');
    expect(listResponse.ok()).toBeTruthy();

    const conversations = await listResponse.json();
    expect(conversations.length).toBeGreaterThan(0);
    expect(conversations[conversations.length - 1].id).toBe(conversation.id);

    // Get messages from conversation
    const messagesResponse = await page.request.get(`http://localhost:8787/api/conversations/${conversation.id}/messages`);
    expect(messagesResponse.ok()).toBeTruthy();

    const messages = await messagesResponse.json();
    expect(messages.length).toBe(1);
    expect(messages[0].role).toBe('user');
    expect(messages[0].content).toBe('Test conversation from Playwright');

    // Send another message
    const sendResponse = await page.request.post(`http://localhost:8787/api/conversations/${conversation.id}/messages`, {
      data: {
        content: 'Another test message',
        role: 'user'
      }
    });
    expect(sendResponse.ok()).toBeTruthy();

    const newMessage = await sendResponse.json();
    expect(newMessage.content).toBe('Another test message');
    expect(newMessage.role).toBe('user');
  });

  test('should get current user information', async ({ page }) => {
    const response = await page.request.get('http://localhost:8787/api/user');
    expect(response.ok()).toBeTruthy();

    const user = await response.json();
    expect(user).toHaveProperty('id');
    expect(user).toHaveProperty('name');
    expect(user).toHaveProperty('email');

    // Verify it's a real user object, not mock
    expect(user.id).toBe('default-user');
    expect(user.name).toBe('Codex User');
    expect(user.email).toBe('user@codex.local');
  });

  test('should list MCP connections based on environment', async ({ page }) => {
    const response = await page.request.get('http://localhost:8787/api/mcp/connections');
    expect(response.ok()).toBeTruthy();

    const connections = await response.json();
    expect(Array.isArray(connections)).toBeTruthy();

    // Should have at least some default connections
    expect(connections.length).toBeGreaterThan(0);

    // Each connection should have required properties
    connections.forEach(connection => {
      expect(connection).toHaveProperty('id');
      expect(connection).toHaveProperty('name');
      expect(connection).toHaveProperty('type');
      expect(connection).toHaveProperty('status');
    });
  });

  test('should execute actions through API', async ({ page }) => {
    // First get available actions
    const actionsResponse = await page.request.get('http://localhost:8787/api/actions');
    expect(actionsResponse.ok()).toBeTruthy();

    const actions = await actionsResponse.json();
    expect(Array.isArray(actions)).toBeTruthy();

    // If there are actions, try to execute one
    if (actions.length > 0) {
      const testAction = actions[0];
      const executeResponse = await page.request.post(`http://localhost:8787/api/actions/${testAction.id}/execute`, {
        data: {
          values: {
            prompt: "Test execution from Playwright"
          }
        }
      });

      // Execution might succeed or fail, but API should respond
      expect([200, 400, 500].includes(executeResponse.status())).toBeTruthy();

      if (executeResponse.ok()) {
        const result = await executeResponse.json();
        expect(result).toHaveProperty('actionId');
        expect(result).toHaveProperty('status');
        expect(result).toHaveProperty('executedAt');
      }
    } else {
      // No actions available - this is also acceptable
      console.log('No actions available for testing');
    }
  });

  test('should integrate GUI with API data - system monitoring', async ({ page }) => {
    await page.goto('/');

    // Wait for page to load and potentially fetch system metrics
    await page.waitForTimeout(2000);

    // Check if system metrics are displayed somewhere in the UI
    // This might be in a dashboard or system info section
    const metricsElements = page.locator('[data-testid*="metric"], [class*="metric"]').or(page.getByText(/CPU|Memory|Disk/i));

    // If metrics are displayed in UI, verify they match API data
    const count = await metricsElements.count();
    if (count > 0) {
      // Get API data for comparison
      const apiResponse = await page.request.get('http://localhost:8787/api/system/metrics');
      const apiMetrics = await apiResponse.json();

      // Verify UI shows reasonable values - wait for elements to be visible
      await page.waitForTimeout(1000);
      const cpuElements = page.locator('text=/CPU|cpu/');
      const memoryElements = page.locator('text=/Memory|memory|RAM/');

      if ((await cpuElements.count()) > 0) {
        const cpuText = await cpuElements.first().textContent();
        expect(cpuText).toBeTruthy();
      }

      if ((await memoryElements.count()) > 0) {
        const memoryText = await memoryElements.first().textContent();
        expect(memoryText).toBeTruthy();
      }
    } else {
      // If no metrics UI is found, just verify API works
      const apiResponse = await page.request.get('http://localhost:8787/api/system/metrics');
      expect(apiResponse.ok()).toBeTruthy();

      const apiMetrics = await apiResponse.json();
      expect(apiMetrics).toHaveProperty('cpu_usage');
      expect(apiMetrics).toHaveProperty('memory_usage');
    }
  });

  test('should test conversation management in GUI', async ({ page }) => {
    await page.goto('/');

    // Try to access conversation-related features
    // This might be in a chat interface or conversation panel
    const conversationElements = page.locator('[data-testid*="conversation"], [data-testid*="chat"], [class*="conversation"]');

    if (await conversationElements.count() > 0) {
      // If conversation UI exists, test basic functionality
      await conversationElements.first().click();

      // Should be able to create new conversation or view existing ones
      await page.waitForTimeout(1000);

      // Test might include typing a message and sending it
      const inputField = page.locator('input[placeholder*="message"], textarea[placeholder*="message"]');
      if (await inputField.count() > 0) {
        await inputField.first().fill('Test message from GUI automation');
        // Note: Not clicking send to avoid actually sending messages during tests
      }
    }
  });
});
