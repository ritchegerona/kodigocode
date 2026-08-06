export interface PluginContext {
  registerCommand(name: string, handler: (...args: any[]) => Promise<any>): void;
  logger: any;
}

export interface KodigoPlugin {
  name: string;
  version: string;
  activate(context: PluginContext): Promise<void>;
  deactivate(): Promise<void>;
}
