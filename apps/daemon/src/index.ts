import { watch } from 'chokidar';
import { Logger } from '@kodigocode/core/src/logger';

const logger = new Logger('[Daemon]');

const watcher = watch('.', { ignored: /node_modules|\.git/, persistent: true });

watcher.on('all', (event, path) => {
  logger.info(`${event} – ${path}`);
});

logger.info('Daemon started, watching for file changes...');
