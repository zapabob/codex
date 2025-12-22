import { SkillsClientFacade } from '../skills';
import type { OrchestratorClient } from '../client';
import type { SkillsInvokeResponse } from '../types';

describe('SkillsClientFacade slash command bridge', () => {
  const response: SkillsInvokeResponse = {
    status: 'ok',
    result: { handled: true },
    version: 'v1',
  };

  const client = {
    skillsInvoke: jest.fn(async () => response),
  } as unknown as OrchestratorClient;

  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('normalizes leading slash and forwards command payload', async () => {
    const facade = new SkillsClientFacade(client, { apiVersion: 'v1' });

    await facade.invokeSlashCommand('/mcp', { extra: true });

    expect(client.skillsInvoke).toHaveBeenCalledWith({
      name: 'slash/mcp',
      input: { command: 'mcp', extra: true },
      version: 'v1',
      audit: undefined,
    });
  });

  it('avoids double prefix when already namespaced', async () => {
    const facade = new SkillsClientFacade(client, { apiVersion: 'v2' });

    await facade.invokeSlashCommand('slash/status');

    expect(client.skillsInvoke).toHaveBeenCalledWith({
      name: 'slash/status',
      input: { command: 'slash/status' },
      version: 'v2',
      audit: undefined,
    });
  });
});
