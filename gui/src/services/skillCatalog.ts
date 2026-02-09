import type { Skill, SkillCategory, SkillDependency, MCPServerRequirement } from '../types/mcp';

interface SkillCatalogConfig {
  maxSkills: number;
  skillsDirectory: string;
  cacheDirectory: string;
  autoInstallDependencies: boolean;
}

const DEFAULT_CONFIG: SkillCatalogConfig = {
  maxSkills: 100,
  skillsDirectory: '.codex/skills',
  cacheDirectory: '.codex/skill-cache',
  autoInstallDependencies: true,
};

interface SkillInstallProgress {
  skillId: string;
  stage: 'downloading' | 'installing' | 'configuring' | 'verifying' | 'complete' | 'error';
  progress: number;
  message: string;
}

interface SkillValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
  requirements: MCPServerRequirement[];
  missingDependencies: SkillDependency[];
}

export class SkillCatalogService {
  private config: SkillCatalogConfig;
  private skills: Map<string, Skill> = new Map();
  private categories: Map<string, SkillCategory> = new Map();
  private installedSkills: Set<string> = new Set();
  private listeners: Set<(skills: Skill[]) => void> = new Set();
  private progressListeners: Set<(progress: SkillInstallProgress) => void> = new Set();

  constructor(config?: Partial<SkillCatalogConfig>) {
    this.config = { ...DEFAULT_CONFIG, ...config };
  }

  async initialize(): Promise<void> {
    await this.loadCatalog();
    await this.scanInstalledSkills();
  }

  private async loadCatalog(): Promise<void> {
    try {
      const catalogPath = `${this.config.skillsDirectory}/catalog.json`;
      const response = await fetch(`/${catalogPath}`);
      if (response.ok) {
        const catalog = await response.json();
        catalog.skills?.forEach((skill: Skill) => {
          this.skills.set(skill.id, skill);
        });
        catalog.categories?.forEach((category: SkillCategory) => {
          this.categories.set(category.id, category);
        });
      }
    } catch (error) {
      console.warn('[SkillCatalog] Could not load remote catalog:', error);
    }

    await this.loadLocalSkills();
  }

  private async loadLocalSkills(): Promise<void> {
    try {
      const skillsPath = this.config.skillsDirectory;
      const entries = await this.listDirectory(skillsPath);

      for (const entry of entries) {
        if (entry.isDirectory) {
          await this.loadSkillFromDirectory(`${skillsPath}/${entry.name}`);
        }
      }
    } catch (error) {
      console.error('[SkillCatalog] Failed to load local skills:', error);
    }
  }

  private async loadSkillFromDirectory(path: string): Promise<void> {
    try {
      const manifestPath = `${path}/skill.json`;
      const response = await fetch(`/${manifestPath}`);
      if (!response.ok) return;

      const manifest = await response.json();
      const skill: Skill = {
        id: manifest.id,
        name: manifest.name,
        description: manifest.description,
        version: manifest.version,
        author: manifest.author,
        category: manifest.category,
        tags: manifest.tags || [],
        icon: manifest.icon,
        parameters: manifest.parameters || [],
        actions: manifest.actions || [],
        dependencies: manifest.dependencies || [],
        mcpServers: manifest.mcpServers || [],
        scripts: manifest.scripts || {},
        permissions: manifest.permissions || [],
        readme: manifest.readme,
        installed: false,
        enabled: true,
      };

      this.skills.set(skill.id, skill);
    } catch (error) {
      console.error(`[SkillCatalog] Failed to load skill from ${path}:`, error);
    }
  }

  private async listDirectory(path: string): Promise<Array<{ name: string; isDirectory: boolean }>> {
    const response = await fetch(`/${path}`);
    if (!response.ok) return [];
    return response.json();
  }

  private async scanInstalledSkills(): Promise<void> {
    this.installedSkills.clear();

    for (const [skillId, skill] of this.skills) {
      const skillPath = `${this.config.skillsDirectory}/${skillId}`;
      try {
        const response = await fetch(`/${skillPath}/.installed`);
        if (response.ok) {
          this.installedSkills.add(skillId);
          skill.installed = true;
        }
      } catch {
        skill.installed = false;
      }
    }

    this.notifyListeners();
  }

  async installSkill(skillId: string): Promise<void> {
    const skill = this.skills.get(skillId);
    if (!skill) {
      throw new Error(`Skill not found: ${skillId}`);
    }

    if (this.installedSkills.size >= this.config.maxSkills) {
      throw new Error(`Maximum skill limit (${this.config.maxSkills}) reached`);
    }

    if (this.installedSkills.has(skillId)) {
      throw new Error(`Skill already installed: ${skillId}`);
    }

    this.notifyProgress({
      skillId,
      stage: 'downloading',
      progress: 0,
      message: `Downloading ${skill.name}...`,
    });

    await this.downloadSkill(skill);

    this.notifyProgress({
      skillId,
      stage: 'installing',
      progress: 30,
      message: 'Installing dependencies...',
    });

    await this.installDependencies(skill);

    this.notifyProgress({
      skillId,
      stage: 'configuring',
      progress: 60,
      message: 'Configuring skill...',
    });

    await this.configureSkill(skill);

    this.notifyProgress({
      skillId,
      stage: 'verifying',
      progress: 90,
      message: 'Verifying installation...',
    });

    await this.verifyInstallation(skill);

    this.installedSkills.add(skillId);
    skill.installed = true;
    skill.enabled = true;

    await this.markSkillInstalled(skillId);

    this.notifyProgress({
      skillId,
      stage: 'complete',
      progress: 100,
      message: `${skill.name} installed successfully`,
    });

    this.notifyListeners();
  }

  private async downloadSkill(skill: Skill): Promise<void> {
    if (skill.repository?.url) {
      const response = await fetch(skill.repository.url);
      if (!response.ok) {
        throw new Error(`Failed to download skill: ${skill.name}`);
      }
    }
  }

  private async installDependencies(skill: Skill): Promise<void> {
    for (const dep of skill.dependencies || []) {
      if (!this.isDependencyInstalled(dep)) {
        if (dep.required || this.config.autoInstallDependencies) {
          await this.installDependency(dep);
        }
      }
    }
  }

  private isDependencyInstalled(dep: SkillDependency): boolean {
    if (dep.type === 'skill') {
      return this.installedSkills.has(dep.id);
    }
    if (dep.type === 'mcp-server') {
      return this.isMCPServerInstalled(dep.id);
    }
    return false;
  }

  private isMCPServerInstalled(serverId: string): boolean {
    return false;
  }

  private async installDependency(dep: SkillDependency): Promise<void> {
    if (dep.type === 'skill' && dep.source?.url) {
      const manifest = await this.fetchSkillManifest(dep.source.url);
      if (manifest) {
        this.skills.set(manifest.id, { ...manifest, installed: false });
        await this.installSkill(manifest.id);
      }
    }
  }

  private async fetchSkillManifest(url: string): Promise<Skill | null> {
    try {
      const response = await fetch(url);
      if (response.ok) {
        return await response.json();
      }
    } catch {
      console.warn(`[SkillCatalog] Could not fetch skill from ${url}`);
    }
    return null;
  }

  private async configureSkill(skill: Skill): Promise<void> {
    for (const [key, script] of Object.entries(skill.scripts || {})) {
      if (script.onInstall) {
        await this.executeScript(script.onInstall);
      }
    }

    if (skill.mcpServers?.length) {
      await this.registerMCPServers(skill.mcpServers);
    }
  }

  private async executeScript(scriptContent: string): Promise<void> {
    console.log('[SkillCatalog] Executing setup script...');
  }

  private async registerMCPServers(servers: MCPServerRequirement[]): Promise<void> {
    for (const server of servers) {
      console.log(`[SkillCatalog] Registering MCP server: ${server.id}`);
    }
  }

  private async verifyInstallation(skill: Skill): Promise<void> {
    const validation = await this.validateSkill(skill);
    if (!validation.valid) {
      throw new Error(`Skill validation failed: ${validation.errors.join(', ')}`);
    }
  }

  async validateSkill(skill: Skill): Promise<SkillValidationResult> {
    const errors: string[] = [];
    const warnings: string[] = [];
    const requirements: MCPServerRequirement[] = [];
    const missingDependencies: SkillDependency[] = [];

    if (!skill.id || !skill.name) {
      errors.push('Skill must have id and name');
    }

    if (skill.parameters) {
      for (const param of skill.parameters) {
        if (!param.name) {
          errors.push('Parameter must have a name');
        }
        if (param.required && param.defaultValue === undefined) {
          warnings.push(`Required parameter "${param.name}" has no default`);
        }
      }
    }

    for (const dep of skill.dependencies || []) {
      if (!this.isDependencyInstalled(dep)) {
        missingDependencies.push(dep);
        if (dep.required) {
          errors.push(`Missing required dependency: ${dep.id}`);
        }
      }
    }

    return {
      valid: errors.length === 0,
      errors,
      warnings,
      requirements: skill.mcpServers || [],
      missingDependencies,
    };
  }

  async uninstallSkill(skillId: string): Promise<void> {
    const skill = this.skills.get(skillId);
    if (!skill) {
      throw new Error(`Skill not found: ${skillId}`);
    }

    if (!this.installedSkills.has(skillId)) {
      throw new Error(`Skill not installed: ${skillId}`);
    }

    await this.cleanupSkill(skill);

    this.installedSkills.delete(skillId);
    skill.installed = false;
    skill.enabled = false;

    await this.markSkillUninstalled(skillId);

    this.notifyListeners();
  }

  private async cleanupSkill(skill: Skill): Promise<void> {
    for (const [key, script] of Object.entries(skill.scripts || {})) {
      if (script.onUninstall) {
        await this.executeScript(script.onUninstall);
      }
    }
  }

  private async markSkillInstalled(skillId: string): Promise<void> {
    try {
      const path = `${this.config.skillsDirectory}/${skillId}/.installed`;
      await fetch(`/${path}`, { method: 'PUT' });
    } catch (error) {
      console.error(`[SkillCatalog] Failed to mark skill as installed: ${skillId}`, error);
    }
  }

  private async markSkillUninstalled(skillId: string): Promise<void> {
    try {
      const path = `${this.config.skillsDirectory}/${skillId}/.installed`;
      await fetch(`/${path}`, { method: 'DELETE' });
    } catch (error) {
      console.error(`[SkillCatalog] Failed to mark skill as uninstalled: ${skillId}`, error);
    }
  }

  async enableSkill(skillId: string): Promise<void> {
    const skill = this.skills.get(skillId);
    if (!skill || !skill.installed) {
      throw new Error(`Skill not installed: ${skillId}`);
    }
    skill.enabled = true;
    this.notifyListeners();
  }

  async disableSkill(skillId: string): Promise<void> {
    const skill = this.skills.get(skillId);
    if (!skill || !skill.installed) {
      throw new Error(`Skill not installed: ${skillId}`);
    }
    skill.enabled = false;
    this.notifyListeners();
  }

  async executeSkill(
    skillId: string,
    parameters?: Record<string, unknown>
  ): Promise<unknown> {
    const skill = this.skills.get(skillId);
    if (!skill || !skill.installed || !skill.enabled) {
      throw new Error(`Skill not available: ${skillId}`);
    }

    const validatedParams = this.validateParameters(skill, parameters);

    if (skill.actions?.length) {
      const defaultAction = skill.actions.find(a => a.id === 'default') || skill.actions[0];
      return this.executeSkillAction(defaultAction, validatedParams);
    }

    return null;
  }

  private validateParameters(
    skill: Skill,
    params?: Record<string, unknown>
  ): Record<string, unknown> {
    const validated: Record<string, unknown> = {};

    for (const param of skill.parameters || []) {
      if (params && param.name in params) {
        validated[param.name] = params[param.name];
      } else if (param.defaultValue !== undefined) {
        validated[param.name] = param.defaultValue;
      } else if (param.required) {
        throw new Error(`Missing required parameter: ${param.name}`);
      }
    }

    return validated;
  }

  private async executeSkillAction(
    action: { id: string; handler: string },
    parameters: Record<string, unknown>
  ): Promise<unknown> {
    console.log(`[SkillCatalog] Executing action ${action.id} for skill...`);
    return { success: true };
  }

  getSkill(skillId: string): Skill | undefined {
    return this.skills.get(skillId);
  }

  getAllSkills(): Skill[] {
    return Array.from(this.skills.values());
  }

  getInstalledSkills(): Skill[] {
    return Array.from(this.skills.values()).filter(s => s.installed);
  }

  getEnabledSkills(): Skill[] {
    return Array.from(this.skills.values()).filter(s => s.installed && s.enabled);
  }

  getSkillsByCategory(categoryId: string): Skill[] {
    return Array.from(this.skills.values()).filter(s => s.category === categoryId);
  }

  searchSkills(query: string): Skill[] {
    const lowerQuery = query.toLowerCase();
    return Array.from(this.skills.values()).filter(s =>
      s.name.toLowerCase().includes(lowerQuery) ||
      s.description.toLowerCase().includes(lowerQuery) ||
      s.tags.some(t => t.toLowerCase().includes(lowerQuery))
    );
  }

  getCategories(): SkillCategory[] {
    return Array.from(this.categories.values());
  }

  getSkillCount(): { installed: number; total: number; max: number } {
    return {
      installed: this.installedSkills.size,
      total: this.skills.size,
      max: this.config.maxSkills,
    };
  }

  private notifyProgress(progress: SkillInstallProgress): void {
    this.progressListeners.forEach(listener => listener(progress));
  }

  subscribe(listener: (skills: Skill[]) => void): () => void {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }

  subscribeToProgress(listener: (progress: SkillInstallProgress) => void): () => void {
    this.progressListeners.add(listener);
    return () => {
      this.progressListeners.delete(listener);
    };
  }

  private notifyListeners(): void {
    const skills = this.getInstalledSkills();
    this.listeners.forEach(listener => listener(skills));
  }
}

let catalogInstance: SkillCatalogService | null = null;

export function getSkillCatalog(config?: Partial<SkillCatalogConfig>): SkillCatalogService {
  if (!catalogInstance) {
    catalogInstance = new SkillCatalogService(config);
  }
  return catalogInstance;
}

export function resetSkillCatalog(): void {
  catalogInstance = null;
}

export { SkillCatalogService as default };
