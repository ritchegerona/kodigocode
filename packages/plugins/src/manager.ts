import { KodigoPlugin, PluginContext } from './types';
import { readdir } from 'fs/promises';
import { join } from 'path';

export class PluginManager {
  private plugins: KodigoPlugin[] = [];
  private commands: Map<string, (..args: any[]) => Promise<any>> = new Map();

  constructor(private readonly pluginsDir: string = join(process.cwd(), 'plugins')) {}

  async loadAll(context: PluginContext) {
    const entries = await readdir(this.pluginsDir, { withFileTypes: true });
    for (const entry of entries) {
      if (entry.isDirectory()) {
        const pluginPath = join(this.pluginsDir, entry.name);
        const mod = await import(pluginPath + '/index.js');
        const plugin: KodigoPlugin = mod.plugin || mod.default;
        if (plugin) {
          await plugin.activate(context);
          this.plugins.push(plugin);
        }
      }
    }
  }

  registerCommand(name: string, handler: (...args: any[]) => Promise<any>) {
    this.commands.set(name, handler);
  }

  async runCommand(name: string, ...args: any[]) {
    const cmd = this.commands.get(name);
    if (!cmd) throw new Error(`Command ${name} not found`);
    return cmd(...args);
  }
}
