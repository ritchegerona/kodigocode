import { KodigoPlugin, PluginContext } from '@kodigocode/plugins';

export const plugin: KodigoPlugin = {
  name: 'hello',
  version: '0.1.0',
  async activate(context: PluginContext) {
    context.registerCommand('hello', async () => {
      context.logger.info('Hello command executed');
      return '👋 Hello from KodigoCode plugin!';
    });
  },
  async deactivate() {
    // nothing to clean up
  },
};
