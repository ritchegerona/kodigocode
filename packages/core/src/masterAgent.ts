import { PluginManager } from '../../plugins/src/manager';
import { FsTool } from '../../tools/src/fsTool';
import { ToolRegistry } from '../../tools/src/toolRegistry';
import { CommandRegistry } from './commandRegistry';
import { Logger } from './logger';
import type { PluginContext } from '../../plugins/src/types';
import { CodingAgent } from '../../agents/src/codingAgent';
import { TestingAgent } from '../../agents/src/testingAgent';
import { DocAgent } from '../../agents/src/docAgent';
import { SecurityAgent } from '../../agents/src/securityAgent';
import { ChatAgent } from '../../agents/src/chatAgent';
import { GitAgent } from '../../agents/src/gitAgent';
import { WorkflowEngine, WorkflowTemplates, type WorkflowDefinition } from '../../planner/src/workflowEngine';

export class MasterAgent {
  private readonly logger = new Logger('[Master]');
  private readonly registry = new CommandRegistry();
  private readonly pluginManager: PluginManager;
  private readonly toolRegistry = new ToolRegistry();

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
    // Symbol search command
    this.registry.register('symbol-search', async (query: string) => {
      return this.toolRegistry.call('symbol-search', { query });
    });
    // Agent commands
    const coding = new CodingAgent();
    this.registry.register('code', async (prompt: string = '') => coding.execute(prompt));

    const testing = new TestingAgent();
    this.registry.register('test', async () => testing.execute());

    const doc = new DocAgent();
    this.registry.register('doc', async () => doc.execute());

    const security = new SecurityAgent();
    this.registry.register('security', async () => security.execute());

    const git = new GitAgent();
    this.registry.register('git', async (action: string = 'status') => git.execute(action));

    const chat = new ChatAgent();
    this.registry.register('chat', async (msg: string = '') => chat.execute(msg));

    // Function calling command – validates against tool schemas and dispatches to the tool
    this.registry.register('function-call', async (fnName: string, args: any = {}) => {
      // First, try to dispatch to a registered tool via ToolRegistry
      if (this.toolRegistry.get(fnName)) {
        return this.toolRegistry.call(fnName, typeof args === 'object' ? args : {});
      }
      // Fallback: forward to the configured LLM provider's functionCalling method
      let providerName = 'openai';
      try {
        const cfg = await loadConfig('kodigo.toml');
        if (cfg?.model?.provider) providerName = cfg.model.provider;
      } catch {}
      const provider = createProvider(providerName);
      if (typeof provider.functionCalling !== 'function') {
        throw new Error(`Provider ${providerName} does not support function calling`);
      }
      return provider.functionCalling(fnName, args);
    });

    // Plugins command – list loaded plugins
    this.registry.register('plugins', async () => {
      const names = (this.pluginManager as any).listPlugins?.() ?? [];
      return `Loaded plugins: ${names.join(', ') || 'none'}`;
    });

    // Plugin install command – copies a plugin folder into the workspace's plugins directory.
    this.registry.register('plugins-install', async (srcPath: string) => {
      const fs = await import('fs/promises');
      const path = await import('path');
      const dest = path.resolve(process.cwd(), 'plugins', path.basename(srcPath));
      await fs.cp(srcPath, dest, { recursive: true, force: true });
      return `Plugin installed to ${dest}`;
    });

    // Plugin remove command – deletes a plugin folder.
    this.registry.register('plugins-remove', async (pluginName: string) => {
      const fs = await import('fs/promises');
      const path = await import('path');
      const target = path.resolve(process.cwd(), 'plugins', pluginName);
      await fs.rm(target, { recursive: true, force: true });
      return `Plugin ${pluginName} removed`;
    });

    // Generic tool call – validates against schema and dispatches
    this.registry.register('call', async (toolName: string, ...args: any[]) => {
      // If args is a single object, treat it as named args; otherwise use positional
      if (args.length === 1 && typeof args[0] === 'object' && !Array.isArray(args[0])) {
        return this.toolRegistry.call(toolName, args[0]);
      }
      // Positional args: map to schema required fields
      const schema = this.toolRegistry.getSchema(toolName);
      if (!schema) throw new Error(`Tool ${toolName} not found`);
      const namedArgs: Record<string, any> = {};
      schema.parameters.required.forEach((key, i) => {
        if (i < args.length) namedArgs[key] = args[i];
      });
      return this.toolRegistry.call(toolName, namedArgs);
    });

    // Multi‑file edit command – uses CodingAgent to get a diff and applies it via FsTool
    this.registry.register('edit', async (prompt: string = '') => {
      const coding = new CodingAgent();
      const diff = await coding.generateDiff(prompt);
      if (!diff?.changes?.length) return 'No changes returned.';
      const fsTool = new FsTool();
      for (const change of diff.changes) {
        const { path, action, content } = change;
        if (action === 'add' || action === 'edit') {
          await fsTool.run(['write', path, content ?? '']);
        } else if (action === 'delete') {
          await fsTool.run(['delete', path]);
        }
      }
      return `Applied ${diff.changes.length} change(s).`;
    });

    // Tools command – exposes all tool schemas (for LLM function-calling APIs)
    this.registry.register('tools', async () => {
      return this.toolRegistry.getAllSchemas();
    });

    // Workflow command – executes a multi-agent workflow
    const workflowEngine = new WorkflowEngine(async (agentName: string, prompt: string) => {
      return this.registry.execute(agentName, prompt);
    });

    this.registry.register('workflow', async (workflowOrName: string | WorkflowDefinition, ...extra: any[]) => {
      let definition: WorkflowDefinition;
      if (typeof workflowOrName === 'string') {
        // Look up predefined template
        const template = (WorkflowTemplates as any)[workflowOrName];
        if (typeof template === 'function') {
          definition = template(...extra);
        } else {
          throw new Error(`Unknown workflow template: ${workflowOrName}. Available: ${Object.keys(WorkflowTemplates).join(', ')}`);
        }
      } else {
        definition = workflowOrName;
      }
      return workflowEngine.execute(definition);
    });

    // Workflow list command – shows available workflow templates
    this.registry.register('workflow-list', async () => {
      return `Available workflows: ${Object.keys(WorkflowTemplates).join(', ')}`;
    });

    this.logger.info('MasterAgent initialized. Commands: ' + this.registry.list().join(', '));
  }

  async runCommand(name: string, ...args: any[]) {
    return this.registry.execute(name, ...args);
  }
}
