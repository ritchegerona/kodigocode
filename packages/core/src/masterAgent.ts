import { PluginManager } from '../../plugins/src/manager';
import { SymbolSearchTool } from '../../tools/src/symbolSearchTool';
import { CommandRegistry } from './commandRegistry';
import { Logger } from './logger';
import type { PluginContext } from '../../plugins/src/types';
import { CodingAgent } from '../../agents/src/codingAgent';
import { TestingAgent } from '../../agents/src/testingAgent';
import { DocAgent } from '../../agents/src/docAgent';
import { SecurityAgent } from '../../agents/src/securityAgent';
import { GitAgent } from '../../agents/src/gitAgent';

export class MasterAgent {
  private readonly logger = new Logger('[Master]');
  private readonly registry = new CommandRegistry();
  private readonly pluginManager: PluginManager;

  constructor() {
    this.pluginManager = new PluginManager();
  }

  async init() {
    const ctx: PluginContext = {
      registerCommand: (name, handler) => this.registry.register(name, handler),
      logger: this.logger,
    } as any;
    await this.pluginManager.loadAll(ctx);
    // Built‑in commands
    this.registry.register('help', async () => {
      return `Available commands: ${this.registry.list().join(', ')}`;
    });
    // Symbol search command for repository intelligence (Phase 2)
    this.registry.register('symbol-search', async (query: string) => {
      const tool = new SymbolSearchTool();
      return tool.run([query]);
    });
      // Register agent commands
      const coding = new CodingAgent();
      this.registry.register('code', async () => coding.execute());

      const testing = new TestingAgent();
      this.registry.register('test', async () => testing.execute());

      const doc = new DocAgent();
      this.registry.register('doc', async () => doc.execute());

      const security = new SecurityAgent();
      this.registry.register('security', async () => security.execute());

      const git = new GitAgent();
      this.registry.register('git', async (action: string = 'status') => git.execute(action));

      this.logger.info('MasterAgent initialized. Commands: ' + this.registry.list().join(', '));
  }

  async runCommand(name: string, ...args: any[]) {
    return this.registry.execute(name, ...args);
  }
}
