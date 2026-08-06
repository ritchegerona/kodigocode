import { PluginManager } from '../../plugins/src/manager';
import { CommandRegistry } from './commandRegistry';
import { Logger } from './logger';
import type { PluginContext } from '../../plugins/src/types';

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
    this.logger.info('MasterAgent initialized. Commands: ' + this.registry.list().join(', '));
  }

  async runCommand(name: string, ...args: any[]) {
    return this.registry.execute(name, ...args);
  }
}
