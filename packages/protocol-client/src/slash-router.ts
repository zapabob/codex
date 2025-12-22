import { OrchestratorClient } from './client';
import { SkillsClientFacade } from './skills';
import type {
  ListMcpServersRequest,
  ListMcpServersResponse,
  McpServerOauthLoginRequest,
  McpServerOauthLoginResponse,
  SkillAuditMetadata,
  SkillsInvokeResponse,
} from './types';

export interface SlashCommandContext {
  origin?: 'cli' | 'tui' | 'gui' | 'automation';
  sessionId?: string;
  requestId?: string;
  tags?: string[];
}

export type SlashCommandResult =
  | { kind: 'skill'; response: SkillsInvokeResponse }
  | { kind: 'mcp:list'; response: ListMcpServersResponse }
  | { kind: 'mcp:oauth'; response: McpServerOauthLoginResponse };

export class SlashCommandRouter {
  constructor(
    private readonly client: OrchestratorClient,
    private readonly skills: SkillsClientFacade,
    private readonly defaultOrigin: SlashCommandContext['origin'] = 'cli',
  ) {}

  async dispatch(
    command: string,
    input: Record<string, unknown> = {},
    context?: SlashCommandContext,
  ): Promise<SlashCommandResult> {
    const normalized = command.startsWith('/') ? command.slice(1) : command;
    const audit = this.buildAudit(context);

    if (normalized === 'mcp/list') {
      const response = await this.client.mcpServersList(this.buildMcpListRequest(input));
      return { kind: 'mcp:list', response };
    }

    if (normalized === 'mcp/login') {
      const response = await this.client.mcpServerOauthLogin(this.buildMcpLoginRequest(input));
      return { kind: 'mcp:oauth', response };
    }

    const response = await this.skills.invokeSlashCommand(normalized, input, audit);
    return { kind: 'skill', response };
  }

  private buildAudit(context?: SlashCommandContext): SkillAuditMetadata | undefined {
    if (!context) {
      return { origin: this.defaultOrigin };
    }

    return {
      origin: context.origin ?? this.defaultOrigin,
      sessionId: context.sessionId,
      requestId: context.requestId,
      tags: context.tags,
    };
  }

  private buildMcpListRequest(input: Record<string, unknown>): ListMcpServersRequest {
    const request: ListMcpServersRequest = {};

    if (typeof input.cursor === 'string') {
      request.cursor = input.cursor;
    }

    if (typeof input.limit === 'number') {
      request.limit = input.limit;
    }

    return request;
  }

  private buildMcpLoginRequest(input: Record<string, unknown>): McpServerOauthLoginRequest {
    const name = typeof input.name === 'string' ? input.name : typeof input.server === 'string' ? input.server : undefined;

    if (!name) {
      throw new Error('mcp/login requires a server name provided as "name" or "server"');
    }

    const request: McpServerOauthLoginRequest = { name };

    if (Array.isArray(input.scopes) && input.scopes.every((value) => typeof value === 'string')) {
      request.scopes = input.scopes as string[];
    }

    if (typeof input.timeoutSecs === 'number') {
      request.timeoutSecs = input.timeoutSecs;
    }

    return request;
  }
}
