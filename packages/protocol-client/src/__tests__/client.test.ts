import { OrchestratorClient } from '../client';

describe('OrchestratorClient MCP RPCs', () => {
  it('dispatches mcpServers.list with defaults when not provided', async () => {
    const client = new OrchestratorClient({ reconnect: false });
    const requestSpy = jest
      .spyOn(client as unknown as { request: (method: string, params: unknown) => Promise<unknown> }, 'request')
      .mockResolvedValue({ data: [] });

    await client.mcpServersList();

    expect(requestSpy).toHaveBeenCalledWith('mcpServers.list', {});
  });

  it('dispatches mcpServer.oauth.login with provided payload', async () => {
    const client = new OrchestratorClient({ reconnect: false });
    const requestSpy = jest
      .spyOn(client as unknown as { request: (method: string, params: unknown) => Promise<unknown> }, 'request')
      .mockResolvedValue({ authorizationUrl: 'https://example.com' });

    await client.mcpServerOauthLogin({ name: 'github', scopes: ['repo'], timeoutSecs: 60 });

    expect(requestSpy).toHaveBeenCalledWith('mcpServer.oauth.login', {
      name: 'github',
      scopes: ['repo'],
      timeoutSecs: 60,
    });
  });
});
