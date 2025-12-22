import { SlashCommandRouter } from '../slash-router';
import type { OrchestratorClient } from '../client';
import type { SkillsClientFacade } from '../skills';
import type { ListMcpServersResponse, McpServerOauthLoginResponse, SkillsInvokeResponse } from '../types';

describe('SlashCommandRouter', () => {
  const skillResponse: SkillsInvokeResponse = {
    status: 'ok',
    result: { echoed: true },
    version: 'v1',
  };

  const listResponse: ListMcpServersResponse = { data: [], nextCursor: undefined };
  const oauthResponse: McpServerOauthLoginResponse = { authorizationUrl: 'https://example.com/oauth' };

  const client = {
    mcpServersList: jest.fn(async () => listResponse),
    mcpServerOauthLogin: jest.fn(async () => oauthResponse),
  } as unknown as OrchestratorClient;

  const skills = {
    invokeSlashCommand: jest.fn(async () => skillResponse),
  } as unknown as SkillsClientFacade;

  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('routes non-MCP commands through the skills facade with audit metadata', async () => {
    const router = new SlashCommandRouter(client, skills);

    const result = await router.dispatch('/qc/check', { severity: 'high' }, {
      origin: 'cli',
      sessionId: 'sess-1',
      requestId: 'req-99',
      tags: ['qc', 'slash'],
    });

    expect(result.kind).toBe('skill');
    expect(skills.invokeSlashCommand).toHaveBeenCalledWith('qc/check', { severity: 'high' }, {
      origin: 'cli',
      sessionId: 'sess-1',
      requestId: 'req-99',
      tags: ['qc', 'slash'],
    });
  });

  it('routes mcp/list to orchestrator MCP listing', async () => {
    const router = new SlashCommandRouter(client, skills, 'tui');

    const result = await router.dispatch('mcp/list', { cursor: 'next', limit: 10 });

    expect(result.kind).toBe('mcp:list');
    expect(client.mcpServersList).toHaveBeenCalledWith({ cursor: 'next', limit: 10 });
    expect(skills.invokeSlashCommand).not.toHaveBeenCalled();
  });

  it('routes mcp/login to orchestrator OAuth helper with defaults', async () => {
    const router = new SlashCommandRouter(client, skills, 'tui');

    const result = await router.dispatch('/mcp/login', { server: 'github', scopes: ['read:user'], timeoutSecs: 30 });

    expect(result.kind).toBe('mcp:oauth');
    expect(client.mcpServerOauthLogin).toHaveBeenCalledWith({
      name: 'github',
      scopes: ['read:user'],
      timeoutSecs: 30,
    });
    expect(skills.invokeSlashCommand).not.toHaveBeenCalled();
  });

  it('throws when mcp/login is missing a server name', async () => {
    const router = new SlashCommandRouter(client, skills);

    await expect(router.dispatch('mcp/login')).rejects.toThrow(
      'mcp/login requires a server name provided as "name" or "server"',
    );
  });
});
