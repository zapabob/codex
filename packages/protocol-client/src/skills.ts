import { appendFile, mkdir } from 'node:fs/promises';
import path from 'node:path';

import { OrchestratorClient } from './client';
import type {
  SkillAuditMetadata,
  SkillsInvokeRequest,
  SkillsInvokeResponse,
  SkillsListRequest,
  SkillsListResponse,
} from './types';

export interface SkillsClientFacadeOptions {
  apiVersion?: string;
  auditLogPath?: string;
  defaultAudit?: SkillAuditMetadata;
}

export class SkillsClientFacade {
  private readonly apiVersion: string;
  private readonly auditLogPath: string;
  private readonly defaultAudit?: SkillAuditMetadata;

  constructor(private readonly client: OrchestratorClient, options: SkillsClientFacadeOptions = {}) {
    this.apiVersion = options.apiVersion ?? process.env.CODEX_SKILLS_API_VERSION ?? 'v1';
    this.auditLogPath = options.auditLogPath ?? process.env.CODEX_SKILLS_AUDIT_LOG ?? path.join(process.cwd(), '.codex-skills-audit.jsonl');
    this.defaultAudit = options.defaultAudit;
  }

  async listSkills(request: SkillsListRequest = {}): Promise<SkillsListResponse> {
    return this.client.skillsList({
      ...request,
      version: request.version ?? this.apiVersion,
    });
  }

  async invokeSkill(
    name: string,
    input?: Record<string, unknown>,
    audit?: SkillAuditMetadata,
  ): Promise<SkillsInvokeResponse> {
    const mergedAudit = this.mergeAudit(audit);
    const response = await this.client.skillsInvoke(this.buildInvokeRequest(name, input, mergedAudit));

    await this.recordAudit(name, response, mergedAudit);
    return response;
  }

  async invokeSlashCommand(
    command: string,
    input?: Record<string, unknown>,
    audit?: SkillAuditMetadata,
  ): Promise<SkillsInvokeResponse> {
    const normalized = command.startsWith('/') ? command.slice(1) : command;
    const skillName = normalized.startsWith('slash/') ? normalized : `slash/${normalized}`;
    const payload = input ? { command: normalized, ...input } : { command: normalized };

    return this.invokeSkill(skillName, payload, audit);
  }

  private buildInvokeRequest(
    name: string,
    input: Record<string, unknown> | undefined,
    audit: SkillAuditMetadata | undefined,
  ): SkillsInvokeRequest {
    return {
      name,
      input,
      version: this.apiVersion,
      audit,
    };
  }

  private mergeAudit(audit?: SkillAuditMetadata): SkillAuditMetadata | undefined {
    if (!audit && !this.defaultAudit) {
      return undefined;
    }

    return {
      ...this.defaultAudit,
      ...audit,
    };
  }

  private async recordAudit(
    name: string,
    response: SkillsInvokeResponse,
    audit: SkillAuditMetadata | undefined,
  ): Promise<void> {
    const entry = {
      timestamp: new Date().toISOString(),
      skill: name,
      version: response.version ?? this.apiVersion,
      status: response.status,
      traceId: response.trace_id,
      durationMs: response.duration_ms,
      audit,
    };

    try {
      await mkdir(path.dirname(this.auditLogPath), { recursive: true });
      await appendFile(this.auditLogPath, `${JSON.stringify(entry)}\n`, 'utf8');
    } catch (error) {
      // Audit logging should not block skill execution; surface in logs only.
      console.warn('Failed to record skills audit entry', error);
    }
  }
}
